use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv]
    field: String
}

fn main(){}