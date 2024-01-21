use crate::{app::App, models::Config};
use std::{rc::Rc, time::Duration};
use tokio::time::sleep;

pub struct Runner {
    config: Rc<Config>,
    app: Option<App>,
}

impl Runner {
    pub fn new(config: Rc<Config>) -> Self {
        Self { config, app: None }
    }

    pub async fn run(&mut self) {
        if !self.initialize() {
            return;
        }
        self.run_loop().await;
    }

    fn initialize(&mut self) -> bool {
        let app = match App::new(self.config.clone()) {
            None => return false,
            Some(value) => value,
        };
        self.app = Some(app);
        return true;
    }

    async fn run_loop(&self) {
        loop {
            if let Some(app) = &self.app {
                app.start();
            }
            sleep(Duration::from_secs(self.config.seconds)).await;
        }
    }
}
