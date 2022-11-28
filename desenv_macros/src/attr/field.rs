use proc_macro2::{Ident, Span};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, Error, Lit, Meta, MetaList, MetaNameValue, NestedMeta, Path};

const RENAME: &str = "rename";
const RENAME_USAGE: &str = "#[desenv(rename = \"value\")]";

const SEPARATOR: &str = "separator";
const SEPARATOR_USAGE: &str = "#[desenv(separator = ',')]";

const NESTED: &str = "nested";
const NESTED_USAGE: &str = "#[desenv(nested)]";

const DEFAULT: &str = "default";
const DEFAULT_USAGE: &str =
    "#[desenv(default)], #[desenv(default = \"value\")], #[desenv(default(value = \"value\"))] or #[desenv(default(env = \"ENV\"))]";

pub struct Field {
    pub rename: Option<String>,
    pub default: Option<Default>,
    pub separator: Option<char>,
    pub nested: bool,
}

impl Field {
    pub fn from_attrs(attrs: &[Attribute], field_span: Span) -> Result<Self, Error> {
        let mut this: Self = Self { rename: None, default: None, separator: None, nested: false };

        if attrs.len() > 1 {
            return Err(Error::new(
                field_span,
                "cannot have more than one `desenv` attribute per field",
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
                    "`desenv` field attribute must be used as list of modifiers",
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

        this.validate(field_span)
    }

    fn validate(self, span: Span) -> Result<Self, Error> {
        match self {
            Self { nested: true, rename, separator, .. } if rename.is_some() || separator.is_some() => Err(Error::new(
                span,
                "cannot set `rename` or `separator` modifiers on a field marked as `nested`",
            )),
            Self { nested: true, default: Some(Default::Env(_)), .. } => Err(Error::new(
                span,
                "cannot set `default` with env modifier on a field marked as `nested`",
            )),
            Self { nested: true, default: Some(Default::Value(_)), .. } => Err(Error::new(
                span,
                "cannot set `default` with value modifier on a field marked as `nested`",
            )),
            Self { rename: Some(rename), .. } if rename.is_empty() => {
                Err(Error::new(span, "`rename` modifiers must not be empty"))
            }
            _ => Ok(self),
        }
    }
}

pub enum Default {
    Std,
    Value(String),
    Env(String),
}

fn parse_path(mut this: Field, path: &Path, span: Span) -> Result<Field, Error> {
    let ty: &str = "path";

    match path {
        _ if path.is_ident(DEFAULT) => this.default = Some(Default::Std),
        _ if path.is_ident(NESTED) => this.nested = true,
        _ if path.is_ident(RENAME) => return Err(Error::new(span, usage_error(ty, RENAME, RENAME_USAGE))),
        _ if path.is_ident(SEPARATOR) => return Err(Error::new(span, usage_error(ty, SEPARATOR, SEPARATOR_USAGE))),
        _ => return Err(unknown_modifier(path.get_ident(), span)),
    }

    Ok(this)
}

fn parse_name_value(mut this: Field, name_value: MetaNameValue, span: Span) -> Result<Field, Error> {
    let ty: &str = "named value";

    match name_value {
        MetaNameValue { path, lit: Lit::Str(lit), .. } if path.is_ident(RENAME) => this.rename = Some(lit.value()),
        MetaNameValue { path, .. } if path.is_ident(RENAME) => {
            return Err(Error::new(span, lit_error("string", RENAME, RENAME_USAGE)))
        }
        MetaNameValue { path, lit: Lit::Char(lit), .. } if path.is_ident(SEPARATOR) => {
            this.separator = Some(lit.value());
        }
        MetaNameValue { path, lit: Lit::Str(lit), .. } if path.is_ident(DEFAULT) => {
            this.default = Some(Default::Value(lit.value()));
        }
        MetaNameValue { path, .. } if path.is_ident(DEFAULT) => {
            return Err(Error::new(span, lit_error("string", DEFAULT, DEFAULT_USAGE)))
        }
        MetaNameValue { path, .. } if path.is_ident(SEPARATOR) => {
            return Err(Error::new(span, lit_error("char", SEPARATOR, SEPARATOR_USAGE)))
        }
        MetaNameValue { path, .. } if path.is_ident(NESTED) => {
            return Err(Error::new(span, usage_error(ty, NESTED, NESTED_USAGE)))
        }
        MetaNameValue { path, .. } => return Err(unknown_modifier(path.get_ident(), span)),
    }

    Ok(this)
}

fn parse_list(mut this: Field, meta_list: MetaList, span: Span) -> Result<Field, Error> {
    let ty: &str = "list";

    match meta_list {
        MetaList { path, nested, .. } if path.is_ident(DEFAULT) => {
            this.default = Some(parse_nested_meta_for_default(&path, &nested)?);
        }
        MetaList { path, .. } if path.is_ident(RENAME) => {
            return Err(Error::new(span, usage_error(ty, RENAME, RENAME_USAGE)))
        }
        MetaList { path, .. } if path.is_ident(SEPARATOR) => {
            return Err(Error::new(span, usage_error(ty, SEPARATOR, SEPARATOR_USAGE)))
        }
        MetaList { path, .. } if path.is_ident(NESTED) => {
            return Err(Error::new(span, usage_error(ty, NESTED, NESTED_USAGE)))
        }
        MetaList { path, .. } => return Err(unknown_modifier(path.get_ident(), span)),
    }

    Ok(this)
}

fn parse_nested_meta_for_default(path: &Path, nested: &Punctuated<NestedMeta, Comma>) -> Result<Default, Error> {
    if nested.len() > 1 {
        let message: String = "`default` modifier cannot contain more than one modifier".to_string();
        return Err(Error::new(path.span(), message));
    }

    match nested.first() {
        Some(NestedMeta::Meta(meta)) => match meta {
            Meta::NameValue(MetaNameValue { path, lit, .. }) if path.is_ident("env") => match lit {
                Lit::Str(str) => Ok(Default::Env(str.value())),
                _ => Err(Error::new_spanned(
                    lit,
                    "`default.env` modifier must contain a string literal",
                )),
            },
            Meta::NameValue(MetaNameValue { path, lit, .. }) if path.is_ident("value") => match lit {
                Lit::Str(str) => Ok(Default::Value(str.value())),
                _ => Err(Error::new_spanned(
                    lit,
                    "`default.value` modifier must contain a string literal",
                )),
            },
            Meta::NameValue(MetaNameValue { path, .. }) => Err(unknown_modifier(path.get_ident(), path.span())),
            Meta::Path(_) => {
                let message = "`default` modifier cannot contain a path modifier".to_string();
                Err(Error::new(path.span(), message))
            }
            Meta::List(_) => {
                let message = "`default` modifier cannot contain a list modifier".to_string();
                Err(Error::new(path.span(), message))
            }
        },
        Some(NestedMeta::Lit(_)) => {
            let message = "`default` modifier must contain exactly one non literal entry".to_string();
            Err(Error::new(path.span(), message))
        }
        None => {
            let message = "`default` modifier must contain exactly one entry".to_string();
            Err(Error::new(path.span(), message))
        }
    }
}

fn usage_error(ty: &str, modifier: &str, usage: &str) -> String {
    format!("`{}` modifier cannot be used as {}. Usage: `{}`", modifier, ty, usage)
}

fn lit_error(ty: &str, modifier: &str, usage: &str) -> String {
    format!(
        "`{}` modifier must contain a {} literal. Usage: `{}`",
        modifier, ty, usage
    )
}

fn unknown_modifier(ident: Option<&Ident>, span: Span) -> Error {
    ident.map_or_else(
        || Error::new(span, "unknown field attribute modifier".to_string()),
        |name| Error::new(span, format!("unknown field attribute modifier `{}`", name)),
    )
}
