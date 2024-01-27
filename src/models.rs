pub struct Config {
    pub app: String,
    pub config_file: String,
    pub seconds: u64,
    pub displays: Vec<Display>,
    pub app_config: String,
}

pub struct Display {
    pub keyword: String,
    pub file: Option<String>,
    pub directory: Option<String>,
}
