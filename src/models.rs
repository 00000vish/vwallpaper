pub struct Config {
    pub app: String,
    pub displays: Vec<Display>,
    pub seconds: u64,
}

pub struct Display {
    pub name: String,
    pub file: Option<String>,
    pub directoy: Option<String>,
}
