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
    #[desenv(prefix = "PREFIX4_")]
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
        // TODO: this should be PREFIX_FIELD anyway
        let _env_util: EnvUtil = EnvUtil::new("PREFIX4_FIELD", "value".to_string());
        let c: Config4 = desenv::load().unwrap();
        assert_eq!(c.field.field, "value");
    }
}
