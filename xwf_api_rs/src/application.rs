use std::ptr::null_mut;
use winapi::shared::ntdef::LPWSTR;
use crate::get_raw_api;
use crate::xwf_types::*;
use crate::raw_api::RAW_API;

use crate::error::XwfError;


pub struct Application {

}
impl Application {

    pub fn new() -> Application {
        Application {
        }
    }

    pub fn output(msg: &[u8], flags: OutputMessageFlags) {
        let s = winsafe::WString::from_str(String::from_utf8_lossy(msg));
        (get_raw_api!().output_message)(s.as_ptr() ,flags.bits())
    }

    pub fn output_string<S: AsRef<str>>(msg: S, flags: OutputMessageFlags) {
        (get_raw_api!().output_message)(winsafe::WString::from_str(&msg).as_ptr(), flags.bits())
    }

    pub fn log<S: AsRef<str>>(msg: S) {
        Self::output_string(msg, OutputMessageFlags::empty());
    }

    pub fn get_user_input_integer<S: AsRef<str>>(msg: S)-> Option<u64> {
        let ret = (get_raw_api!().get_user_input)(unsafe { winsafe::WString::from_str(&msg).as_mut_ptr() }, null_mut(), 0, 0x1);
        if ret < 0 {
            None
        } else {
            Some(ret as u64)
        }
    }

    pub fn get_user_input_str<S: AsRef<str>>(msg: S, allow_empty: bool) -> Option<String> {
        let flags = if allow_empty {0x2} else {0x0};
        let mut s = winsafe::WString::new_alloc_buf(65535);
        let ret: i64 =  (get_raw_api!().get_user_input)(winsafe::WString::from_str(msg).as_ptr() as LPWSTR , unsafe {s.as_mut_ptr()},  s.buf_len() as u32, flags);
        if ret > 0 {
            Some(s.to_string())
        } else {
            None
        }        
    }

    pub fn show_progress<S: AsRef<str>>(caption: S, flags: ProgressFlags) {
        (get_raw_api!().show_progress)(winsafe::WString::from_str(&caption).as_ptr() as LPWSTR, flags.bits())
    }

    pub fn set_progress_description<S: AsRef<str>>(caption: S) {
        (get_raw_api!().set_progress_description)(winsafe::WString::from_str(&caption).as_ptr() as LPWSTR)
    }

    pub fn should_stop() -> Result<(), XwfError> {
        if (get_raw_api!().should_stop)() != 0 {
            Err(XwfError::OperationAbortedByUser)
        } else {
            Ok(())
        }
    }
    pub fn hide_progress() {
        (get_raw_api!().hide_progress)()
    }

    pub fn set_progress_percentage(num: u32, total: u32) {
        let percentage:u32;
        if total > 0 {
            percentage = (100.0f32 * num as f32 / total as f32).round() as u32;
        } else {
            percentage = 100;
        }
        (get_raw_api!().set_progress_percentage)(percentage);
    }

}


#[macro_export]
macro_rules! xwfinfo {
    
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        
        $crate::application::Application::output_string(format!("{} [INFO]: {}", $crate::chrono::offset::Local::now().format("%H:%M:%S"), res), $crate::xwf_types::OutputMessageFlags::empty())
    }}
}

#[macro_export]
macro_rules! xwfwarn {
    
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        
        $crate::application::Application::output_string(format!("{} [WARN]: {}", $crate::chrono::offset::Local::now().format("%H:%M:%S"), res), $crate::xwf_types::OutputMessageFlags::empty())
    }}
}

#[macro_export]
macro_rules! xwferror {
    
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        
        $crate::application::Application::output_string(format!("{} [ERROR]: {}", $crate::chrono::offset::Local::now().format("%H:%M:%S"), res), $crate::xwf_types::OutputMessageFlags::empty())
    }}
}


#[macro_export]
#[cfg(feature = "debug_output")]
macro_rules! xwfdebug {
    
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        
        $crate::application::Application::output_string(format!("{} [DEBUG]: {}", $crate::chrono::offset::Local::now().format("%H:%M:%S"), res), $crate::xwf_types::OutputMessageFlags::empty())
    }}
}


#[macro_export]
#[cfg(not(feature = "debug_output"))]
macro_rules! xwfdebug {
    
    ($($arg:tt)*) => {{
    }}
}