use std::u64;

pub mod reader {
    use super::Config;
    use super::Display;
    use dirs;
    use std::{fs, path::Path};
    use toml::Table;

    pub fn read_config() -> Result<Config, String> {
        let found = Path::new(&get_config_file_path()).exists();
        if !found {
            return Result::Err(format!(
                "Could not find config file.\nLooking at {}.\nExiting.",
                get_config_file_path(),
            ));
        }

        let toml_str = fs::read_to_string(get_config_file_path());
        match toml_str {
            Err(error) => {
                return Result::Err(format!("Could not read config file.\n{}\nExiting.", error))
            }
            _ => (),
        }

        let toml_map = toml_str.unwrap().parse::<Table>().unwrap();

        for (key, value) in toml_map {
            println!("{} => {} ", key, value);
        }
        Err("error".to_string())
    }

    fn parse_display_struct() -> Result<Display, String> {
        Err("todo".to_string())
    }

    fn get_config_file_path() -> String {
        let home_dir = dirs::home_dir().unwrap();
        let home_dir_str = home_dir.to_str().unwrap();
        let file_path = format!("{}/.config/vwallpaper/config.toml", home_dir_str);
        file_path
    }
}

pub struct Config {
    app: String,
    displays: Vec<Display>,
}

pub struct Display {
    name: String,
    file: Option<String>,
    directoy: Option<String>,
    seconds: u64,
}
