use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(separator = ',')]
    field: Vec<String>
}

fn main() {}