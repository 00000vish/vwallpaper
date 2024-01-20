use crate::{
    apps::{self, App},
    models::Config,
};
use std::time::Duration;
use tokio::time::sleep;

pub struct Runner {
    config: Config,
    app: Option<Box<dyn App>>,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        Self { config, app: None }
    }

    pub async fn run(&mut self) {
        if !self.initialize() {
            return;
        }
        self.run_loop().await;
    }

    fn initialize(&mut self) -> bool {
        self.app = match apps::get_app(&self.config) {
            Some(value) => Some(value),
            None => return false,
        };
        return true;
    }

    async fn run_loop(&self) {
        loop {
            if let Some(app) = &self.app {
                app.run(&self.config);
            }
            sleep(Duration::from_secs(self.config.seconds)).await;
        }
    }
}
