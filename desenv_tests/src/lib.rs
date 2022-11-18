#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests {
    use crate::test_utils::EnvUtil;
    use desenv::Desenv;

    #[derive(Desenv)]
    pub struct Config {
        field: String,
    }

    #[test]
    fn works() {
        let _env_util: EnvUtil = EnvUtil::new("FIELD", "value".to_string());
        let c: Config = desenv::load().unwrap();
        assert_eq!(c.field, "value");
    }

    #[derive(Desenv)]
    pub struct Config2 {
        field: i32,
    }

    #[test]
    fn works_with_i32() {
        let _env_util: EnvUtil = EnvUtil::new("FIELD", 10.to_string());
        let c: Config2 = desenv::load().unwrap();
        assert_eq!(c.field, 10);
    }

    #[derive(Desenv)]
    #[desenv(prefix = "PREFIX_")]
    pub struct Config3 {
        field: i32,
    }

    #[test]
    fn works_with_i32_and_prefix() {
        let _env_util: EnvUtil = EnvUtil::new("PREFIX_FIELD", 10.to_string());
        let c: Config3 = desenv::load().unwrap();
        assert_eq!(c.field, 10);
    }

    #[derive(Desenv)]
    #[desenv(prefix = "PREFIX_")]
    pub struct Config4 {
        #[desenv(nested)]
        field: Nested4,
    }

    #[derive(Desenv)]
    pub struct Nested4 {
        field: String,
    }

    #[test]
    fn works_with_nested() {
        let _env_util: EnvUtil = EnvUtil::new("PREFIX_FIELD", "value".to_string());
        let c: Config4 = desenv::load().unwrap();
        assert_eq!(c.field.field, "value");
    }

    #[derive(Desenv)]
    pub struct Config5 {
        field: Option<String>,
    }

    #[test]
    fn works_with_optional() {
        let _env_util: EnvUtil = EnvUtil::new("FIELD", "value".to_string());
        let c: Config5 = desenv::load().unwrap();
        assert!(c.field.is_some());
        assert_eq!(c.field.unwrap(), "value");
    }

    #[derive(Desenv)]
    pub struct Config6 {
        field: Vec<String>,
    }

    #[test]
    fn works_with_vector() {
        let _env_util: EnvUtil = EnvUtil::new("FIELD", "value1,value2".to_string());
        let c: Config6 = desenv::load().unwrap();
        assert_eq!(c.field, vec!["value1", "value2"]);
    }
}
