use desenv::Desenv;

#[derive(Desenv)]
#[desenv(prefix = '_')]
struct Config {
    field: String
}

fn main(){}