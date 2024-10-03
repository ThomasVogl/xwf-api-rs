use serde::{Deserialize, Deserializer};
use crate::xwf_types::FileTypeCategory;

impl<'de> Deserialize<'de> for FileTypeCategory {

    fn deserialize<D>(deserializer: D) -> Result<FileTypeCategory, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        FileTypeCategory::try_from(s).map_err(serde::de::Error::custom)

    }
}