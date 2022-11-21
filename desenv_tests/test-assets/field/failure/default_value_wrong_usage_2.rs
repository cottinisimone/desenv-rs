use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(default(value(value = "value")))]
    field: String
}

fn main(){}