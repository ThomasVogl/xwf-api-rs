use winapi::ctypes::__int64;
use winapi::shared::minwindef::{DWORD, LPVOID};

#[repr(packed(2))]
pub struct SrcInfo {
    pub n_struct_size: DWORD,
    pub n_buf_size: __int64 ,
    pub p_buffer: LPVOID
}
