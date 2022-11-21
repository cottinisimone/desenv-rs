use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(default(value))]
    field: String
}

fn main(){}