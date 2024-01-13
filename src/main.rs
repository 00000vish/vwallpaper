use std::process::{self};

mod config;

fn main() {
    let something = config::reader::read_config();
    match something {
        Err(error) => exit_with_msg(Some(&error)),
        Ok(_) => (),
    }
}

fn exit_with_msg(message: Option<&str>) {
    match message {
        Some(n) => println!("{}", n),
        None => (),
    }
    process::exit(1);
}
