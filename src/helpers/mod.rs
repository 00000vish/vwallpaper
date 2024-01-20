use std::fs::{self, ReadDir};

use rand::seq::SliceRandom;

use crate::models::Display;

pub fn get_wallpaper(display: Display) -> Result<String, String> {
    match display.file {
        Some(value) => return Ok(value),
        None => (),
    }
    let directory = match display.directoy {
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
