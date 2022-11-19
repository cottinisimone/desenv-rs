use desenv::{Desenv, Error};

use crate::test_utils::EnvUtil;

#[derive(Desenv, Debug)]
pub struct VecField {
    #[desenv(separator = '|')]
    pub field: Vec<bool>,
}

#[test]
fn deserialize_vec_with_custom_separator() {
    let _env_util: EnvUtil = EnvUtil::new("FIELD", "true| false|true| false");

    let config: VecField = desenv::load().unwrap();
    assert_eq!(config.field, vec![true, false, true, false]);
}

#[test]
fn deserialize_vec_with_custom_separator_fail() {
    let _env_util: EnvUtil = EnvUtil::new("FIELD", "true| false|true, false");

    let config: Result<VecField, Error> = desenv::load();
    assert!(config.is_err());
    assert_eq!(
        config.unwrap_err().to_string(),
        Error::ParseFromStr("ParseBoolError".to_string()).to_string()
    );
}
