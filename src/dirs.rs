use std::{fs::canonicalize, path::PathBuf};

use error_stack::{Result, ResultExt};
use crate::error::TmsError;

pub trait DirContainer {
    fn find_dir(&self, name: &str) -> Option<&PathBuf>;
}

impl DirContainer for Vec<PathBuf>{

fn find_dir(&self, name: &str) -> Option<&PathBuf>{

}
}

pub fn find_folders(manual_dirs: Option<Vec<String>>) -> Result<Option<Vec<PathBuf>>, TmsError> {
    match manual_dirs {
        Some(x) => Ok(Some(
            x.iter()
                .map(|path| canonicalize(path).change_context(TmsError::IoError))
                .collect::<Result<Vec<PathBuf>, TmsError>>()?,
        )),
        None => Ok(None),
    }
}
