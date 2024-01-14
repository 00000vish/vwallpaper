use std::process::{self};

mod config;
mod models;
mod runner;

fn main() {
    let config = config::read_config();
    match config {
        Err(ref error) => exit_with_msg(Some(&error)),
        Ok(_) => (),
    }

    let config = config.unwrap();
    runner::run(config)
}

fn exit_with_msg(message: Option<&str>) {
    match message {
        Some(n) => println!("{}", n),
        None => (),
    }
    process::exit(1);
}
