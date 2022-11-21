use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(rename = ',')]
    field: String
}

fn main(){}