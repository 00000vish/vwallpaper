use crate::{helpers, models::Config};
use std::{process::Command, rc::Rc};

pub struct App {
    config: Rc<Config>,
    app_config: String,
}

impl App {
    pub fn new(config: Rc<Config>) -> Option<Self> {
        let app_config = helpers::read_file(config.config_file.clone())?;
        Some(Self { config, app_config })
    }

    pub fn start(&self) {
        self.stop();
    }

    pub fn stop(&self) -> bool {
        match Command::new("kilall").arg(self.config.app.clone()).output() {
            Err(_) => return false,
            Ok(_) => return true,
        }
    }
}
