use crate::{app::App, models::Config};
use std::time::Duration;
use tokio::time::sleep;

pub struct Runner<'r> {
    config: Config,
    app: Option<App<'r>>,
}

impl<'r> Runner<'r> {
    pub fn new(config: Config) -> Self {
        Self { config, app: None }
    }

    pub async fn run(&mut self) {
        self.initialize();
        self.run_loop().await;
    }

    fn initialize(&mut self) {
        self.app = Some(App::new(&self.config));
    }

    async fn run_loop(&self) {
        loop {
            if let Some(app) = &self.app {
                app.start(&self.config);
            }
            sleep(Duration::from_secs(self.config.seconds)).await;
        }
    }
}
