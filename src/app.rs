use crate::{helpers, models::Config};
use std::{
    process::{Child, Command},
    rc::Rc,
};

pub struct App {
    config: Rc<Config>,
    app_instance: Option<Child>,
}

impl App {
    pub fn new(config: Rc<Config>) -> Option<Self> {
        Some(Self {
            config,
            app_instance: None,
        })
    }

    pub fn start(&mut self) {
        let mut app_config = self.config.app_config.clone();
        for display in &self.config.displays {
            let wallpaper = match helpers::get_wallpaper(&display) {
                Err(_) => continue,
                Ok(value) => value,
            };
            app_config = app_config.replace(&display.keyword, &wallpaper);
        }
        helpers::update_file(&self.config.config_file, &app_config);
        self.close();
        self.open();
    }

    fn open(&mut self) {
        let instance = Command::new(&self.config.app).spawn();
        self.app_instance = instance.ok();
    }

    fn close(&mut self) {
        if let Some(child_process) = &mut self.app_instance {
            _ = child_process.kill();
            _ = child_process.wait();
        } else {
            self.kill();
        }
    }

    fn kill(&self) {
        _ = Command::new("killall").arg(&self.config.app).output();
    }
}
