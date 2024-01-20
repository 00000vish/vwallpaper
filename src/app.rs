use crate::models::Config;

pub struct App<'a> {
    config: &'a Config,
}

impl<'a> App<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
    pub fn start(&self, config: &Config) {}
    pub fn stop(&self) -> bool {
        return true;
    }
}
