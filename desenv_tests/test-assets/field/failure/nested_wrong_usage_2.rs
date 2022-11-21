use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(nested(value = true))]
    field: String
}

fn main(){}