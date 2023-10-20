use std::io::Write;
use bitflags::Flags;
use crate::get_raw_api;
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
        (get_raw_api!().output_message)(buf_to_wchar_cstr(msg) ,flags.bits())
    }

    pub fn output_string(msg: String, flags: OutputMessageFlags) {
        (get_raw_api!().output_message)(string_to_wchar_cstr(&msg) ,flags.bits())
    }

    pub fn show_progress(caption: String, flags: ProgressFlags) {
        (get_raw_api!().show_progress)(string_to_wchar_cstr(&caption), flags.bits())
    }

    pub fn set_progress_description(caption: String) {
        (get_raw_api!().set_progress_description)(string_to_wchar_cstr(&caption), )
    }

    pub fn should_stop() -> bool {
        (get_raw_api!().should_stop)() != 0
    }
    pub fn hide_progress() {
        (get_raw_api!().hide_progress)()
    }

    pub fn set_progress_percentage(num: u32, total: u32) {
        let mut percentage:u32;
        if total > 0 {
            percentage = (100.0f32 * num as f32 / total as f32).round() as u32;
        } else {
            percentage = 100;
        }
        (get_raw_api!().set_progress_percentage)(percentage);
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