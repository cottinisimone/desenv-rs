use proc_macro2::{Ident, Span};
use syn::spanned::Spanned;
use syn::{Attribute, Error, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Path};

const PREFIX: &str = "prefix";
const PREFIX_USAGE: &str = "#[desenv(prefix = \"value\")]";

pub struct Struct {
    pub prefix: Option<String>,
}

impl Struct {
    pub fn from_attrs(attrs: &[Attribute], struct_span: Span) -> Result<Self, Error> {
        let mut this: Self = Self { prefix: None };

        if attrs.len() > 1 {
            return Err(Error::new(
                attrs.first().unwrap().span(),
                "cannot have more than one `desenv` attribute per struct",
            ));
        }

        let attr: &Attribute = match attrs.first() {
            None => return Ok(this),
            Some(attr) => attr,
        };

        let modifiers = match attr.parse_meta()? {
            Meta::List(meta_list) => meta_list.nested.into_iter().filter_map(|meta| match meta {
                NestedMeta::Meta(meta) => Some(meta),
                NestedMeta::Lit(_) => None,
            }),

            meta => {
                return Err(Error::new(
                    meta.span(),
                    "`desenv` struct attribute must be used as list of modifiers",
                ))
            }
        };

        for meta in modifiers {
            let span: Span = meta.span();
            match meta {
                Meta::Path(path) => this = parse_path(this, &path, span)?,
                Meta::NameValue(name_value) => this = parse_name_value(this, name_value, span)?,
                Meta::List(meta_list) => this = parse_list(this, meta_list, span)?,
            }
        }

        this.validate(struct_span)
    }

    fn validate(self, span: Span) -> Result<Self, Error> {
        match self {
            Self { prefix: Some(prefix) } if prefix.is_empty() => {
                Err(Error::new(span, "`prefix` modifiers must not be empty"))
            }
            _ => Ok(self),
        }
    }

    pub fn get_prefix(&self) -> String {
        self.prefix.clone().unwrap_or_default()
    }
}

fn parse_path(mut _this: Struct, path: &Path, span: Span) -> Result<Struct, Error> {
    match path {
        _ if path.is_ident(PREFIX) => Err(Error::new(span, usage_error("path", PREFIX, PREFIX_USAGE))),
        _ => Err(unknown_modifier(path.get_ident(), span)),
    }
}

fn parse_name_value(mut this: Struct, name_value: MetaNameValue, span: Span) -> Result<Struct, Error> {
    match name_value {
        MetaNameValue { path, lit, .. } if path.is_ident(PREFIX) => match &lit {
            Lit::Str(str) => this.prefix = Some(str.value()),
            _ => {
                return Err(Error::new_spanned(
                    lit,
                    format!("`{}` modifier must contain a string literal", PREFIX),
                ))
            }
        },
        MetaNameValue { path, .. } => {
            return Err(unknown_modifier(path.get_ident(), span));
        }
    }

    Ok(this)
}

fn parse_list(mut _this: Struct, meta_list: MetaList, span: Span) -> Result<Struct, Error> {
    match meta_list {
        MetaList { path, .. } if path.is_ident(PREFIX) => {
            Err(Error::new(span, usage_error("list", PREFIX, PREFIX_USAGE)))
        }
        MetaList { path, .. } => Err(unknown_modifier(path.get_ident(), span)),
    }
}

fn usage_error(ty: &str, modifier: &str, usage: &str) -> String {
    format!("`{}` modifier cannot be used as {}. Usage: `{}`", modifier, ty, usage)
}

fn unknown_modifier(ident: Option<&Ident>, span: Span) -> Error {
    ident.map_or_else(
        || Error::new(span, "unknown struct attribute modifier".to_string()),
        |name| Error::new(span, format!("unknown struct attribute modifier `{}`", name)),
    )
}
