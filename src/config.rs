use crate::helpers;
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

    let toml_map = match toml_str.parse::<Table>() {
        Ok(value) => value,
        Err(error) => return Err(error.to_string()),
    };

    let mut config = Config {
        app: "".to_string(),
        config_file: "".to_string(),
        displays: vec![],
        seconds: 0,
        app_config: "".to_string(),
    };

    for (key, value) in toml_map {
        if key == "app" {
            config.app = check_value_valid(key, &value)?;
        } else if key == "config_file" {
            config.config_file = parse_path(check_value_valid(key, &value)?);
        } else if key == "seconds" {
            config.seconds = check_value_valid(key, &value)?.parse().unwrap_or(1800);
        } else if key == "Config" {
            config.app_config = get_app_config(value)?;
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

fn get_app_config(data: Value) -> Result<String, String> {
    let template = get_data_from_value("template".to_string(), &data);
    let file = get_data_from_value("file".to_string(), &data);

    if !file.is_ok() && !template.is_ok() {
        return Err("Please specify file or directory".to_string());
    }

    if file.is_ok() {
        match helpers::read_file(parse_path(file.unwrap().to_string())) {
            None => return Err("Could not read config file".to_string()),
            Some(value) => return Ok(value),
        }
    } else {
        return Ok(template.unwrap());
    }
}

fn check_value_valid(key: String, value: &Value) -> Result<String, String> {
    let value_str = match value {
        Value::String(string_val) => string_val.to_string(),
        _ => value.to_string(),
    };

    match value_str.trim() {
        "" => Err(format!("Invalid input for key {}", key)),
        string_val => Ok(string_val.to_string()),
    }
}

fn get_data_from_value(key: String, value: &Value) -> Result<String, String> {
    match value.get(key.to_string()) {
        Some(data) => Ok(check_value_valid(key, data)?),
        None => Err(format!("Invalid input for key {}", key)),
    }
}

fn parse_display_struct(data: Value) -> Result<Display, String> {
    let keyword = get_data_from_value("keyword".to_string(), &data)?;
    let file = get_data_from_value("file".to_string(), &data);
    let directory = get_data_from_value("directory".to_string(), &data);

    if !file.is_ok() && !directory.is_ok() {
        return Err("Please specify file or directory".to_string());
    }

    let display = Display {
        keyword,
        file: match file {
            Ok(value) => Some(parse_path(value)),
            Err(_) => None,
        },
        directory: match directory {
            Ok(value) => Some(parse_path(value)),
            Err(_) => None,
        },
    };

    Ok(display)
}

fn parse_path(data: String) -> String {
    let home_dir = dirs::home_dir();
    if !home_dir.is_some() {
        return data;
    }
    if data.starts_with("~") {
        return data.replace("~", home_dir.unwrap().to_str().unwrap());
    }
    data
}

fn get_config_file_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let home_dir_str = home_dir.to_str().unwrap();
    let file_path = format!("{}/.config/vwallpaper/config.toml", home_dir_str);
    file_path
}
