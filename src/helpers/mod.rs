use crate::models::Display;
use rand::seq::SliceRandom;
use std::{
    fs::{self, File},
    io::Write,
};

pub fn get_wallpaper(display: &Display) -> Result<String, String> {
    match &display.file {
        Some(value) => return Ok(value.to_string()),
        None => (),
    }
    let directory = match &display.directory {
        None => return Err("Please specify a wallpaper for directory.".to_string()),
        Some(value) => value,
    };
    let files = match fs::read_dir(directory) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    };

    let valid_files: Vec<String> = files
        .into_iter()
        .filter_map(|x| x.ok())
        .map(|x| x.path().display().to_string())
        .filter_map(|x| {
            if x.contains(".png") || x.contains(".jpg") || x.contains(".jpeg") {
                Some(x)
            } else {
                None
            }
        })
        .collect();

    if valid_files.len() == 0 {
        return Err("Not valid files in the directory".to_string());
    }

    let wallpaper_file = match valid_files.choose(&mut rand::thread_rng()) {
        None => return Err("Could find a wallpaper".to_string()),
        Some(value) => value,
    };

    Ok(wallpaper_file.to_string())
}

pub fn update_file(path: &String, content: &String) -> bool {
    let mut file = match File::create(path) {
        Err(_) => return false,
        Ok(value) => value,
    };

    _ = file.write_all(content.as_bytes());

    return true;
}

pub fn read_file(path: String) -> Option<String> {
    let file = match fs::read_to_string(path) {
        Err(_) => return None,
        Ok(value) => value,
    };

    return Some(file);
}
