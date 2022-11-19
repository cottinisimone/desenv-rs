use desenv::Desenv;
use desenv::Error;

use crate::test_utils::EnvUtil;

#[derive(Desenv, Debug)]
pub struct CustomNestedPrefixedField {
    #[desenv(nested)]
    pub field: NestedPrefixed,
}

#[derive(Desenv, Debug)]
#[desenv(prefix = "PREFIX_")]
pub struct NestedPrefixed {
    pub field: i32,
}

#[test]
fn deserialize_nested_prefixed_struct_field() {
    let field_value: i32 = 32;
    let _env_util: EnvUtil = EnvUtil::new("PREFIX_FIELD", field_value.to_string());

    let config: CustomNestedPrefixedField = desenv::load().unwrap();
    assert_eq!(config.field.field, field_value);
}

#[test]
fn deserialize_nested_prefixed_struct_field_fail() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value);

    let config: Result<CustomNestedPrefixedField, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::MissingVar("PREFIX_FIELD".to_string()).to_string()
    );
}

#[derive(Desenv, Debug)]
#[desenv(prefix = "PARENT_")]
pub struct PrefixedConfigWithNestedPrefixedField {
    #[desenv(nested)]
    pub field: NestedPrefixed,
}

#[test]
fn deserialize_prefixed_config_with_nested_prefixed_struct_field() {
    let field_value: i32 = 32;
    let _env_util: EnvUtil = EnvUtil::new("PARENT_PREFIX_FIELD", field_value.to_string());

    let config: PrefixedConfigWithNestedPrefixedField = desenv::load().unwrap();
    assert_eq!(config.field.field, field_value);
}

#[test]
fn deserialize_prefixed_config_with_nested_prefixed_struct_field_fail() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value);

    let config: Result<PrefixedConfigWithNestedPrefixedField, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::MissingVar("PARENT_PREFIX_FIELD".to_string()).to_string()
    );
}
