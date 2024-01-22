use crate::models::{Config, Display};
use dirs;
use std::rc::Rc;
use std::{fs, path::Path};
use toml::Table;
use toml::Value;

pub fn read_config() -> Result<Rc<Config>, String> {
    let found = Path::new(&get_config_file_path()).exists();
    if !found {
        return Result::Err(format!(
            "Could not find config file.\nLooking at {}.\nExiting.",
            get_config_file_path(),
        ));
    }

    let toml_str = match fs::read_to_string(get_config_file_path()) {
        Err(error) => {
            return Result::Err(format!("Could not read config file.\n{}\nExiting.", error))
        }
        Ok(value) => value,
    };

    let toml_map = toml_str.parse::<Table>().unwrap();

    let mut config = Config {
        app: "".to_string(),
        config_file: "".to_string(),
        displays: vec![],
        seconds: 0,
        app_config: "".to_string(),
    };

    for (key, value) in toml_map {
        if key == "app" {
            config.app = check_value_valid(key, value)?;
        } else if key == "config_file" {
            config.config_file = check_value_valid(key, value)?;
        } else if key == "seconds" {
            config.seconds = check_value_valid(key, value)?.parse().unwrap_or(1800);
        } else if key == "Config" {
        } else {
            let display = parse_display_struct(value);
            match display {
                Err(_) => continue,
                Ok(display) => config.displays.push(display),
            }
        }
    }

    Ok(Rc::new(config))
}

fn check_value_valid(key: String, value: Value) -> Result<String, String> {
    match value.to_string().trim() {
        "" => Err(format!("Invalid input for key {}", key)),
        string_val => Ok(string_val.to_string()),
    }
}

fn get_data_from_value(key: String, value: Value) -> Result<String, String> {
    match value.get(key.to_string()) {
        Some(data) => Ok(check_value_valid(key, value)?),
        None => Err(format!("Invalid input for key {}", key)),
    }
}

fn parse_display_struct(data: Value) -> Result<Display, String> {
    let output_name_value = data.get("keyword").unwrap().clone().to_string();
    let file_value = data.get("file");
    let directoy_value = data.get("directoy");

    let mut display = Display {
        keyword: output_name_value.unwrap().to_string(),
        file: None,
        directoy: None,
    };

    if !output_name_value.is_some() {
        return Err("Please specify output name.".to_string());
    } else {
        display.keyword = output_name_value.unwrap().to_string();
    }

    if file_value.is_some() {
        display.file = Some(file_value.unwrap().to_string());
    }

    if directoy_value.is_some() {
        display.directoy = Some(directoy_value.unwrap().to_string());
    }

    if !display.directoy.is_some() && !display.file.is_some() {
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
