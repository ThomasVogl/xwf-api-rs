use std::ffi::{OsString};
use std::os::windows::prelude::*;
use winapi::shared::ntdef::LPWSTR;

pub fn string_to_wchar_cstr(str: &String) -> *mut u16 {
    let mut v: Vec<u16> = str.encode_utf16().collect();
    v.push(0u16);
    v.as_mut_ptr()
}

pub fn buf_to_wchar_cstr(buf: &[u8]) -> *mut u16 {
    let msg = String::from_utf8_lossy(buf);
    let mut v: Vec<u16> = msg.encode_utf16().collect();
    v.push(0u16);
    v.push(0u16);
    v.as_mut_ptr()
}

pub fn wchar_str_to_string(bytes: &[u16]) -> String {
    let mut iter = bytes.split(|c| *c == 0u16);
    match iter.next() {
        Some(slice) => OsString::from_wide(slice).into_string().unwrap(),
        None => OsString::from_wide(bytes).into_string().unwrap(),
    }
}


pub fn wchar_ptr_to_string(mut ptr: LPWSTR) -> String {
    let mut vec_u16: Vec<u16> = Vec::new();

    unsafe {
        let mut chr = *ptr;

        while chr != 0 {
            vec_u16.push(chr);
            ptr = ptr.add(1);
            chr = *ptr;
        }
    }
    wchar_str_to_string(&vec_u16)
}