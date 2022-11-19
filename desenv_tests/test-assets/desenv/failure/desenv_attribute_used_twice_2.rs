use desenv::Desenv;

#[derive(Desenv)]
#[desenv(prefix = "PREFIX_")]
#[desenv(prefix = "PREFIX_")]
struct Config {
    field: String
}

fn main(){}