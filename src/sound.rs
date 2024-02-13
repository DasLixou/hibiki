use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Cursor, Read},
    path::PathBuf,
    sync::Arc,
};

use rodio::{Decoder, Sink};
use serde::{Deserialize, Serialize};

use crate::error::HibikiError;

pub struct Sound {
    pub kind: SoundKind,
    pub source: SoundSource,
    /// Sink of the audio, unused by `Trigger`
    pub sink: Sink,
    /// Used by `Activating` for storing its state
    pub state: bool,
    pub volume: f32,
    pub pan: f32,
}

pub struct SoundSource {
    pub path: PathBuf,
    pub data: Arc<[u8]>,
}

impl SoundSource {
    pub fn from_file(path: PathBuf) -> Result<SoundSource, HibikiError> {
        if !path.exists() {
            return Err(HibikiError::DoesNotExist(path));
        }
        // necessary on Linux as file dialog allows selecting dirs
        if !path.is_file() {
            return Err(HibikiError::NotAFile(path));
        }
        let mut reader = BufReader::new(File::open(&path).map_err(HibikiError::InternalError)?);
        let mut bytes = vec![];
        reader
            .read_to_end(&mut bytes)
            .map_err(HibikiError::InternalError)?;
        Ok(Self {
            path,
            data: bytes.into(),
        })
    }

    pub fn source(&self) -> Decoder<Cursor<Arc<[u8]>>> {
        let source = Decoder::new(Cursor::new(self.data.clone()));
        source.unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum SoundKind {
    Trigger,
    CutItself,
    Hold,
    HoldRepeat,
    Toggle,
    ToggleRepeat,
}

impl Display for SoundKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SoundKind::Trigger => f.write_str("Trigger"),
            SoundKind::CutItself => f.write_str("Cut Itself"),
            SoundKind::Hold => f.write_str("Hold"),
            SoundKind::HoldRepeat => f.write_str("Hold (Repeating)"),
            SoundKind::Toggle => f.write_str("Toggle"),
            SoundKind::ToggleRepeat => f.write_str("Toggle (Repeating)"),
        }
    }
}
