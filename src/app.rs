use crate::models::Config;
use std::{process::Command, rc::Rc};

pub struct App {
    config: Rc<Config>,
}

impl App {
    pub fn new(config: Rc<Config>) -> Option<Self> {
        Some(Self { config })
    }

    pub fn start(&self) {
        self.stop();
    }

    pub fn stop(&self) -> bool {
        match Command::new("killall")
            .arg(self.config.app.clone())
            .output()
        {
            Err(_) => return false,
            Ok(_) => return true,
        }
    }
}
