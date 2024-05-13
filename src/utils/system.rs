extern crate chrono;

use chrono::Local;
use std::path::PathBuf;
use once_cell::sync::Lazy;
use dirs_next::config_dir;

use crate::configs::global::Global;

pub struct System;

impl System {

    pub const APP_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Global::APP_NAME);
        path
    });
    
    pub const SETTINGS_FILE: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = config_dir().expect("No config directory");
        path.push(Global::APP_NAME);
        path.push("paimon.yml");
        path
    });
    
    pub const README_FOLDER: Lazy<PathBuf> = Lazy::new(|| {
        let mut path = System::APP_FOLDER.clone();
        path.push("readme");
        path
    });

    pub fn date_time() -> String {
        let local_time = Local::now();
    
        let date_formated = local_time.format("%Y-%m-%d").to_string();
        let hour_formated = local_time.format("%H:%M:%S").to_string();
    
        format!("{} {}", date_formated, hour_formated)
    }

}
