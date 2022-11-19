use desenv::Desenv;

#[derive(Desenv)]
#[desenv(prefix = "")]
struct Config {
    field: String
}

fn main(){}