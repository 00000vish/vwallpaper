mod app;
mod config;
mod helpers;
mod models;
mod runner;

use runner::Runner;
use std::process::{self};

#[tokio::main]
async fn main() -> () {
    let config = match config::read_config() {
        Ok(value) => value,
        Err(error) => {
            println!("{}", error);
            process::exit(1);
        }
    };

    let mut runner = Runner::new(config);
    runner.run().await;
}
