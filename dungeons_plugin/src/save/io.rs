use std::fs;
use std::path::{Path, PathBuf};

use bevy::log;
use bevy::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Resource, Clone)]
pub struct SavePaths {
    pub dir: PathBuf,
    pub global: PathBuf,
    pub run: PathBuf,
}

impl Default for SavePaths {
    fn default() -> Self {
        Self::new()
    }
}

impl SavePaths {
    pub fn new() -> Self {
        let dir = dirs::data_local_dir()
            .unwrap_or_else(std::env::temp_dir)
            .join("dungeons_sweeper");
        Self {
            global: dir.join("global_save.ron"),
            run: dir.join("run_save.ron"),
            dir,
        }
    }

    pub fn ensure_dir(&self) {
        if let Err(e) = fs::create_dir_all(&self.dir) {
            log::warn!("save dir create failed: {e}");
        }
    }
}

pub fn read_ron<T: DeserializeOwned>(path: &Path) -> Option<T> {
    let text = fs::read_to_string(path).ok()?;
    ron::from_str(&text).ok()
}

pub fn write_ron<T: Serialize>(path: &Path, value: &T) -> bool {
    match ron::ser::to_string_pretty(value, ron::ser::PrettyConfig::default()) {
        Ok(text) => fs::write(path, text).is_ok(),
        Err(e) => {
            log::error!("serialize save failed: {e}");
            false
        }
    }
}

pub fn file_exists(path: &Path) -> bool {
    path.is_file()
}

pub fn delete_file(path: &Path) {
    if path.is_file() {
        let _ = fs::remove_file(path);
    }
}
