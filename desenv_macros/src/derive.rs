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
            fn _load() -> Result<Self, ::desenv::Error>
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

    if field_attr.nested {
        let field_type: &syn::Type = &field.ty;
        Ok(quote!(#field_ident: <#field_type>::_load()?))
    } else {
        let field_identity_as_string: String = field
            .ident
            .as_ref()
            .map(ToString::to_string)
            .ok_or_else(|| Error::new(field.span(), "failed to stringify identity"))?;
        let prefix: String = struct_attr.get_prefix();

        let var_name: TokenStream = var_name(field_identity_as_string.as_str(), prefix.as_str(), field_attr);
        let token_stream: TokenStream = quote_field(&var_name, field_attr);
        Ok(quote!(#field_ident: #token_stream))
    }
}

fn quote_field(var_name: &TokenStream, _field_attr: &attr::Field) -> TokenStream {
    quote!(std::env::var(#var_name.as_str()).unwrap().parse().unwrap())
}

// Returns the environment variable name that should be fetched. If could be the field name upcased
// or the rename value (both prefixed).
fn var_name(field_name: &str, prefix: &str, field_attr: &attr::Field) -> TokenStream {
    let var_name: String = if let attr::Field { rename: Some(rename), .. } = field_attr {
        rename.to_string()
    } else {
        field_name.to_uppercase()
    };

    quote!(format!("{}{}", #prefix, #var_name))
}