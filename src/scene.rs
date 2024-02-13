use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::sound::SoundKind;

pub struct Scene {
    pub entries: Box<[SceneEntry]>,
}

impl Scene {
    pub fn load(scene_path: &PathBuf) -> Self {
        if scene_path.exists() {
            let file = File::open(&scene_path).expect("Couldn't load or create scene file");
            Self {
                entries: ron::de::from_reader(&file).expect("Broken scene file"),
            }
        } else {
            Self {
                entries: Box::new([]),
            }
        }
    }

    pub fn save(&self, scene_path: &PathBuf) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&scene_path)
            .expect("Couldn't open or create scene file");
        ron::ser::to_writer_pretty(file, &self.entries, PrettyConfig::default())
            .expect("Couldn't save scene file");
    }
}

#[derive(Deserialize, Serialize)]
pub struct SceneEntry {
    pub sound_path: PathBuf,
    pub controller: SoundKind,
}
