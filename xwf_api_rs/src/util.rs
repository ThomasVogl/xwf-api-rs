use crate::error::XwfError;
use crate::xwf_types::XtVersion;
#[allow(unused_imports)]
use crate::xwfwarn;

pub fn check_supported_xwf_version(version: XtVersion) -> Result<(), XwfError> {
    const MIN_VERSION: (u16, u16) = {
        #[cfg(feature = "api_21_3")]
        { (21, 3) }
        #[cfg(not(feature = "api_21_3"))]
        #[cfg(feature = "api_21_2")]
        { (21, 2) }
        #[cfg(not(feature = "api_21_2"))]
        #[cfg(feature = "api_21_1")]
        { (21, 1) }
        #[cfg(not(feature = "api_21_1"))]
        #[cfg(feature = "api_21_0")]
        { (21, 0) }
        #[cfg(not(feature = "api_21_0"))]
        #[cfg(feature = "api_20_9")]
        { (20, 9) }
        #[cfg(not(feature = "api_20_9"))]
        #[cfg(feature = "api_20_8")]
        { (20, 8) }
        #[cfg(not(feature = "api_20_8"))]
        #[cfg(feature = "api_20_7")]
        { (20, 7) }
        #[cfg(not(feature = "api_20_7"))]
        #[cfg(feature = "api_20_6")]
        { (20, 6) }
        #[cfg(not(feature = "api_20_6"))]
        #[cfg(feature = "api_20_5")]
        { (20, 5) }
        #[cfg(not(feature = "api_20_5"))]
        #[cfg(feature = "api_20_4")]
        { (20, 4) }
        #[cfg(not(feature = "api_20_4"))]
        #[cfg(feature = "api_20_3")]
        { (20, 3) }
        #[cfg(not(feature = "api_20_3"))]
        #[cfg(feature = "api_20_2")]
        { (20, 2) }
        #[cfg(not(feature = "api_20_2"))]
        #[cfg(feature = "api_20_1")]
        { (20, 1) }
        #[cfg(not(feature = "api_20_1"))]
        { (20, 0) }
    };


    #[cfg(not(feature = "api_20_1"))] {
        xwfwarn!("No API level defined. Assuming minimum supported XWF Version 20.0 of xwf-api-rs crate.");
        xwfwarn!("You should define your minimal required API Level of xwf-api-rs (via feature api_<major>_<minor>).");
        xwfwarn!("For differences of XWF API, also consult official X-Tension API Documentation \
        https://www.x-ways.net/forensics/x-tensions/api.html");
    }

    if version.major < MIN_VERSION.0 || (version.major == MIN_VERSION.0 && version.minor < MIN_VERSION.1) {
        Err(XwfError::IncompatibleXwfVersion(version, MIN_VERSION))
    } else {
        Ok(())
    }
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

