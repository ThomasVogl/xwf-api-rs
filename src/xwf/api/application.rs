use std::io::Write;
use bitflags::Flags;
use crate::xwf::api::util::{buf_to_wchar_cstr, string_to_wchar_cstr};
use crate::xwf::xwf_types::{OutputMessageFlags, ProgressFlags};
use crate::xwf::raw_api::RAW_API;

pub struct Application {
    output_buffer: Vec<u8>
}
impl Application {

    pub fn new() -> Application {
        Application {
            output_buffer: Vec::new(),
        }
    }

    pub fn output(msg: &[u8], flags: OutputMessageFlags) {
        (RAW_API.lock().unwrap().as_ref().unwrap().output_message)(buf_to_wchar_cstr(msg) ,flags.bits())
    }

    pub fn output_string(msg: String, flags: OutputMessageFlags) {
        (RAW_API.lock().unwrap().as_ref().unwrap().output_message)(string_to_wchar_cstr(&msg) ,flags.bits())
    }

    pub fn show_progress(caption: String, flags: ProgressFlags) {
        (RAW_API.lock().unwrap().as_ref().unwrap().show_progress)(string_to_wchar_cstr(&caption), flags.bits())
    }

    pub fn set_progress_description(caption: String) {
        (RAW_API.lock().unwrap().as_ref().unwrap().set_progress_description)(string_to_wchar_cstr(&caption), )
    }

    pub fn should_stop() -> bool {
        (RAW_API.lock().unwrap().as_ref().unwrap().should_stop)() != 0
    }
    pub fn hide_progress() {
        (RAW_API.lock().unwrap().as_ref().unwrap().hide_progress)()
    }

    pub fn set_progress_percentage(num: u32, total: u32) {
        let mut percentage = 0;
        if total > 0 {
            percentage = (100.0f32 * num as f32 / total as f32).round() as u32;
        } else {
            percentage = 100;
        }
        (RAW_API.lock().unwrap().as_ref().unwrap().set_progress_percentage)(percentage);
    }

}

impl Write for Application {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {

        for i in buf {

            if *i == '\n' as u8 {
                Application::output(self.output_buffer.as_slice(), OutputMessageFlags::empty());
                self.output_buffer.clear();
            } else {
                self.output_buffer.push(*i);
            }
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.output_buffer.flush()
    }
}