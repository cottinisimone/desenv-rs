use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv = "value"]
    field: String
}

fn main(){}