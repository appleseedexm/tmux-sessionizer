use std::{fs::canonicalize, path::PathBuf};

use crate::error::TmsError;
use error_stack::{Result, ResultExt};

pub trait DirContainer {
    fn find_dir(&self, name: &str) -> Option<&PathBuf>;
    fn as_strings(&self) -> Vec<String>;
}

impl DirContainer for Vec<PathBuf> {
    fn find_dir(&self, name: &str) -> Option<&PathBuf> {
        self.iter()
            .find(|&path| path.as_os_str().to_str().unwrap() == name)
    }

    fn as_strings(&self) -> Vec<String> {
        self.iter()
            .map(|path| path.as_os_str().to_str().unwrap().to_string())
            .collect()
    }
}

pub fn manual_dirs(manual_dirs: Option<Vec<String>>) -> Result<Option<Vec<PathBuf>>, TmsError> {
    match manual_dirs {
        Some(x) => Ok(Some(
            x.iter()
            // todo: check error type
                .map(|path| canonicalize(path).change_context(TmsError::IoError))
                .collect::<Result<Vec<PathBuf>, TmsError>>()?,
        )),
        None => Ok(None),
    }
}
