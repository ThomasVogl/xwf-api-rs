use std::cmp::Ordering;
use crate::error::XwfError;
use crate::evidence::Evidence;
use crate::item::{Item, UniqueItemId};
use crate::volume::HashType;
use crate::xwf_types::{FileFormatConsistency, FileTypeCategory, FileTypeStatus, ItemInfoClassification, ItemInfoDeletion, ItemInfoFlags, StorageLocationType};

struct ItemAttributes {
    unique_id: UniqueItemId,
    flags: ItemInfoFlags,
    name: String,
    alt_name: String,
    path: String,
    classification: ItemInfoClassification,
    deletion: ItemInfoDeletion,
    status: FileTypeStatus,
    consistency: FileFormatConsistency,
    category: FileTypeCategory,
    filetype: String,
    location_type: StorageLocationType,
    hash1: Option<Vec<u8>>,
    hash2: Option<Vec<u8>>

}


impl ItemAttributes {
    fn new(item: &Item, evidence: &Evidence) -> Result<Self, XwfError> {
        let (status, consistency, category) = item.get_item_category()?;
        Ok(ItemAttributes  {
            unique_id: item.unique_id(evidence),
            flags: item.get_item_info_flags()?,
            name: item.get_name(false)?,
            alt_name: item.get_name(true)?,
            path: item.get_path()?,
            classification: item.get_item_info_classification()?,
            deletion: item.get_item_info_deletion()?,
            status,
            consistency,
            category,
            filetype: item.get_item_type(false)?,
            location_type: StorageLocationType::new(item.get_name(false)?.as_str(), item.get_path()?.as_str()),
            hash1: item.get_hash_value(HashType::MD5, false),
            hash2: item.get_hash_value(HashType::MD4, true),
        })
    }
}

impl Eq for ItemAttributes {}

impl PartialEq<Self> for ItemAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.unique_id.eq(&other.unique_id)
    }
}

impl PartialOrd<Self> for ItemAttributes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ItemAttributes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unique_id.cmp(&other.unique_id)
    }
}


fn sort_by_evidence_path_name(attrib_a: &ItemAttributes, attrib_b: &ItemAttributes) -> Ordering {
    attrib_a.unique_id.short_ev_id.cmp(&attrib_b.unique_id.short_ev_id).then(
        attrib_a.path.to_uppercase().cmp(&attrib_b.path.to_uppercase()).then(
            attrib_a.name.to_uppercase().cmp(&attrib_b.name.to_uppercase())
        )
    )
}


fn sort_by_hash1(attrib_a: &ItemAttributes, attrib_b: &ItemAttributes) -> Ordering {
    attrib_a.hash1.cmp(&attrib_b.hash1).then(
        attrib_a.deletion.cmp(&attrib_b.deletion)
    )
}