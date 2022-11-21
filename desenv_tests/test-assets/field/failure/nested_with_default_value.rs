use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(nested, default(value = "value"))]
    field: String
}

fn main(){}