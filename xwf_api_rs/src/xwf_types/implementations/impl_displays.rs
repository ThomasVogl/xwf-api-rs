use std::fmt;
use crate::xwf_types::{FileTypeCategory, ItemInfoClassification, ItemInfoDeletion};

impl fmt::Display for ItemInfoClassification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl fmt::Display for ItemInfoDeletion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


impl fmt::Display for FileTypeCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileTypeCategory::Other(value) => write!(f, "{}", value),
            _ => write!(f, "{:?}", self)
        }
    }
}
