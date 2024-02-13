use std::{io, path::PathBuf};

use egui_notify::Toasts;
use ron::error::SpannedError;

pub enum HibikiError {
    DoesNotExist(PathBuf),
    NotAFile(PathBuf),
    InternalError(io::Error),
    BrokenScene(SpannedError),
    SceneSerialize(ron::Error),
}

pub trait ToastyError<T> {
    fn handle_toasty(self, toasts: &mut Toasts) -> Option<T>;
}

impl<T> ToastyError<T> for Result<T, HibikiError> {
    fn handle_toasty(self, toasts: &mut Toasts) -> Option<T> {
        match self {
            Ok(val) => Some(val),
            Err(HibikiError::DoesNotExist(path)) => {
                toasts.error(format!("File at '{path:?}' does not exist."));
                None
            }
            Err(HibikiError::NotAFile(path)) => {
                toasts.error(format!("'{path:?}' is not a file."));
                None
            }
            Err(HibikiError::InternalError(err)) => {
                toasts.error(format!("Internal error occured: {err:?}"));
                None
            }
            Err(HibikiError::BrokenScene(err)) => {
                toasts.error(format!("Broken scene file: {err:?}"));
                None
            }
            Err(HibikiError::SceneSerialize(err)) => {
                toasts.error(format!("Couldn't save scene: {err:?}"));
                None
            }
        }
    }
}
