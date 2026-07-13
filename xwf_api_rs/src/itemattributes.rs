use std::cmp::Ordering;
use crate::error::XwfError;
use crate::evidence::Evidence;
use crate::item::{Item, UniqueItemId};
use crate::xwf_types::{FileFormatConsistency, FileTypeCategory, FileTypeStatus, XwfHashType, ItemInfoClassification, ItemInfoDeletion, ItemInfoFlags, StorageLocationType};

#[derive(Clone)]
pub struct ItemAttributes {
    pub unique_id: UniqueItemId,
    pub unique_id_parent: Option<UniqueItemId>,
    pub evidence_name: String,
    pub flags: ItemInfoFlags,
    pub name: String,
    pub alt_name: Option<String>,
    pub path: String,
    pub filesize: usize,
    pub classification: ItemInfoClassification,
    pub deletion: ItemInfoDeletion,
    pub status: FileTypeStatus,
    pub consistency: FileFormatConsistency,
    pub category: FileTypeCategory,
    pub filetype: String,
    pub location_type: StorageLocationType,
    pub hash1: Option<Vec<u8>>,
    pub hash2: Option<Vec<u8>>

}


impl ItemAttributes {
    pub fn new(item: &Item, evidence: &Evidence) -> Result<Self, XwfError> {
        let (status, consistency, category) = item.get_item_category()?;
        Ok(ItemAttributes  {
            unique_id: item.unique_id(evidence),
            unique_id_parent: item.get_parent_item().map(|i| i.unique_id(evidence)),
            evidence_name: evidence.get_name()?,
            flags: item.get_item_info_flags()?,
            name: item.get_name(false)?,
            alt_name: item.get_name(true).ok(),
            path: item.get_path()?,
            filesize: item.get_size(),
            classification: item.get_item_info_classification()?,
            deletion: item.get_item_info_deletion()?,
            status,
            consistency,
            category,
            filetype: item.get_item_type(false)?,
            location_type: StorageLocationType::new(item.get_name(false)?.as_str(), item.get_path()?.as_str()),
            hash1: item.get_hash_value(XwfHashType::MD5, false),
            hash2: item.get_hash_value(XwfHashType::MD4, true),
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


pub fn sort_by_evidence_path_name(attrib_a: &ItemAttributes, attrib_b: &ItemAttributes) -> Ordering {
    attrib_a.evidence_name.cmp(&attrib_b.evidence_name).then(
        attrib_a.path.to_uppercase().cmp(&attrib_b.path.to_uppercase()).then(
            attrib_a.name.to_uppercase().cmp(&attrib_b.name.to_uppercase())
        )
    )
}


pub fn sort_by_best_duplicate_item(attrib_a: &ItemAttributes, attrib_b: &ItemAttributes) -> Ordering {
    attrib_a.deletion.cmp(&attrib_b.deletion).then(
        attrib_a.location_type.cmp(&attrib_b.location_type).then(
            attrib_a.filesize.cmp(&attrib_b.filesize).reverse().then(
                sort_by_evidence_path_name(attrib_a, attrib_b)
            )
        )
    )
}


pub fn sort_by_hash1(attrib_a: &ItemAttributes, attrib_b: &ItemAttributes) -> Ordering {
    attrib_a.hash1.cmp(&attrib_b.hash1).then(
        attrib_a.deletion.cmp(&attrib_b.deletion)
    )
}


