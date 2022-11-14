#[cfg(test)]
mod tests {
    use desenv::Desenv;

    #[derive(Desenv)]
    pub struct Config {
        field: String,
    }

    #[test]
    fn works() {
        std::env::set_var("FIELD", "value");
        let c: Config = desenv::load().unwrap();
        assert_eq!(c.field, "value");
    }

    #[derive(Desenv)]
    pub struct Config2 {
        field: i32,
    }

    #[test]
    fn works_with_i32() {
        std::env::set_var("FIELD", 10.to_string());
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
        std::env::set_var("PREFIX_FIELD", 10.to_string());
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
        // TODO: this should be PREFIX_FIELD anyway
        std::env::set_var("FIELD", "value");
        let c: Config4 = desenv::load().unwrap();
        assert_eq!(c.field.field, "value");
    }
}
