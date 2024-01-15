use std::{error::Error, path::PathBuf};

use crate::user::Username;

#[derive(Debug, Clone)]
pub enum PasswordReadFailedReason {
    FileNotFound,
    DecipherError,
    MalformedMetadata,
}

#[derive(Debug, Clone)]
pub enum PasswordWriteFailedReason {
    WritePermission,
    CipherError,
}

#[derive(Debug, Clone)]
pub enum PluginFailedToInvokeFunctionReason {
    FailedToSerializeReturnValue(String),
    NotFound,
}

impl PluginFailedToInvokeFunctionReason {
    fn to_string(&self) -> String {
        match self {
            Self::FailedToSerializeReturnValue(error) => {
                format!("couldn't serialize return value, error details: {error}")
            }
            Self::NotFound => "command not found".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PluginErrorKind {
    PluginNotFound,
    FailedToLoad(String),
    FailedToInvokeFunction(String, PluginFailedToInvokeFunctionReason),
    FailedToDeserializeData,
}

impl PluginErrorKind {
    fn to_string(self) -> String {
        match self {
            Self::PluginNotFound => format!("plugin not found"),
            Self::FailedToInvokeFunction(name, reason) => {
                format!("Failed to invoke function {}, {}", name, reason.to_string())
            }
            Self::FailedToLoad(reason) => format!("failed to load plugin, reason : {}", reason),
            Self::FailedToDeserializeData => "failed to deserialize data".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PassyError {
    NoAppdataDir,
    UserNotFound(Username),
    UserAlreadyExists(Username),
    FailedToCreateDir((PathBuf, String)),
    FailedToReadDir(PathBuf),
    FailedToReadPassword(PathBuf, PasswordReadFailedReason),
    FailedToWritePassword(PathBuf, PasswordWriteFailedReason),
    DeletationFailed(PathBuf),
    PluginError(String, PluginErrorKind),
    NotLoggedIn,
    UnknowError(&'static dyn Error),
}

impl From<PassyError> for String {
    fn from(value: PassyError) -> String {
        match value {
            PassyError::FailedToCreateDir((path, reason)) => {
                format!(
                    "Failed to create directory at path {}, reason : {}",
                    path.to_string_lossy().to_string(),
                    reason
                )
            }
            PassyError::FailedToReadDir(path) => format!(
                "Failed to read directory at {}",
                path.to_string_lossy().to_string()
            ),
            PassyError::FailedToReadPassword(path, reason) => {
                format!(
                    "Failed to read password at {}, {}",
                    path.to_string_lossy().to_string(),
                    match reason {
                        PasswordReadFailedReason::DecipherError =>
                            "failed to decipher password data",
                        PasswordReadFailedReason::FileNotFound => "file not found",
                        PasswordReadFailedReason::MalformedMetadata => "malformed metadata in file",
                    }
                )
            }
            PassyError::FailedToWritePassword(path, reason) => format!(
                "Failed to write password at path {}, {}",
                path.to_string_lossy().to_string(),
                match reason {
                    PasswordWriteFailedReason::CipherError => "couldn't cipher password",
                    PasswordWriteFailedReason::WritePermission =>
                        "can't write file here, maybe missing permissions",
                }
            ),
            PassyError::DeletationFailed(path) => format!(
                "Failed to delete password at {}",
                path.to_string_lossy().to_string()
            ),
            PassyError::PluginError(name, kind) => {
                format!("Error happened in plugin {}, {}", name, kind.to_string())
            }
            PassyError::NotLoggedIn => "Not logged in".to_string(),
            PassyError::NoAppdataDir => "Can't find appdata dir".to_string(),
            PassyError::UserNotFound(user) => format!("User {} not found", user),
            PassyError::UserAlreadyExists(user) => format!("User {} already exists", user),
            PassyError::UnknowError(e) => e.to_string(),
        }
    }
}

impl From<&'static dyn Error> for PassyError {
    fn from(value: &'static dyn Error) -> PassyError {
        PassyError::UnknowError(value)
    }
}
