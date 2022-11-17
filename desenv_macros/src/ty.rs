#[derive(Eq, PartialEq)]
pub enum Type {
    Option,
    Other,
}

impl Type {
    pub fn from_field(field: &syn::Field) -> Self {
        let ty: &syn::Type = &field.ty;

        match quote::quote!(#ty).to_string().as_str() {
            s if s.starts_with("Option <") => Self::Option,
            _ => Self::Other,
        }
    }
}
