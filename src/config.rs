use std::u64;

pub mod reader {
    use super::Config;
    use super::Display;
    use dirs;
    use std::u64;
    use std::{fs, path::Path};
    use toml::Table;
    use toml::Value;

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

        let mut config = Config {
            app: "".to_string(),
            displays: vec![],
            seconds: 0,
        };

        for (key, value) in toml_map {
            println!("{} => {} ", key, value);
            if key == "app" {
                config.app = value.to_string();
            } else if key == "seconds" {
                config.seconds = value.to_string().parse::<u64>().unwrap();
            } else {
                let display = parse_display_struct(value);
                match display {
                    Err(_) => continue,
                    Ok(display) => config.displays.push(display),
                }
            }
        }

        Ok(config)
    }

    fn parse_display_struct(data: Value) -> Result<Display, String> {
        let output_name_value = data.get("output_name");
        let file_value = data.get("file");
        let directoy_value = data.get("directoy");

        let display = Display {
            name: "".to_string(),
            file: None,
            directoy: None,
        };

        if !output_name_value.is_some() {
            return Err("Please specify output name.".to_string());
        }
        if !file_value.is_some() && !directoy_value.is_some() {
            return Err("Please specify the folder or image to use.".to_string());
        }

        Ok(display)
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
    seconds: u64,
}

pub struct Display {
    name: String,
    file: Option<String>,
    directoy: Option<String>,
}
