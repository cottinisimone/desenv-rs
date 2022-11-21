use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(separator = 1)]
    field: String
}

fn main(){}