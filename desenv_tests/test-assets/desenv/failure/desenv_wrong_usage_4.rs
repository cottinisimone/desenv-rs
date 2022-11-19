use desenv::Desenv;

#[derive(Desenv)]
#[desenv = "hello"]
struct Config {
    field: String
}

fn main(){}