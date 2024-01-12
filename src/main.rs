use toml::Table;

mod config;

fn main() {
    let something = config::config::read_config();
    match something {
        Err(error) => println!("{}", error),
        Ok(_) => (),
    }
}

fn exit(message: String) {}
