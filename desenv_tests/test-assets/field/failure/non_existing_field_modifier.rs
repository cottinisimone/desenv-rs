use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(non_existing = ',')]
    field: String
}

fn main(){}