use std::ptr::null_mut;

use crate::get_raw_api;
use crate::xwf::api::util::{buf_to_wchar_cstr, string_to_wchar_cstr};
use crate::xwf::xwf_types::{OutputMessageFlags, ProgressFlags};
use crate::xwf::raw_api::RAW_API;

use super::error::XwfError;
use super::util::wchar_ptr_to_string;

pub struct Application {

}
impl Application {

    pub fn new() -> Application {
        Application {
        }
    }

    pub fn output(msg: &[u8], flags: OutputMessageFlags) {
        (get_raw_api!().output_message)(buf_to_wchar_cstr(msg) ,flags.bits())
    }

    pub fn output_string(msg: String, flags: OutputMessageFlags) {
        (get_raw_api!().output_message)(string_to_wchar_cstr(&msg) ,flags.bits())
    }

    pub fn log(msg: String) {
        (get_raw_api!().output_message)(string_to_wchar_cstr(&msg) , OutputMessageFlags::empty().bits());
    }

    pub fn get_user_input_integer(msg: String) -> Option<u64> {
        let ret = (get_raw_api!().get_user_input)(string_to_wchar_cstr(&msg) , null_mut(),  0, 0x1);
        if ret < 0 {
            None
        } else {
            Some(ret as u64)
        }
    }

    pub fn get_user_input_str(msg: String, allow_empty: bool) -> Option<String> {
        let flags = if allow_empty {0x2} else {0x0};
        let mut w_buf = [0u16;65535];
        let ret: i64 =  (get_raw_api!().get_user_input)(string_to_wchar_cstr(&msg) , w_buf.as_mut_ptr(),  w_buf.len() as u32, flags);
        if ret > 0 {
            Some(wchar_ptr_to_string(w_buf.as_mut_ptr()))
        } else {
            None
        }        
    }

    pub fn show_progress(caption: String, flags: ProgressFlags) {
        (get_raw_api!().show_progress)(string_to_wchar_cstr(&caption), flags.bits())
    }

    pub fn set_progress_description(caption: String) {
        (get_raw_api!().set_progress_description)(string_to_wchar_cstr(&caption), )
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
        
        Application::output_string(format!("{} [INFO]: {}", chrono::offset::Local::now().format("%H:%M:%S"), res), $crate::xwf::xwf_types::OutputMessageFlags::empty())
    }}
}

#[macro_export]
macro_rules! xwfwarn {
    
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        
        Application::output_string(format!("{} [WARN]: {}", chrono::offset::Local::now().format("%H:%M:%S"), res), $crate::xwf::xwf_types::OutputMessageFlags::empty())
    }}
}

#[macro_export]
macro_rules! xwferror {
    
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        
        Application::output_string(format!("{} [ERROR]: {}", chrono::offset::Local::now().format("%H:%M:%S"), res), $crate::xwf::xwf_types::OutputMessageFlags::empty())
    }}
}


#[macro_export]
#[cfg(feature = "debug_output")]
macro_rules! xwfdebug {
    
    ($($arg:tt)*) => {{
        let res = std::fmt::format(format_args!($($arg)*));
        
        Application::output_string(format!("{} [DEBUG]: {}", chrono::offset::Local::now().format("%H:%M:%S"), res), $crate::xwf::xwf_types::OutputMessageFlags::empty())
    }}
}


#[macro_export]
#[cfg(not(feature = "debug_output"))]
macro_rules! xwfdebug {
    
    ($($arg:tt)*) => {{
    }}
}