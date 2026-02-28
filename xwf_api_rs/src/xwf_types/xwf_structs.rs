use std::fmt;
use serde::{Deserialize, Serialize};
use winapi::ctypes::__int64;
use winapi::shared::minwindef::{DWORD, LPVOID};
use crate::xwf_types::XwfHashType;

#[repr(packed(2))]
pub struct SrcInfo {
    pub n_struct_size: DWORD,
    pub n_buf_size: __int64 ,
    pub p_buffer: LPVOID
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct XwfHash {
    pub hash_type: XwfHashType,
    #[serde(serialize_with = "hex::serialize", deserialize_with = "hex::deserialize")]
    pub hash_value: Vec<u8>,
}


#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug)]
pub struct DaylightSavingsDefinition {
    pub flags_and_more: u16,
    pub start_hour: u8,
    pub start_day_and_week: u8,
    pub start_month: u8,
    pub end_hour: u8,
    pub end_day_and_week: u8,
    pub end_month: u8,
}

impl DaylightSavingsDefinition {
    pub fn zeroed() -> Self {
        // Doku: "zero it out before the call"
        unsafe { std::mem::zeroed() }
    }

    /// Höchstes Bit von flags_and_more: Sommerzeit aktiv?
    pub fn has_daylight_saving(&self) -> bool {
        (self.flags_and_more & 0x8000) != 0
    }

    /// Wochentag (oberes Nibble): 0=Sonntag, 1=Montag, ...
    pub fn start_weekday(&self) -> u8 {
        self.start_day_and_week >> 4
    }

    /// Welcher Wochentag im Monat (unteres Nibble): 1–4, oder 5 = letzter
    pub fn start_week_number(&self) -> u8 {
        self.start_day_and_week & 0x0F
    }

    pub fn end_weekday(&self) -> u8 {
        self.end_day_and_week >> 4
    }

    pub fn end_week_number(&self) -> u8 {
        self.end_day_and_week & 0x0F
    }
}

impl fmt::Display for DaylightSavingsDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.has_daylight_saving() {
            return write!(f, "Keine Sommerzeit definiert");
        }
        let weekday_name = |d: u8| -> &str {
            match d {
                0 => "So", 1 => "Mo", 2 => "Di", 3 => "Mi",
                4 => "Do", 5 => "Fr", 6 => "Sa", _ => "??",
            }
        };
        write!(
            f,
            "Sommerzeit: Start = {}. {} im Monat {} um {}:00, \
             Ende = {}. {} im Monat {} um {}:00",
            self.start_week_number(),
            weekday_name(self.start_weekday()),
            self.start_month,
            self.start_hour,
            self.end_week_number(),
            weekday_name(self.end_weekday()),
            self.end_month,
            self.end_hour,
        )
    }
}
