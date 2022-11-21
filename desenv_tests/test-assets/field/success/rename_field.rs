use desenv::Desenv;

#[derive(Desenv)]
struct Config {
    #[desenv(rename = "field_2")]
    field: String
}

fn main() {}