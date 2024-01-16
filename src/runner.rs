use crate::models::Config;
use std::time::Duration;
use tokio::time::sleep;

pub async fn run(config: Config) {
    sleep(Duration::from_secs(config.seconds)).await;
}
