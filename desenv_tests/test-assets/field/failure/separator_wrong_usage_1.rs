use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(separator)]
    field: String
}

fn main(){}