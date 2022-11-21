use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(nested, default)]
    field: NestedConfig
}

#[derive(Desenv, Default)]
struct NestedConfig {
    nested_field: String,
}

fn main() {}