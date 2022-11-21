use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(default(env(value = "1")))]
    field: String
}

fn main(){}