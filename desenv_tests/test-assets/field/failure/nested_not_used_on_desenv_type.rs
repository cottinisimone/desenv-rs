use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(nested)]
    field: String
}

fn main(){}