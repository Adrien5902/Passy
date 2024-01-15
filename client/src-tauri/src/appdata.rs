use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
    sync::Mutex,
};

use crate::{error::PassyError, user::AppUser};

pub fn get_appdata_path() -> Result<PathBuf, PassyError> {
    let appdata_dir = dirs::config_dir().ok_or(PassyError::NoAppdataDir)?;

    let appdata = appdata_dir.join("Adrien5902/Passy");

    if !Path::exists(&appdata) {
        fs::create_dir_all(&appdata)
            .map_err(|e| PassyError::FailedToCreateDir((appdata.clone(), e.to_string())))?;
    };

    Ok(appdata)
}

pub fn read_appdata(appdata: PathBuf) -> Result<Mutex<Vec<AppUser>>, PassyError> {
    let users_dirs: Vec<_> = fs::read_dir(appdata.as_path())
        .map_err(|_| PassyError::FailedToReadDir(appdata.clone()))?
        .collect();

    let users = users_dirs
        .iter()
        .map(|dir| {
            dir.as_ref().and_then(|d| {
                let name = d.file_name().to_string_lossy().to_string();
                let path = d.path();

                Ok(AppUser { name, path })
            })
        })
        .collect::<Result<Vec<AppUser>, &Error>>()
        .map_err(|_| PassyError::FailedToReadDir(appdata.clone()))?;

    Ok(Mutex::new(users))
}
