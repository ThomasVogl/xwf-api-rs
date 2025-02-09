use std::fmt;
use crate::xwf_types::{ItemInfoClassification, ItemInfoDeletion};

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