use crate::apps::App;

pub struct Hyprpaper {}

impl Hyprpaper {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for Hyprpaper {
    fn get_name(&self) -> String {
        return "hyprpaper".to_string();
    }
    fn set_wallpaper(&self) {}
}
