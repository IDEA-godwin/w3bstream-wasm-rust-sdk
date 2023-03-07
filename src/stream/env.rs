use super::super::host::abi::*;

/// Retrieves the environment variables of the project by the key.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::stream::get_env;
/// let env = get_env("key").unwrap_or("default_value");
/// ```
pub fn get_env(key: &str) -> Option<String> {
    let data_ptr = &mut (0 as i32) as *const _ as *const *mut u8;
    let data_size = &mut (0 as i32) as *const i32;
    match unsafe { ws_get_env(key.as_bytes().as_ptr(), key.len() as _, data_ptr, data_size) } {
        0 => Some(unsafe { String::from_raw_parts(*data_ptr, *data_size as _, *data_size as _) }),
        _ => None,
    }
}
