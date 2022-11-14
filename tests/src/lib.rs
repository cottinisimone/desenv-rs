#[cfg(test)]
mod tests {
    use desenv::Desenv;

    #[derive(Desenv)]
    pub struct Config {
        field: String,
    }

    #[test]
    fn could_compile() {
        let c: Config = desenv::load().unwrap();
        assert!(c.field.is_empty());
    }
}
