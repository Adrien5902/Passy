// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod appdata;
mod crypto;
mod error;
mod password;
mod plugin;
mod user;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use appdata::{get_appdata_path, read_appdata};
use error::{PassyError, PluginErrorKind};
use password::{Metadata, Password};
use plugin::{AppState, JSResult, Plugin, PluginManifest, PluginPayload};
use serde::Serialize;
use tauri::{Manager, State};
use user::{AppUser, SerializableAppUser, Username};

type CurrentUserState = Mutex<Option<CurrentUser>>;

#[derive(Clone)]
struct CurrentUser {
    username: Username,
    key: [u8; 32],
}

#[derive(Serialize)]
struct AccountData {
    plugins: HashMap<String, PluginManifest>,
    appdata_path: String,
    passwords: Vec<Password>,
}

impl Into<SerializableAppUser> for &AppUser {
    fn into(self) -> SerializableAppUser {
        SerializableAppUser {
            name: self.name.clone(),
        }
    }
}

#[tauri::command]
fn get_users(users: State<Mutex<Vec<AppUser>>>) -> Result<Vec<SerializableAppUser>, String> {
    Ok(users.lock().unwrap().iter().map(|u| u.into()).collect())
}

#[tauri::command]
fn create_user(
    appdata: State<PathBuf>,
    users_state: State<Mutex<Vec<AppUser>>>,
    name: String,
) -> Result<(), String> {
    let mut users = users_state.lock().unwrap();
    let user = AppUser::create(&appdata, name)?;
    users.push(user);
    Ok(())
}

fn check_login(current_user_state: State<CurrentUserState>) -> Result<CurrentUser, PassyError> {
    Ok(current_user_state
        .lock()
        .unwrap()
        .as_ref()
        .ok_or(PassyError::NotLoggedIn)?
        .clone())
}

fn recursive_pwd_read(
    key: &[u8; 32],
    user_path: &Path,
    path: String,
) -> Result<Vec<Password>, PassyError> {
    let curr_path = user_path.join(path.clone());
    fs::read_dir(&curr_path)
        .map_err(|_| PassyError::FailedToReadDir(curr_path.to_owned()))?
        .into_iter()
        .filter_map(|res| {
            res.ok().and_then(|entry| {
                if entry.path().is_dir() {
                    Some(recursive_pwd_read(
                        key,
                        user_path,
                        path.clone()
                            + entry.file_name().to_string_lossy().to_string().as_str()
                            + "/",
                    ))
                } else {
                    let lossy_filename = entry.file_name().to_string_lossy().to_string();

                    if lossy_filename.ends_with(".passy") {
                        Some(
                            Password::read(key, user_path, &(path.clone() + &lossy_filename))
                                .and_then(|pwd| Ok(vec![pwd])),
                        )
                    } else {
                        None
                    }
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .and_then(|r| Ok(r.into_iter().flat_map(|v| v).collect()))
}

#[tauri::command]
fn get_user_data(
    appdata: State<PathBuf>,
    plugins_state: State<Arc<Mutex<HashMap<String, Plugin>>>>,
    current_user_state: State<CurrentUserState>,
) -> Result<AccountData, String> {
    let current_user = check_login(current_user_state)?;
    let username = current_user.username;

    let user_path = appdata.join(username.clone());

    if !Path::exists(&user_path) {
        return Err(PassyError::UserNotFound(username.clone()).into());
    }

    let key = &current_user.key;

    let passwords = recursive_pwd_read(key, &user_path, "".to_string())?;

    let plugins = Plugin::init_loader(&appdata)?;

    let mut plugins_manifests = HashMap::new();
    plugins.keys().for_each(|k| {
        plugins_manifests.insert(
            k.clone(),
            plugins.get(k).unwrap().resolve_manifest(&appdata),
        );
    });

    *plugins_state.lock().unwrap() = plugins;

    Ok(AccountData {
        plugins: plugins_manifests,
        passwords,
        appdata_path: user_path.to_string_lossy().to_string(),
    })
}

#[tauri::command]
fn create_password(
    appdata: State<PathBuf>,
    users_state: State<Mutex<Vec<AppUser>>>,
    current_user_state: State<CurrentUserState>,
    path: String,
) -> Result<Password, String> {
    let users = users_state.lock().unwrap();

    let current_user = check_login(current_user_state)?;
    let key = &current_user.key;
    let username = current_user.username.as_str();

    let password = Password::new(key, &appdata, username, path, Metadata::default())?;
    Ok(password)
}

#[tauri::command]
fn login(
    username: String,
    current_user_state: State<CurrentUserState>,
    users_state: State<Mutex<Vec<AppUser>>>,
    password: String,
) -> Result<(), String> {
    let users = users_state.lock().unwrap();

    let user = users
        .iter()
        .find(|u| u.name == username)
        .ok_or(PassyError::UserNotFound(username))?;

    let mut current_user = current_user_state.lock().unwrap();

    *current_user = Some(CurrentUser {
        username: user.name.clone(),
        key: [0; 32],
    });

    Ok(())
}

#[tauri::command]
fn update_password(
    current_user_state: State<CurrentUserState>,
    appdata: State<PathBuf>,
    password: Password,
) -> Result<(), String> {
    let current_user = check_login(current_user_state)?;
    password.write(&current_user.key, &appdata, &current_user.username)?;
    Ok(())
}

#[tauri::command]
fn delete_password(
    current_user_state: State<CurrentUserState>,
    appdata: State<PathBuf>,
    password_path: String,
) -> Result<(), String> {
    let current_user = check_login(current_user_state)?;
    Password::delete(password_path, &current_user.username, &appdata)?;
    Ok(())
}

fn main() {
    let appdata_path = get_appdata_path().unwrap();
    let users = read_appdata(appdata_path.clone()).unwrap();

    tauri::Builder::default()
        .manage(appdata_path.clone())
        .manage(users)
        .manage::<CurrentUserState>(Mutex::new(None))
        .manage::<Arc<Mutex<HashMap<String, Plugin>>>>(Arc::new(Mutex::new(HashMap::new())))
        .invoke_handler(tauri::generate_handler![
            get_users,
            create_user,
            get_user_data,
            create_password,
            login,
            update_password,
            delete_password,
        ])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            let plugins_ref = Arc::clone(&app.state::<Arc<Mutex<HashMap<String, Plugin>>>>());

            app.listen_global("plugin", move |event| {
                let res: Result<_, String> = (|| {
                    let ser_err = PassyError::PluginError(
                        "unknown".to_string(),
                        PluginErrorKind::FailedToDeserializeData,
                    );

                    let payload_str = event.payload().ok_or(ser_err.clone())?;

                    let payload = serde_json::from_str::<PluginPayload>(payload_str)
                        .map_err(|_| ser_err.clone())?;

                    let plugins = plugins_ref.lock().unwrap();

                    let p = plugins.get(&payload.plugin).ok_or(PassyError::PluginError(
                        payload.plugin.clone(),
                        PluginErrorKind::PluginNotFound,
                    ))?;

                    let states = payload
                        .states
                        .iter()
                        .map(|state| match state {
                            AppState::AppdataPath => {
                                appdata_path.clone().to_string_lossy().to_string()
                            }
                        })
                        .collect::<Vec<_>>();

                    p.invoke(&payload.command, &payload.data, &states)
                        .map_err(|e| String::from(e))
                })();

                let _ = main_window.emit("plugin_res", JSResult::from(res));
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
