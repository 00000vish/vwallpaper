use self::hyprpaper::Hyprpaper;
use crate::models::Config;

mod hyprpaper;

pub trait App {
    fn get_name(&self) -> String;
    fn run(&self, config: &Config);
}

pub fn get_app(config: &Config) -> Option<Box<dyn App>> {
    let apps = vec![Hyprpaper::new()];
    for app in apps {
        if app.get_name() == config.app {
            return Some(Box::new(app));
        }
    }
    return None;
}
