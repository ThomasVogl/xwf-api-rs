use std::fmt;
use crate::xwf_types::ItemInfoClassification;

impl fmt::Display for ItemInfoClassification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}