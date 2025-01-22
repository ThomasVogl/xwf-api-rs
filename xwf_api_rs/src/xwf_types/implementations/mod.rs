use chrono::NaiveDateTime;
use winapi::ctypes::__int64;
use winapi::shared::minwindef::LPVOID;
use crate::xwf_types::*;

mod impl_conversions;
mod impl_defaults;
mod impl_displays;
mod impl_orderings;

impl ItemInfoDeletion {
    pub fn is_existing(&self) -> bool {
        match &self {
            ItemInfoDeletion::Existing => true,
            _ => false
        }
    }
}

impl XwfDateTime {
    pub fn to_naive(&self) -> NaiveDateTime {
        match &self {
            XwfDateTime::Utc(v) => v.naive_local(),
            XwfDateTime::Local(v) => v.naive_local(),
            XwfDateTime::NoTimezone(v) => *v,
        }
    }
}



impl SrcInfo {
    pub fn from_buffer(data: &mut [u8]) -> SrcInfo {
        assert!(!data.is_empty(), "Buffer cannot be empty");

        #[cfg(not(feature = "api_21_1"))]
        assert!(
            data.len() as u64 <= 0x7FFF_FFFF_FFFF_FFFF,
            "Buffer size exceeds maximum supported size for SrcInfo"
        );

        SrcInfo {
            n_struct_size: size_of::<SrcInfo>() as u32,
            n_buf_size: data.len() as __int64,
            p_buffer: data.as_mut_ptr() as LPVOID
        }
    }
}