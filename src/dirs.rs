use std::{fs::canonicalize, path::PathBuf};

use crate::error::TmsError;
use error_stack::{Result, ResultExt};

pub trait DirContainer {
    fn find_dir(&self, name: &str) -> Option<&PathBuf>;
}

impl DirContainer for Vec<PathBuf> {
    fn find_dir(&self, name: &str) -> Option<&PathBuf> {
        self.iter()
            .find(|&&path| path.as_os_str().to_str().unwrap() == name)
    }
}

pub fn manual_dirs(manual_dirs: Option<Vec<String>>) -> Result<Option<Vec<PathBuf>>, TmsError> {
    match manual_dirs {
        Some(x) => Ok(Some(
            x.iter()
                .map(|path| canonicalize(path).change_context(TmsError::IoError))
                .collect::<Result<Vec<PathBuf>, TmsError>>()?,
        )),
        None => Ok(None),
    }
}
