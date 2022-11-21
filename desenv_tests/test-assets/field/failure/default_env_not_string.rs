use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(default(env = 0))]
    field: String
}

fn main(){}