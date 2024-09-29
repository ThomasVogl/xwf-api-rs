use std::ptr::null_mut;
use winapi::shared::ntdef::HANDLE;
use crate::traits::NativeHandle;

pub struct Window {
    window_handle: HANDLE,
}

impl Window {
    pub fn new(handle: HANDLE) -> Option<Window> {

        if handle == null_mut() {
            None
        } else {
            Some(Window {
                window_handle: handle
            })
        }
    }
}

impl NativeHandle for Window {
    fn get_handle(&self) -> HANDLE {
        self.window_handle
    }
}