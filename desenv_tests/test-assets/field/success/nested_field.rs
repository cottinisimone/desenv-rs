use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(nested)]
    field: NestedConfig
}

#[derive(Desenv)]
struct NestedConfig {
    nested_field: String,
}

fn main() {}