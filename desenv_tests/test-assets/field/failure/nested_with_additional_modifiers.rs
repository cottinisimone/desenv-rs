use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(nested, rename = "renamed")]
    field: String
}

fn main(){}