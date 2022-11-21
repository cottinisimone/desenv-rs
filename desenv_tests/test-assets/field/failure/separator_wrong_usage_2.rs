use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(separator(value = ""))]
    field: String
}

fn main(){}