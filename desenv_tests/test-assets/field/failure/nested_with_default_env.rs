use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(nested, default(env = "env"))]
    field: String
}

fn main(){}