use crate::{
    crypto::{cipher, decipher},
    error::{PasswordReadFailedReason, PasswordWriteFailedReason},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::error::PassyError;

#[derive(Serialize, Default, Deserialize)]
pub struct Metadata(HashMap<String, String>);
impl Metadata {
    pub fn parse(data: &str) -> Result<Metadata, ()> {
        let mut map = HashMap::new();

        for line in data.lines() {
            let mut parts = line.splitn(2, ':');
            let key = parts.next().ok_or(())?.trim().to_string();
            let value = parts.next().ok_or(())?.trim().to_string();

            map.insert(key, value);
        }

        Ok(Metadata(map))
    }

    pub fn stringify(&self) -> String {
        self.get()
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn get(&self) -> &HashMap<String, String> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Password {
    pub path: String,
    data: Metadata,
}

impl Password {
    pub fn new(
        key: &[u8; 32],
        appdata: &Path,
        username: &str,
        path: String,
        data: Metadata,
    ) -> Result<Self, PassyError> {
        let pwd = Password { path, data };
        pwd.write(key, appdata, username)?;
        Ok(pwd)
    }

    pub fn write(&self, key: &[u8; 32], appdata: &Path, username: &str) -> Result<(), PassyError> {
        let path = Password::get_path(appdata, username, &(self.path.to_string() + ".passy"));
        let data = Metadata::stringify(&self.data);
        let (mut content, nonce) = cipher(key, data.as_bytes()).map_err(|_| {
            PassyError::FailedToWritePassword(path.clone(), PasswordWriteFailedReason::CipherError)
        })?;

        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir).map_err(|_| {
                PassyError::FailedToCreateDir((parent_dir.to_owned(), "idk".to_string()))
            })?;
        }

        let mut data = nonce.to_vec();
        data.append(&mut content);

        fs::write(&path, data).map_err(|_| {
            PassyError::FailedToWritePassword(path, PasswordWriteFailedReason::WritePermission)
        })
    }

    pub fn get_pwdpath_from_ospath(user_path: &Path, path: &Path) -> String {
        let str1 = &path
            .to_string_lossy()
            .to_string()
            .replace(user_path.to_string_lossy().to_string().as_str(), "")[1..];

        str1[..str1.len() - 6].to_string()
    }
    pub fn read(key: &[u8; 32], user_path: &Path, path: &str) -> Result<Self, PassyError> {
        let path = user_path.join(path);

        let content = fs::read(&path).map_err(|_| {
            PassyError::FailedToReadPassword(
                path.clone().into(),
                PasswordReadFailedReason::FileNotFound,
            )
        })?;

        let nonce: [u8; 12] = content[..12].try_into().map_err(|_| {
            PassyError::FailedToReadPassword(
                path.clone(),
                PasswordReadFailedReason::MalformedMetadata,
            )
        })?;

        let data = &content[12..];

        let data = decipher(key, nonce, data).map_err(|_| {
            PassyError::FailedToReadPassword(path.clone(), PasswordReadFailedReason::DecipherError)
        })?;

        let text = std::str::from_utf8(&data).map_err(|_| {
            PassyError::FailedToReadPassword(
                path.clone(),
                PasswordReadFailedReason::MalformedMetadata,
            )
        })?;

        let map = Metadata::parse(text).map_err(|_| {
            PassyError::FailedToReadPassword(
                path.clone(),
                PasswordReadFailedReason::MalformedMetadata,
            )
        })?;

        Ok(Password {
            path: Password::get_pwdpath_from_ospath(user_path, &path),
            data: map,
        })
    }

    pub fn get_path(appdata: &Path, username: &str, path: &str) -> PathBuf {
        appdata.join(username).join(path)
    }

    pub fn delete(password_path: String, username: &str, appdata: &Path) -> Result<(), PassyError> {
        let path = appdata.join(username).join(password_path + ".passy");
        fs::remove_file(&path).map_err(|_| PassyError::DeletationFailed(path.clone()))?;

        // let parent_dir = path.parent().unwrap();

        // let entries = fs::read_dir(parent_dir)
        //     .ok()
        //     .and_then(|entires| Some(entires.count()))
        //     .unwrap_or(1);

        // if entries == 0 {
        //     fs::remove_dir(parent_dir)
        //         .map_err(|_| PassyError::DeletationFailed(parent_dir.to_owned()))?;
        // }

        Ok(())
    }
}
