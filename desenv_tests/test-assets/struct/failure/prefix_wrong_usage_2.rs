use desenv::Desenv;

#[derive(Desenv)]
#[desenv(prefix(value = ""))]
struct Config {
    field: String
}

fn main(){}