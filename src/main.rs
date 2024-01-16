use std::process::{self};

mod config;
mod models;
mod runner;

#[tokio::main]
async fn main() -> () {
    let config = match config::read_config() {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    };

    runner::run(config).await;
}
