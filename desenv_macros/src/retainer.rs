trait AttributeFilter {
    fn retain_attrs(&mut self);
}
impl AttributeFilter for Vec<syn::Attribute> {
    fn retain_attrs(&mut self) {
        self.retain(|attr| attr.path.is_ident("desenv"));
    }
}
impl AttributeFilter for syn::Field {
    fn retain_attrs(&mut self) {
        self.attrs.retain_attrs();
    }
}
impl AttributeFilter for syn::Fields {
    fn retain_attrs(&mut self) {
        self.iter_mut().for_each(AttributeFilter::retain_attrs);
    }
}
impl AttributeFilter for syn::FieldsNamed {
    fn retain_attrs(&mut self) {
        self.named.iter_mut().for_each(AttributeFilter::retain_attrs);
    }
}
impl AttributeFilter for syn::FieldsUnnamed {
    fn retain_attrs(&mut self) {
        self.unnamed.iter_mut().for_each(AttributeFilter::retain_attrs);
    }
}
impl AttributeFilter for syn::Data {
    fn retain_attrs(&mut self) {
        match self {
            Self::Enum(e) => e.variants.iter_mut().for_each(|variant| {
                variant.attrs.retain_attrs();
                variant.fields.retain_attrs();
            }),
            Self::Struct(s) => s.fields.retain_attrs(),
            Self::Union(u) => u.fields.retain_attrs(),
        }
    }
}

pub trait DeriveAttributeFilter {
    /// Removes any non-veil attributes from the derive macro input.
    fn retain_attrs(&mut self);
}
impl DeriveAttributeFilter for syn::DeriveInput {
    fn retain_attrs(&mut self) {
        self.attrs.retain_attrs();
        self.data.retain_attrs();
    }
}
