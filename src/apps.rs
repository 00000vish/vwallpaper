use crate::models::Config;

mod hyprpaper;

pub trait App {
    fn initialize(&mut self, config: Config);
    fn get_name(&self) -> String;
}

pub fn get_app() -> Box<dyn App> {
    let hyper = hyprpaper::Hyprpaper::new();
    return Box::new(hyper);
}
