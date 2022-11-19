use desenv::Desenv;
use desenv::Error;

use crate::test_utils::EnvUtil;

#[derive(Desenv, Debug)]
pub struct Simple {
    #[desenv(rename = "rename_field")]
    pub field: String,
}

#[test]
fn deserialize_simple_string_field_with_rename_field_attr() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("rename_field", field_value);

    let config: Simple = desenv::load().unwrap();
    assert_eq!(config.field, field_value);
}

#[test]
fn deserialize_simple_string_field_with_rename_field_attr_fail() {
    let field_value: &str = "value";
    let _env_util: EnvUtil = EnvUtil::new("FIELD", field_value);

    let config: Result<Simple, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::MissingVar("rename_field".to_string()).to_string()
    );
}
