use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;

use desenv::Desenv;
use desenv::Error;

use crate::test_utils::EnvUtil;

#[derive(Desenv, Debug)]
pub struct SimpleString {
    pub field: String,
}

#[test]
fn deserialize_simple_string_field_without_field_attr() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value);

    let config: SimpleString = desenv::load().unwrap();
    assert_eq!(config.field, field_value);
}

#[test]
fn deserialize_simple_string_field_without_field_attr_fail() {
    let config: Result<SimpleString, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::MissingVar("FIELD".to_string()).to_string()
    );
}

#[derive(Desenv, Debug)]
pub struct SimpleOsString {
    pub field: OsString,
}

#[test]
fn deserialize_simple_os_string_field_without_field_attr() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value);

    let config: SimpleOsString = desenv::load().unwrap();
    assert_eq!(config.field, field_value);
}

#[test]
fn deserialize_non_utf8_os_string_field_without_field_attr() {
    let field_value: OsString = OsString::from_vec(vec![255]);
    std::env::set_var("FIELD", field_value.clone());

    let config: SimpleOsString = desenv::load().unwrap();
    assert_eq!(config.field, field_value);
    std::env::remove_var("FIELD");
}

#[test]
fn deserialize_simple_os_string_field_without_field_attr_fail() {
    let config: Result<SimpleOsString, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::MissingVar("FIELD".to_string()).to_string()
    );
}

#[derive(Desenv, Debug)]
pub struct ConfigWithBool {
    pub field: bool,
}

#[test]
fn deserialize_bool_field() {
    let field_value: bool = true;
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value.to_string());

    let config: ConfigWithBool = desenv::load().unwrap();
    assert!(config.field);
}

#[derive(Desenv, Debug)]
pub struct OptionalField {
    pub field: Option<bool>,
}

#[test]
fn deserialize_optional_field_to_none() {
    let config: OptionalField = desenv::load().unwrap();
    assert_eq!(config.field, None);
}

#[test]
fn deserialize_optional_field_to_some() {
    let field_value: bool = true;
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value.to_string());

    let config: OptionalField = desenv::load().unwrap();
    assert_eq!(config.field, Some(true));
}

#[test]
fn deserialize_optional_field_to_some_as_false() {
    let field_value: bool = false;
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value.to_string());

    let config: OptionalField = desenv::load().unwrap();
    assert_eq!(config.field, Some(false));
}

#[derive(Desenv, Debug)]
pub struct VecField {
    pub field: Vec<bool>,
}

#[test]
fn deserialize_empty_vector_with_default_separator_when_var_is_empty() {
    let field_value: &str = "";
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value);

    let config: VecField = desenv::load().unwrap();
    assert!(config.field.is_empty());
}

#[test]
fn deserialize_well_formatted_list_of_values() {
    // Space in between is not considered
    let _env_util: EnvUtil = EnvUtil::new("FIELD", "true, false,true, false");

    let config: VecField = desenv::load().unwrap();
    assert_eq!(config.field.len(), 4);
    assert_eq!(config.field, vec![true, false, true, false]);
}

#[test]
fn fail_to_deserialize_wrong_list_of_values() {
    let _env_util: EnvUtil = EnvUtil::new("FIELD", "true, false,true1, false2");

    let config: Result<VecField, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::ParseFromStr("ParseBoolError".to_string()).to_string()
    );
}

#[test]
fn fail_to_deserialize_for_non_existing_env_var() {
    let config: Result<VecField, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::MissingVar("FIELD".to_string()).to_string()
    );
}

#[derive(Desenv, Debug)]
pub struct CustomEnumField {
    pub field: TestEnum,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TestEnum {
    Val1,
    Val2,
}

impl std::str::FromStr for TestEnum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "val_1" => Ok(Self::Val1),
            "val_2" => Ok(Self::Val2),
            v => Err(Error::Custom(format!("invalid value {}", v))),
        }
    }
}

#[test]
fn deserialize_custom_enum_field_with_valid_value() {
    let _env_util: EnvUtil = EnvUtil::new("FIELD", "val_1");

    let config: CustomEnumField = desenv::load().unwrap();
    assert_eq!(config.field, TestEnum::Val1);
}

#[test]
fn deserialize_custom_enum_field_with_invalid_value() {
    let _env_util: EnvUtil = EnvUtil::new("FIELD", "val_12");

    let config: Result<CustomEnumField, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::ParseFromStr("invalid value val_12".to_string()).to_string()
    );
}

#[test]
fn deserialize_custom_enum_field_with_empty_value() {
    let config: Result<CustomEnumField, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::MissingVar("FIELD".to_string()).to_string()
    );
}
