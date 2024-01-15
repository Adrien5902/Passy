use std::{
    ffi::{OsStr, OsString},
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use passy_plugin_maker::export_function;
use serde::{self, Deserialize};
use serde_json;
use winreg::{
    enums::{RegType::REG_BINARY, HKEY_CURRENT_USER},
    RegKey, RegValue,
};

const GENSHIN_REG_PATH: &'static str = "Software\\miHoYo\\Genshin Impact";
const GENSHIN_TOKEN_KEYNAME: &'static str = "MIHOYOSDK_ADL_PROD_OVERSEA_h1158948810";

fn read_token(_: ()) -> Result<Vec<u8>, &'static str> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let genshin_reg = hkcu
        .open_subkey(GENSHIN_REG_PATH)
        .map_err(|_| "genshin doesnt seem to be installed")?;

    genshin_reg
        .get_raw_value(GENSHIN_TOKEN_KEYNAME)
        .map_err(|_| "login first before trying to get the token")
        .and_then(|d| Ok(d.bytes))
}

fn set_token(arg: &[u8]) -> Result<(), &'static str> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let genshin_reg = hkcu
        .open_subkey(GENSHIN_REG_PATH)
        .map_err(|_| "genshin doesnt seem to be installed")?;

    genshin_reg
        .set_raw_value(
            GENSHIN_TOKEN_KEYNAME,
            &RegValue {
                vtype: REG_BINARY,
                bytes: arg.to_owned(),
            },
        )
        .map_err(|_| "login first before trying to get the token")
}

fn on_load(plugin_path: &Path) {
    let data_path = plugin_path.join("data.json");
}

export_function!(on_load);
