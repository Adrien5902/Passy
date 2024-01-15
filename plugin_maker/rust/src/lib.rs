use std::ffi::{c_char, CStr, CString};

use serde_json::Error;

#[macro_export]
macro_rules! export_function {
    ($($fn_name:ident), +) => {
        use paste::paste;
        use std::ffi::c_char;
        use passy_plugin_maker::{data_to_pointer, pointer_to_data};

        $(
            paste!{
                #[no_mangle]

                pub extern "C" fn [<$fn_name _external>](arg: *const c_char) -> *const c_char{
                    let res = pointer_to_data(arg).and_then(|d|  Ok($fn_name(d)));
                    data_to_pointer(res)
                }
            }
        )*
    };
}

pub fn data_to_pointer<T: serde::Serialize>(d: Result<T, Error>) -> *const c_char {
    let data = match d {
        Ok(ok) => serde_json::to_string(&ok).unwrap(),
        Err(e) => format!("err:{:?}", e),
    };

    let cstring =
        Box::new(CString::new(data).expect("returned string may not contain null bytes (\\0)"));

    Box::into_raw(cstring) as *const c_char
}

pub fn pointer_to_data<'a, T: serde::Deserialize<'a>>(ptr: *const c_char) -> Result<T, Error> {
    let s = unsafe { CStr::from_ptr(ptr).to_str().unwrap() };
    serde_json::from_str(s)
}
