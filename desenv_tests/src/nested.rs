use desenv::{Desenv, Error};

use crate::test_utils::EnvUtil;

#[derive(Desenv, Debug)]
pub struct CustomNestedField {
    #[desenv(nested)]
    pub field: Nested,
}

#[derive(Desenv, Debug)]
pub struct Nested {
    pub field: i32,
}

#[test]
fn deserialize_nested_struct_field() {
    let field_value: i32 = 18;
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value.to_string());

    let config: CustomNestedField = desenv::load().unwrap();
    assert_eq!(config.field.field, field_value);
}

#[test]
fn deserialize_nested_struct_field_fail() {
    let config: Result<CustomNestedField, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::MissingVar("FIELD".to_string()).to_string()
    );
}
