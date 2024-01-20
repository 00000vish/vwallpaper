pub struct Config {
    pub config_file: String,
    pub displays: Vec<Display>,
    pub seconds: u64,
}

pub struct Display {
    pub keyword: String,
    pub file: Option<String>,
    pub directoy: Option<String>,
}
