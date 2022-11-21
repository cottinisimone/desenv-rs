use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(rename(value = ""))]
    field: String
}

fn main(){}