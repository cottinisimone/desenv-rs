use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, Error, Fields};
use syn::spanned::Spanned;

use crate::retainer::DeriveAttributeFilter;

mod retainer;

#[proc_macro_derive(Desenv, attributes(desenv))]
pub fn derive_desenv(input: TokenStream) -> TokenStream {
    let mut derive_input: DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);

    // Remove all non-desenv attributes to avoid conflicting with other derive proc macro attributes.
    derive_input.retain_attrs();

    // Create a new span to get expansion information
    let derive_input_span: Span = derive_input.span();

    let derivation_result: Result<proc_macro2::TokenStream, Error> = match derive_input.data {
        // Only non-tuple structs are allowed
        syn::Data::Struct(ref data_struct) => match data_struct.fields {
            // Only structs with named fields are allowed
            Fields::Named(ref _fields) => {
                let struct_name: &Ident = &derive_input.ident;
                Ok(quote! {
                    impl Desenv for #struct_name {
                        fn load() -> Result<Self, ::desenv::Error>
                        where
                            Self: Sized,
                        {
                            todo!()
                        }
                    }
                })
            }
            _ => Err(Error::new(
                derive_input_span,
                "desenv could be derived only on structs with named fields",
            )),
        },
        _ => Err(Error::new(
            derive_input_span,
            "desenv could be derived only on non-tuple struct types",
        )),
    };

    match derivation_result {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
