use std::{
    collections::HashMap,
    ffi::{c_char, CString},
    fmt::Debug,
    fs,
    panic::catch_unwind,
    path::Path,
};

use image_base64;
use libloading::Library;
use serde::{Deserialize, Serialize};

use crate::error::{PassyError, PluginErrorKind, PluginFailedToInvokeFunctionReason};

pub struct Plugin {
    id: String,
    pub manifest: PluginManifestResolvable,
    lib: Library,
}

impl Plugin {
    pub fn load(plugin_path: &Path) -> Result<Self, PassyError> {
        let id = plugin_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        fn to_plugin_load_error<T, E: Debug>(
            res: Result<T, E>,
            id: String,
            reason: &str,
        ) -> Result<T, PassyError> {
            res.map_err(|e| {
                PassyError::PluginError(
                    id,
                    PluginErrorKind::FailedToLoad(format!("{} {:?}", reason, e)),
                )
            })
        }

        let manifest_path = plugin_path.join("manifest.json");
        let manifest_data = to_plugin_load_error(
            fs::read(manifest_path),
            id.clone(),
            "failed to read manifest file",
        )?;
        let manifest: PluginManifestResolvable = to_plugin_load_error(
            serde_json::from_slice(&manifest_data),
            id.clone(),
            "failed to parse manifest",
        )?;

        unsafe {
            let lib = to_plugin_load_error(
                Library::new(
                    plugin_path.join(&manifest.back.clone().unwrap_or("back.dll".to_string())),
                ),
                id.clone(),
                "failed to import lib",
            )?;

            let plugin = Plugin {
                id: id.clone(),
                manifest,
                lib,
            };

            plugin
                .invoke("on_load", &id, &[plugin_path.to_string_lossy().to_string()])
                .ok();

            Ok(plugin)
        }
    }

    pub fn invoke(
        &self,
        function: &str,
        data: &str,
        states: &[String],
    ) -> Result<String, PassyError> {
        unsafe {
            let f = self
                .lib
                .get::<fn(*const c_char, *const c_char) -> *const c_char>(
                    (function.to_owned() + "_external").as_bytes(),
                )
                .map_err(|_| {
                    PassyError::PluginError(
                        self.id.to_string(),
                        PluginErrorKind::FailedToInvokeFunction(
                            function.to_string(),
                            PluginFailedToInvokeFunctionReason::NotFound,
                        ),
                    )
                })?;

            let c_data = CString::new(data).unwrap();
            let states_str = serde_json::to_string(&states).unwrap();
            let c_states = CString::new(states_str).unwrap();

            let ptr = f(c_data.as_ptr(), c_states.as_ptr());
            let cstring = Box::from_raw(ptr as *mut CString); // Convert raw pointer back to Box
            let result_string = cstring
                .into_string()
                .expect("Failed to convert CString to String");

            if result_string.starts_with("err:") {
                Err(PassyError::PluginError(
                    self.id.clone(),
                    PluginErrorKind::FailedToInvokeFunction(
                        function.to_owned(),
                        PluginFailedToInvokeFunctionReason::FailedToSerializeReturnValue(
                            result_string[4..].to_string(),
                        ),
                    ),
                ))
            } else {
                Ok(result_string)
            }
        }
    }

    pub fn init_loader(appdata_path: &Path) -> Result<HashMap<String, Self>, PassyError> {
        let mut plugins = HashMap::new();

        let path = appdata_path.join("plugins");
        if !Path::exists(&path) {
            fs::create_dir_all(&path)
                .map_err(|_| PassyError::FailedToCreateDir((path, "idk".to_string())))?;
            return Ok(plugins);
        }

        let entries = fs::read_dir(&path).map_err(|_| PassyError::FailedToReadDir(path.clone()))?;
        for res in entries {
            let dir = res.map_err(|_| PassyError::FailedToReadDir(path.clone()))?;
            let plugin = Self::load(&dir.path())?;
            plugins.insert(plugin.id.clone(), plugin);
        }

        Ok(plugins)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginManifestResolvable {
    name: String,
    author: String,
    icon: Option<String>,
    back: Option<String>,
}

impl Plugin {
    pub fn resolve_manifest(&self, appdata: &Path) -> PluginManifest {
        PluginManifest {
            author: self.manifest.author.clone(),
            name: self.manifest.name.clone(),
            icon: self.manifest.icon.clone().and_then(|icon| {
                catch_unwind(|| {
                    image_base64::to_base64(
                        appdata
                            .join("plugins")
                            .join(self.id.clone())
                            .join(icon)
                            .to_string_lossy()
                            .to_string()
                            .as_str(),
                    )
                })
                .ok()
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PluginManifest {
    name: String,
    author: String,
    icon: Option<String>,
}
#[derive(Serialize, Clone)]
pub struct JSResult<T, E> {
    data: Option<T>,
    err: Option<E>,
}

impl<T, E> From<Result<T, E>> for JSResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => JSResult {
                data: Some(v),
                err: None,
            },
            Err(e) => JSResult {
                data: None,
                err: Some(e),
            },
        }
    }
}

#[derive(Deserialize)]
pub struct PluginPayload {
    pub plugin: String,
    pub command: String,
    pub data: String,
    pub states: Vec<AppState>,
}

#[derive(Deserialize)]
pub enum AppState {
    AppdataPath,
}
