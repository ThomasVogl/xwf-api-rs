use std::sync::Mutex;
use chrono::{DateTime, TimeZone, Utc};
use crate::error::XwfError;
#[allow(unused_imports)]
use crate::xwfwarn;

static WRITE_LOCK: Mutex<()> = Mutex::new(());

/// Serializes all XWF_* calls that alter data in the case or volume snapshot.
/// API documentation (XT_Init): "If you call XWF_* functions that alter data (in the case or
/// volume snapshot), please ensure that you call each of them only once concurrently."
pub(crate) fn with_write_lock<R, F: FnOnce() -> R>(f: F) -> R {
    let _guard = WRITE_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    f()
}

/// Converts a Windows FILETIME value (100 ns intervals since 1601-01-01) into a UTC timestamp.
/// Returns None for values that cannot be represented (e.g. negative or corrupt timestamps).
pub fn filetime_to_utc(filetime: i64) -> Option<DateTime<Utc>> {
    let unix_epoch_sec = filetime.div_euclid(10_000_000) - 11644473600i64;
    let nsec_fraction = (filetime.rem_euclid(10_000_000) * 100) as u32;
    Utc.timestamp_opt(unix_epoch_sec, nsec_fraction).single()
}

pub fn split_values_by_comma(input: &String, num_expected: usize) -> Result<Vec<String>, XwfError> {
    let vec_assocs: Vec<String> = input.split(", ").map(|s| s.to_string()).collect();

    if vec_assocs.len() != num_expected {
        Err(XwfError::GivenBufferToSmallForContent)
    } else {
        Ok(vec_assocs)
    }
}


pub fn char_ptr_to_string(mut ptr: *mut u8) -> String {

    let mut vec_u8: Vec<u8> = Vec::new();

    unsafe {
        let mut chr = *ptr;

        while chr != 0 {
            vec_u8.push(chr);
            ptr = ptr.add(1);
            chr = *ptr;
        }
    }

    String::from_utf8(vec_u8).unwrap_or_default()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filetime_to_utc() {
        // 2021-01-01 00:00:00.5 UTC
        let t = filetime_to_utc(132_539_328_005_000_000).unwrap();
        assert_eq!(t.to_rfc3339(), "2021-01-01T00:00:00.500+00:00");
        // FILETIME 0 = 1601-01-01
        assert_eq!(filetime_to_utc(0).unwrap().to_rfc3339(), "1601-01-01T00:00:00+00:00");
        // negative / corrupt values must not panic
        assert!(filetime_to_utc(-1).is_some());
        let _ = filetime_to_utc(i64::MIN);
        let _ = filetime_to_utc(i64::MAX);
    }
}
