use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(default(value = "value", env = "ENV"))]
    field: String
}

fn main(){}