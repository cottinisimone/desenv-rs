use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::Error;

use crate::attr;

pub fn desenv(
    struct_name: &Ident,
    struct_attr: &attr::Struct,
    fields: &Punctuated<syn::Field, Comma>,
) -> Result<TokenStream, Error> {
    let expanded_fields: Vec<TokenStream> = expand_fields(struct_attr, fields)?;

    Ok(quote! {
        impl Desenv for #struct_name {
            fn _load(parent_prefix: Option<String>) -> Result<Self, ::desenv::Error>
            where
                Self: Sized,
            {
                Ok(Self {
                    #(#expanded_fields,)*
                })
            }
        }
    })
}

fn expand_fields(
    struct_attr: &attr::Struct,
    fields: &Punctuated<syn::Field, Comma>,
) -> Result<Vec<TokenStream>, Error> {
    let mut expanded_fields: Vec<TokenStream> = vec![];

    for field in fields {
        let field_attr: attr::Field = attr::Field::from_attrs(&field.attrs, field.span())?;
        expanded_fields.push(expand_field(field, &field_attr, struct_attr)?);
    }

    Ok(expanded_fields)
}

fn expand_field(
    field: &syn::Field,
    field_attr: &attr::Field,
    struct_attr: &attr::Struct,
) -> Result<TokenStream, Error> {
    let field_ident: &Option<Ident> = &field.ident;
    let current_prefix: TokenStream = current_prefix(struct_attr);

    if field_attr.nested {
        let field_type: &syn::Type = &field.ty;
        Ok(quote!(#field_ident: <#field_type>::_load(#current_prefix)?))
    } else {
        let field_identity_as_string: String = field
            .ident
            .as_ref()
            .map(ToString::to_string)
            .ok_or_else(|| Error::new(field.span(), "failed to stringify identity"))?;

        let var_name: TokenStream = var_name(field_identity_as_string.as_str(), &current_prefix, field_attr);
        let token_stream: TokenStream = quote_field(&var_name, field_attr);
        Ok(quote!(#field_ident: #token_stream))
    }
}

fn quote_field(var_name: &TokenStream, field_attr: &attr::Field) -> TokenStream {
    let parse_token: TokenStream = parse_early_return();

    match &field_attr.default {
        Some(attr::Default::Std) => quote! {
            std::env::var(#var_name.as_str()).unwrap_or_default() #parse_token
        },
        Some(attr::Default::Value(value)) => quote! {
            std::env::var(#var_name.as_str()).unwrap_or(#value.to_string()) #parse_token
        },
        Some(attr::Default::Env(env_var)) => {
            let map_err_token: TokenStream = map_err_early_return(var_name);
            quote! {
                match std::env::var(#var_name.as_str()) {
                    Ok(var) => var #parse_token,
                    Err(std::env::VarError::NotUnicode(_)) => return Err(::desenv::Error::NotUnicodeVar(#var_name)),
                    Err(std::env::VarError::NotPresent) => std::env::var(#env_var) #map_err_token #parse_token,
                }
            }
        }
        None => {
            let map_err_token: TokenStream = map_err_early_return(var_name);
            quote!(std::env::var(#var_name.as_str()) #map_err_token #parse_token)
        }
    }
}

fn map_err_early_return(var_name: &TokenStream) -> TokenStream {
    quote!(.map_err(|err| match err {
        std::env::VarError::NotPresent => ::desenv::Error::MissingVar(#var_name),
        std::env::VarError::NotUnicode(_) => ::desenv::Error::NotUnicodeVar(#var_name),
    })?)
}

fn parse_early_return() -> TokenStream {
    quote!(.parse().map_err(|error| ::desenv::Error::ParseFromStr(format!("{:?}", error)))?)
}

// Returns the environment variable name that should be fetched. If could be the field name upcased
// or the rename value (both prefixed).
fn var_name(field_name: &str, current_prefix: &TokenStream, field_attr: &attr::Field) -> TokenStream {
    let var_name: String = if let attr::Field { rename: Some(rename), .. } = field_attr {
        rename.to_string()
    } else {
        field_name.to_uppercase()
    };

    quote!(format!("{}{}", #current_prefix.unwrap_or_default(), #var_name))
}

// Concat parent prefix with current prefix returning an Option<String> in the quoted code.
fn current_prefix(struct_attr: &attr::Struct) -> TokenStream {
    let prefix: String = struct_attr.get_prefix();
    quote!(parent_prefix.clone().map(|v| format!("{}{}", v, #prefix)).or_else(|| Some(#prefix.to_string())))
}
