use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(default(env))]
    field: String
}

fn main(){}