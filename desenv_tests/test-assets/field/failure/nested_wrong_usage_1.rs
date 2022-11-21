use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(nested = true)]
    field: String
}

fn main(){}