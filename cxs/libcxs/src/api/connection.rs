extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use std::ptr;
use api::CxsStateType;
use std::thread;
use std::time::Duration;
use connection::{build_connection, connect, to_string, get_state, release, is_valid_connection_handle};

/**
 * connection object
 */

#[no_mangle]
#[allow(unused_assignments)]
pub extern fn cxs_connection_create(command_handle: u32,
                                    source_id: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, claim_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);

    thread::spawn(move|| {
        let handle = build_connection(source_id);
        let mut rc = error::UNKNOWN_ERROR.code_num;

        loop {
            if get_state(handle) == CxsStateType::CxsStateInitialized as u32 {
                rc = error::SUCCESS.code_num;
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }

        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_connect(command_handle:u32,
                                     connection_handle: u32,
                                     connection_options: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    if !is_valid_connection_handle(connection_handle) {
        return error::INVALID_CONNECTION_HANDLE.code_num;
    }

    let options = if !connection_options.is_null() {
        check_useful_c_str!(connection_options, error::UNKNOWN_ERROR.code_num);
        connection_options.to_owned()
    }
    else {
        "".to_string()
    };

    thread::spawn(move|| {
        let rc = connect(connection_handle, options);

        cb(command_handle,rc);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_get_data(connection_handle: u32) -> *mut c_char {
    let json_string = to_string(connection_handle);

    if json_string.is_empty() {
        return ptr::null_mut()
    }
    else {
        let msg = CStringUtils::string_to_cstring(json_string);

        msg.into_raw()
    }
}

#[no_mangle]
pub extern fn cxs_connection_get_state(connection_handle: u32, status: *mut u32) -> u32 {

    if status.is_null() {return error::UNKNOWN_ERROR.code_num}

    let state = get_state(connection_handle);

    unsafe { *status = state }

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn cxs_connection_release(connection_handle: u32) -> u32 {
    release(connection_handle)
}

#[cfg(test)]
mod tests {
    extern crate mockito;

    use super::*;
    use settings;
    use std::ffi::CString;
    use std::ptr;
    use utils::error;
    use std::thread;
    use std::time::Duration;
    use api::CxsStateType;

    extern "C" fn create_cb(command_handle: u32, err: u32, connection_handle: u32) {
        if err != 0 {panic!("create_cb failed")}
        if connection_handle == 0 {panic!("invalid handle")}
        println!("successfully called create_cb")
    }

    #[test]
    fn test_cxs_connection_create() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = cxs_connection_create(0,
                                       CString::new("test_create").unwrap().into_raw(),
                                       Some(create_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_cxs_connection_create_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let rc = cxs_connection_create(0,
                                       CString::new("test_create_fails").unwrap().into_raw(),
                                       None);
        assert_eq!(rc, error::INVALID_OPTION.code_num);

        let rc = cxs_connection_create(0,
                                       ptr::null(),
                                       Some(create_cb));
        assert_eq!(rc, error::INVALID_OPTION.code_num);
    }

    extern "C" fn connect_cb(command_handle: u32, err: u32) {
        assert_eq!(err, 0);
        println!("successfully called connect_cb");
    }

    #[test]
    fn test_cxs_connection_connect() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let handle = build_connection("test_cxs_connection_connect".to_owned());
        assert!(handle > 0);
        thread::sleep(Duration::from_millis(500));
        let rc = cxs_connection_connect(0,handle, CString::new("{}").unwrap().into_raw(),Some(connect_cb));
        assert_eq!(rc, error::SUCCESS.code_num);
    }

    #[test]
    fn test_cxs_connection_connect_fails() {
        let rc = cxs_connection_connect(0,0, CString::new("{}").unwrap().into_raw(),Some(connect_cb));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    #[test]
    fn test_cxs_connection_get_state() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxs_connection_get_state".to_owned());
        assert!(handle > 0);

        let mut state: u32 = 0;
        let rc = cxs_connection_get_state(handle, &mut state);
        assert_eq!(rc, error::SUCCESS.code_num);
        assert_eq!(state,CxsStateType::CxsStateNone as u32);
    }

    #[test]
    fn test_cxs_connection_get_state_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxs_connection_get_state_fails".to_owned());
        assert!(handle > 0);

        let rc = cxs_connection_get_state(handle, ptr::null_mut());
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);

        let rc = cxs_connection_get_state(0, ptr::null_mut());
        assert_eq!(rc, error::UNKNOWN_ERROR.code_num);
    }

    #[test]
    #[allow(unused_assignments)]
    fn test_cxs_connection_get_data() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxs_connection_get_data".to_owned());
        assert!(handle > 0);

        let data = cxs_connection_get_data(handle);
        let mut final_string = String::new();

        unsafe {
            let c_string = CString::from_raw(data);
            final_string = c_string.into_string().unwrap();
        }

        assert!(final_string.len() > 0);
    }

    #[test]
    fn test_cxs_connection_get_data_fails() {
        let data = cxs_connection_get_data(0);

        assert_eq!(data, ptr::null_mut());
    }

    #[test]
    fn test_cxs_connection_release() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");
        let handle = build_connection("test_cxs_connection_release".to_owned());
        assert!(handle > 0);

        let rc = cxs_connection_release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        let rc = cxs_connection_connect(0,handle, CString::new("{}").unwrap().into_raw(),Some(connect_cb));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }
}
