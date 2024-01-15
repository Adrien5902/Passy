use crate::error::PassyError;
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub type Username = String;

#[derive(Serialize)]
pub struct SerializableAppUser {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct AppUser {
    pub name: Username,
    pub path: PathBuf,
}

impl AppUser {
    pub fn create(appdata: &Path, name: String) -> Result<AppUser, PassyError> {
        let folder_name = Username::from(&name);
        let path = appdata.join(&folder_name);
        fs::create_dir(path.clone()).map_err(|_| PassyError::UserAlreadyExists(name.clone()))?;

        Ok(AppUser {
            name: folder_name,
            path,
        })
    }
}
