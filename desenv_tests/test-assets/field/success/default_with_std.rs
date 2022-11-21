use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(default)]
    field: String
}

fn main() {}