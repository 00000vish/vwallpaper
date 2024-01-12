pub mod config {
    use dirs;
    use std::path::Path;
    use toml::Table;

    pub fn read_config() -> Result<Table, String> {
        let found = Path::new(&get_config_file_path()).exists();
        if !found {
            return Result::Err(format!(
                "Could not find config file.\n Looking at {}.\n Exiting.",
                get_config_file_path(),
            ));
        }
        Err("error".to_string())
    }

    fn get_config_file_path() -> String {
        let home_dir = dirs::home_dir().unwrap();
        let home_dir_str = home_dir.to_str().unwrap();
        let file_path = format!("{}/.config/vwallpaper/config.toml", home_dir_str);
        file_path
    }
}
