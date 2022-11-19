use desenv::Desenv;
use desenv::Error;

use crate::test_utils::EnvUtil;

#[derive(Desenv, Debug)]
pub struct AttrAsPath {
    #[desenv(default)]
    pub field: String,
}

#[test]
fn deserialize_field_with_std_default_using_env_var_value() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value);

    let config: AttrAsPath = desenv::load().unwrap();
    assert_eq!(config.field, field_value);
}

#[test]
fn deserialize_field_with_std_default_using_std_default() {
    let config: AttrAsPath = desenv::load().unwrap();
    assert_eq!(config.field, "");
}

#[derive(Desenv, Debug)]
pub struct AttrWithValue {
    #[desenv(default(value = "default_value"))]
    pub field: String,
}

#[test]
fn deserialize_field_with_value_default_as_array_using_env_var_value() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value);

    let config: AttrWithValue = desenv::load().unwrap();
    assert_eq!(config.field, field_value);
}

#[test]
fn deserialize_field_with_value_default_as_array_using_value_default() {
    let config: AttrWithValue = desenv::load().unwrap();
    assert_eq!(config.field, "default_value");
}

#[derive(Desenv, Debug)]
pub struct AttrWithI32_2 {
    #[desenv(default(value = "7"))]
    pub field: i32,
}

#[test]
fn deserialize_i32_field_with_default_field_attr_using_default() {
    let config: AttrWithI32_2 = desenv::load().unwrap();
    assert_eq!(config.field, 7);
}

#[derive(Desenv, Debug)]
pub struct AttrWithI32AndWrongValue {
    // Note that this default value is wrong (not an i32)
    #[desenv(default(value = "default_value"))]
    pub field: i32,
}

#[test]
fn deserialize_i32_field_with_default_field_attr() {
    let field_value: i32 = 10;
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value.to_string());

    let config: AttrWithI32AndWrongValue = desenv::load().unwrap();
    assert_eq!(config.field, field_value);
}

#[test]
fn deserialize_i32_field_with_default_field_attr_fail_for_wrong_default() {
    let config: Result<AttrWithI32AndWrongValue, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::Custom("Cannot parse env var: ParseIntError { kind: InvalidDigit }".to_string()).to_string()
    );
}

#[derive(Desenv, Debug)]
pub struct AttrWithEnvVar {
    #[desenv(default(env = "DEFAULT_ENV"))]
    pub field: String,
}

#[test]
fn deserialize_field_with_env_default_using_env_var_value() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value);

    let config: AttrWithEnvVar = desenv::load().unwrap();
    assert_eq!(config.field, field_value);
}

#[test]
fn deserialize_field_with_env_default_using_env_var_default() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("DEFAULT_ENV", field_value);

    let config: AttrWithEnvVar = desenv::load().unwrap();
    assert_eq!(config.field, "value");
}

#[test]
fn deserialize_field_with_env_default_fail_using_missing_env_var_default() {
    let config: Result<AttrWithEnvVar, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::MissingVar("FIELD".to_string()).to_string()
    );
}

#[derive(Desenv, Debug)]
pub struct AttrWithValueAndOptionalField {
    #[desenv(default(value = "default_value"))]
    pub field: Option<String>,
}

#[test]
fn deserialize_optional_field_with_value_default_using_env_var_default() {
    let config: AttrWithValueAndOptionalField = desenv::load().unwrap();
    assert_eq!(config.field, Some("default_value".to_string()));
}

#[derive(Desenv, Debug)]
pub struct AttrWithEnvVarAndOptionalField {
    #[desenv(default(env = "DEFAULT_ENV"))]
    pub field: Option<String>,
}

#[test]
fn deserialize_optional_field_with_env_default_using_env_var_default() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("DEFAULT_ENV", field_value);

    let config: AttrWithEnvVarAndOptionalField = desenv::load().unwrap();
    assert_eq!(config.field, Some("value".to_string()));
}

#[derive(Desenv, Debug)]
pub struct AttrWithEnvVarAndVectorField {
    #[desenv(default(env = "DEFAULT_ENV"))]
    pub field: Vec<bool>,
}

#[test]
fn deserialize_vector_field_with_env_default_using_env_var_default() {
    let field_value: &str = "true, false,true, false";
    let _env_util: EnvUtil = EnvUtil::new("DEFAULT_ENV", field_value);

    let config: AttrWithEnvVarAndVectorField = desenv::load().unwrap();
    assert_eq!(config.field.len(), 4);
    assert_eq!(config.field, vec![true, false, true, false]);
}

#[derive(Desenv, Debug)]
pub struct VecField {
    #[desenv(default(value = "true, false"))]
    pub field: Vec<bool>,
}

#[test]
fn deserialize_without_env_var_set_default_values() {
    let config: VecField = desenv::load().unwrap();
    assert_eq!(config.field, vec![true, false]);
}
