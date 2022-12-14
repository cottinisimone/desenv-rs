#[derive(Eq, PartialEq)]
pub enum Type {
    OsString,
    Option,
    Vector,
    Other,
}

impl Type {
    pub fn from_field(field: &syn::Field) -> Self {
        let ty: &syn::Type = &field.ty;

        match quote::quote!(#ty).to_string().as_str() {
            "OsString" => Self::OsString,
            s if s.starts_with("Option <") => Self::Option,
            s if s.starts_with("Vec < ") => Self::Vector,
            _ => Self::Other,
        }
    }
}
