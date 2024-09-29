use crate::error::XwfError;

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

