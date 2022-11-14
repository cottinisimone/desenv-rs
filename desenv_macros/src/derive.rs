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
        expanded_fields.push(expand_field(field, &field_attr, struct_attr));
    }

    Ok(expanded_fields)
}

fn expand_field(field: &syn::Field, _field_attr: &attr::Field, _struct_attr: &attr::Struct) -> TokenStream {
    let field_ident: &Option<Ident> = &field.ident;
    quote!(#field_ident: "".to_string())
}
