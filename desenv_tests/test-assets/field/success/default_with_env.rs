use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(default(env = "ENV"))]
    field: String
}

fn main() {}