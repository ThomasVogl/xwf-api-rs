use crate::xwf_types::XwfDateTime;

impl Ord for XwfDateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_naive().cmp(&other.to_naive())
    }
}

impl PartialOrd for XwfDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for XwfDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.to_naive().eq(&other.to_naive())
    }
}

impl Eq for XwfDateTime {}