use libc::c_char;
use context::StatusCode;
use std::ffi::{CStr, CString};

pub fn catch_error_code (value : i32) -> StatusCode {
    match value {
        0 | 10 => StatusCode::Success,
        x @ _ if x < 0 => StatusCode::Failure,
        _ => unreachable! (),
    }
}

pub unsafe fn const_c_to_string (ptr : *const c_char) -> String {
    String::from_utf8 (CStr::from_ptr (ptr).to_bytes ().to_vec ()).unwrap ()
}

pub fn str_to_const_c (value : &str) -> *const c_char {
    let new_string = value.clone();
    CString::new (new_string).unwrap ().as_ptr ()
}

#[macro_escape]
macro_rules! switch_freenect_device_flags {
    ($result:expr; $motor:block, $camera:block, $audio:block) => {
        if $result & FreenectDeviceFlags::MOTOR as u8 == FreenectDeviceFlags::MOTOR as u8
            $motor
        if $result & FreenectDeviceFlags::CAMERA as u8 == FreenectDeviceFlags::CAMERA as u8
            $camera
        if $result & FreenectDeviceFlags::AUDIO as u8 == FreenectDeviceFlags::AUDIO as u8
            $audio
    }
}

#[macro_escape]
macro_rules! catch_error {
    ($result:ident; $($pat:pat => $value:expr),*) => {
        match $result {
            $(
                $pat => $value,
            ),*
            x @ _ if x < 0 => None,
            _ => unreachable! (),
        }
    };

    ($result:ident; $($pat:pat if $cond:expr => $value:expr),*) => {
        match $result {
            $(
                $pat if $cond => $value,
            ),*
            x @ _ if x < 0 => None,
            _ => unreachable! (),
        }
    };
}
