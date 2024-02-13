use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::{error::HibikiError, sound::SoundKind};

pub struct Scene {
    pub entries: Box<[SceneEntry]>,
}

impl Scene {
    pub fn load(scene_path: &PathBuf) -> Result<Self, HibikiError> {
        if scene_path.exists() {
            // necessary on Linux as file dialog allows selecting dirs
            if !scene_path.is_file() {
                return Err(HibikiError::NotAFile(scene_path.clone()));
            }
            let file = File::open(&scene_path).map_err(HibikiError::InternalError)?;
            Ok(Self {
                entries: ron::de::from_reader(&file).map_err(HibikiError::BrokenScene)?,
            })
        } else {
            Ok(Self {
                entries: Box::new([]),
            })
        }
    }

    pub fn save(&self, scene_path: &PathBuf) -> Result<(), HibikiError> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&scene_path)
            .map_err(HibikiError::InternalError)?;
        ron::ser::to_writer_pretty(file, &self.entries, PrettyConfig::default())
            .map_err(HibikiError::SceneSerialize)
    }
}

#[derive(Deserialize, Serialize)]
pub struct SceneEntry {
    pub sound_path: PathBuf,
    pub controller: SoundKind,
    pub volume: f64,
    pub pan: f64,
    pub color: [u8; 3],
}
