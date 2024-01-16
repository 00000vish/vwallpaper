use crate::{apps::App, models::Config};

pub struct Hyprpaper {
    name: Option<String>,
}
impl Hyprpaper {
    pub fn new() -> Self {
        Self {
            name: Some("".to_string()),
        }
    }
}

impl App for Hyprpaper {
    fn initialize(&mut self, config: Config) {
        self.name = Some(config.app);
    }
    fn get_name(&self) -> String {
        let val = &self.name.as_ref().unwrap();
        return val.to_string();
    }
}
