use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Write};
use std::ops::BitOr;
use std::path::Path;
use std::ptr::null_mut;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::DateTime;

use std::hash::{Hash, Hasher};

use winapi::shared::minwindef::{BOOL, DWORD, LPVOID, PDWORD};
use winapi::shared::ntdef::{HANDLE, LPWSTR, PVOID};
use serde::{Deserialize, Serialize};
use crate::{get_raw_api, util};
use crate::error::XwfError;
use crate::evidence::Evidence;
use crate::traits::NativeHandle;
use crate::raw_api::RAW_API;
use crate::volume::Volume;
use crate::xwf_types::*;
use regex::Regex;
use winapi::ctypes::__int64;
use winsafe::WString;
use crate::util::char_ptr_to_string;

const DEFAULT_DATA_CHUNK_SIZE: usize = 1*1024*1024;
const BUF_SIZE_HASHSET_QUERY: usize = 512;
const BUF_SIZE_REPORTTABLE_QUERY: usize = 1024;
const MAX_BUF_SIZE: usize = 65535;
const MAX_PARENT_ITERATIONS: usize = 1024;
pub struct ItemIterator {
    cur_item: Option<Item>,
    num_iterations: usize

}

impl ItemIterator {
    fn create(item: &Item) -> Self{
        ItemIterator {
            cur_item: Some(*item),
            num_iterations: 0
        }
    }
}

impl Iterator for ItemIterator {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {

        match self.cur_item {
            Some(i) => {
                //force end of iter() method by MAX_PARENT_ITERATIONS iterations
                if self.num_iterations > MAX_PARENT_ITERATIONS {
                    return None;
                }
                self.cur_item = i.get_parent_item();
                self.num_iterations += 1;
                Some(i)
            },
            None => None,
        }
    }
}

/// Source of the contents of a file created via Item::create_file_ext()
pub enum FileSource<'a> {
    /// no contents (behaves like XWF_CreateItem)
    None,
    /// excerpt of the parent file, starting at the given offset (flag 0x02)
    ExcerptFromParent(i64),
    /// external file attached to the volume snapshot of an evidence object (flag 0x04)
    ExternalFile(&'a str),
    /// contents taken from a buffer (flag 0x10)
    Buffer(&'a mut SrcInfo),
}

#[derive(Copy, Clone, Debug)]
pub struct Item {
    pub item_id: i32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id
    }
}
impl Eq for Item {}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item_id.hash(state)
    }
}


#[derive(Copy, Clone, Debug)]
pub struct UniqueItemId {
    pub item_id: i32,
    pub evidence_id: u32,
    pub short_ev_id: u16,
}

impl Hash for UniqueItemId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item_id.hash(state);
        self.evidence_id.hash(state);
    }
}

impl PartialEq for UniqueItemId {
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id && self.evidence_id == other.evidence_id
    }
}

impl Eq for UniqueItemId {}

impl PartialOrd for UniqueItemId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
        
    }
}


impl Ord for UniqueItemId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evidence_id.cmp(&other.evidence_id).then(self.item_id.cmp(&other.item_id))
    }
}

impl UniqueItemId {
    pub fn new(evidence_id: u32, short_ev_id: u16, item_id: u32) -> UniqueItemId {
        UniqueItemId {
            item_id: item_id as i32,
            evidence_id: evidence_id,
            short_ev_id: short_ev_id
        }
    }
    pub fn item(&self) -> Item {
        Item::new(self.item_id)
    }

    /// Panics if the evidence object does not exist (anymore). Use try_evidence() to avoid this.
    pub fn evidence(&self) -> Evidence {
        self.try_evidence().expect("evidence object of unique item id does not exist")
    }

    pub fn try_evidence(&self) -> Option<Evidence> {
        Evidence::new((get_raw_api!().get_ev_obj)(self.evidence_id))
    }
}

impl Display for UniqueItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.short_ev_id, self.item_id)
    }
}

impl TryFrom<String> for UniqueItemId {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"([0-9]+)-([0-9]+)")?;
        let caps = re.captures(&value).ok_or(regex::Error::Syntax("invalid unique id syntax".to_string()))?;

        let evidence_id: u16 = caps[1].parse()?;
        let item_id: i32 = caps[2].parse()?;

        Ok(UniqueItemId {
            item_id: item_id,
            evidence_id: evidence_id as u32,
            short_ev_id: evidence_id }
        )
    }
}

impl Into<i64> for UniqueItemId {
    fn into(self) -> i64 {
        let mut ret = self.item_id as i64;
        ret |= (self.evidence_id as i64) << 32;
        ret
    }
}

impl From<i64> for UniqueItemId {
    fn from(id: i64) -> Self {
        let evidence_id = (id >> 32) as u32;
        let item_id = (id & 0xffffffff) as i32;

        UniqueItemId {
            item_id,
            evidence_id,
            short_ev_id: 0,
        }
    }
}

impl Serialize for UniqueItemId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for UniqueItemId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {

        let s: String = Deserialize::deserialize(deserializer)?;

        UniqueItemId::try_from(s).map_err(serde::de::Error::custom)
    }
}



impl Item {

    pub fn iter(&self) -> ItemIterator {
        ItemIterator::create(self)
    }
    pub fn new(item_id: i32) -> Item {
        Item {
            item_id,
        }
    }

    pub fn get_child_items(&self, volume: &Volume) -> Result<Vec<Item>, XwfError> {
        let mut ret: Vec<Item> = Vec::new();
        let num_items = volume.select()?;

        for i in 0..num_items {
            let item = Item::new(i);
            match item.get_parent_item() {
                Some(parent_item) => { 
                    if self.item_id == parent_item.item_id {
                        ret.push(item)
                    }
                },
                None => {},
            }

        }
        Ok(ret)
    }



    pub fn open(&self, volume: &Volume, flags: OpenItemFlags) -> Result<ItemHandle, XwfError> {
        let handle = (get_raw_api!().open_item)(
            volume.handle(),
            self.item_id,
            flags.bits());

        if handle == null_mut() {
            return Err(XwfError::FailedToGetItemHandle(self.item_id));
        }

        ItemHandle::new(handle, *self, true)
    }



    /// Returns the hash value of the item, if the hash type of the currently selected volume
    /// snapshot matches `hash_type`. Returns None if no hash is available or the hash type differs.
    pub fn get_hash_value(&self, hash_type: XwfHashType, get_secondary: bool) -> Option<Vec<u8>>{
        let hash = self.get_hash(get_secondary)?;
        if hash.hash_type == hash_type {
            Some(hash.hash_value)
        } else {
            None
        }
    }

    /// Returns the hash value of the item together with the hash type of the currently selected
    /// volume snapshot.
    pub fn get_hash(&self, get_secondary: bool) -> Option<XwfHash> {
        let hash_type = Volume::current_hash_type(get_secondary)?;

        let flags: u32 = if get_secondary { 0x02 } else { 0x01 };

        // X-Ways writes the hash of the type configured for the volume snapshot, so the buffer
        // must be large enough for any hash type, not just for the expected one
        let mut buf: Vec<u8> = vec![0u8; HASH_BUFFER_SIZE];
        buf[0..4].clone_from_slice(&flags.to_le_bytes());

        let ret = (get_raw_api!().get_hash_value)(self.item_id, buf.as_mut_ptr() as LPVOID);

        if ret != 0 {
            buf.truncate(hash_type.get_hash_size());
            Some(XwfHash { hash_type, hash_value: buf })
        } else {
            None
        }
    }

    pub fn set_hash_value(&self, hash_value: &Vec<u8>, set_secondary: bool) -> Result<(), XwfError>{
        let mut param: u32 = 1;

        if set_secondary {
            param = 2;
        }

        // X-Ways reads as many bytes as implied by the hash type of the volume snapshot
        let hash_type = Volume::current_hash_type(set_secondary)
            .ok_or(XwfError::XwfFunctionCallFailed("get_vs_prop"))?;
        if hash_value.len() != hash_type.get_hash_size() {
            return Err(XwfError::InvalidInputArgument);
        }

        let x = util::with_write_lock(|| (get_raw_api!().set_hash_value)(self.item_id, hash_value.as_ptr() as LPVOID, param));

        if x != 0 {
            Ok(())
        } else {
            Err(XwfError::XwfFunctionCallFailed("set_hash_value"))
        }
    }

    pub fn set_item_info_flags(&self, flags: ItemInfoFlags, remove_flags: bool) -> Result<(), XwfError> {
        let n_info_type = if remove_flags { 65 } else { 64 };

        // some flags can only be read, but must not be set or removed (API documentation)
        let flags = flags.difference(ItemInfoFlags::FileEmbeddedinOtherFile
            | ItemInfoFlags::FileContentsStoredExternally
            | ItemInfoFlags::AlternativeData);

        let result = util::with_write_lock(|| (get_raw_api!().set_item_information)(self.item_id, n_info_type, flags.bits() as i64));

        if result != 0 {
            Ok(())
        } else {
            Err(XwfError::XwfFunctionCallFailed("set_item_information"))
        }
    }


    pub fn set_item_info_classification(&self, classification: ItemInfoClassification) -> Result<(), XwfError> {
        if classification == ItemInfoClassification::UnknownEnumValue {
            return Err(XwfError::InvalidInputArgument);
        }

        let result = util::with_write_lock(|| (get_raw_api!().set_item_information)(self.item_id, 5, classification as i64));

        if result != 0 {
            Ok(())
        } else {
            Err(XwfError::XwfFunctionCallFailed("set_item_information"))
        }
    }


    /// Creates a file whose contents are taken from a buffer (flag 0x10 is set automatically).
    /// For excerpts of the parent file or attached external files use create_file_ext().
    pub fn create_file(&self, name: &String, creation_flags: FileCreationFlags, src_info: &mut SrcInfo) -> Result<Item, XwfError> {
        if creation_flags.intersects(FileCreationFlags::ExcerptFromParent | FileCreationFlags::AttachExternalFile) {
            return Err(XwfError::InvalidInputArgument);
        }
        self.create_file_ext(name, creation_flags, FileSource::Buffer(src_info))
    }

    /// Creates a file as child of this item. The type of pSourceInfo expected by X-Ways depends on
    /// the creation flags, so the corresponding flag is derived from `source`.
    pub fn create_file_ext(&self, name: &String, creation_flags: FileCreationFlags, source: FileSource) -> Result<Item, XwfError> {
        let wstr = WString::from_str(name);

        // flags 0x02, 0x04 and 0x10 are mutually exclusive and define the type of pSourceInfo
        let mut flags = creation_flags.difference(FileCreationFlags::ExcerptFromParent
            | FileCreationFlags::AttachExternalFile
            | FileCreationFlags::FileContentsFromBuffer);

        // declared here, so that they outlive the X-Ways call
        let mut offset: i64;
        let path: WString;

        let p_source: PVOID = match source {
            FileSource::None => {
                flags.remove(FileCreationFlags::KeepExternalFile);
                null_mut()
            },
            FileSource::ExcerptFromParent(start_offset) => {
                flags.remove(FileCreationFlags::KeepExternalFile);
                flags.insert(FileCreationFlags::ExcerptFromParent);
                offset = start_offset;
                &mut offset as *mut i64 as PVOID
            },
            FileSource::ExternalFile(file_path) => {
                flags.insert(FileCreationFlags::AttachExternalFile);
                path = WString::from_str(file_path);
                path.as_ptr() as PVOID
            },
            FileSource::Buffer(src_info) => {
                flags.remove(FileCreationFlags::KeepExternalFile);
                flags.insert(FileCreationFlags::FileContentsFromBuffer);
                src_info as *mut SrcInfo as PVOID
            },
        };

        let result = util::with_write_lock(|| (get_raw_api!().create_file)(wstr.as_ptr() as LPWSTR, flags.bits(), self.item_id, p_source));

        if result < 0 {
            Err(XwfError::XwfFunctionCallFailed("create_file"))
        } else {
            Ok(Item::new(result))
        }
    }

    pub fn get_item_info(&self, infotype: XwfItemInfoTypes) -> Result<i64, XwfError> {
        let mut success: Box<BOOL> = Box::new(1);
        let success_ptr: *mut BOOL = &mut *success;
        let result = (get_raw_api!().get_item_information)(self.item_id, infotype as i32, success_ptr);

        if *success != 0 {
            Ok(result)
        } else {
            Err(XwfError::XwfFunctionCallFailed("get_item_information"))
        }
    }

    

    pub fn get_item_info_deletion(&self) -> Result<ItemInfoDeletion, XwfError> {
        let result = self.get_item_info(XwfItemInfoTypes::Deletion)?;
        ItemInfoDeletion::try_from(result)
    }


    pub fn get_item_info_coloranalysis(&self) -> Result<ItemInfoColorAnalysis, XwfError> {
        let result = self.get_item_info(XwfItemInfoTypes::ColorAnalysis)?;
        ItemInfoColorAnalysis::try_from(result)
    }

    pub fn get_item_info_flags(&self) -> Result<ItemInfoFlags, XwfError> {
        let result = self.get_item_info(XwfItemInfoTypes::Flags)?;
        Ok(ItemInfoFlags::from_bits_truncate(result as u64))
    }

    pub fn get_item_info_attr(&self) -> Result<ItemInfoAttributes, XwfError> {
        let result = self.get_item_info(XwfItemInfoTypes::Attr)?;
        Ok(ItemInfoAttributes::from_bits_truncate(result))
    }

    pub fn get_item_info_time(&self, infotype: XwfItemInfoTypes, local_time: bool, flags: &ItemInfoFlags) -> Option<XwfDateTime> {

        let res_time_config = match infotype {
            XwfItemInfoTypes::CreationTime => {
                Ok(( XwfItemInfoTypes::CreationTime, flags.contains(ItemInfoFlags::FilesystemTimestampsNotInUTC) ))
            }
            XwfItemInfoTypes::ModificationTime => {
                Ok(( XwfItemInfoTypes::ModificationTime, flags.contains(ItemInfoFlags::FilesystemTimestampsNotInUTC) ))
            }
            XwfItemInfoTypes::LastAccessTime => {
                Ok(( XwfItemInfoTypes::LastAccessTime, flags.contains(ItemInfoFlags::FilesystemTimestampsNotInUTC) ))
            }
            XwfItemInfoTypes::EntryModificationTime => {
                Ok(( XwfItemInfoTypes::EntryModificationTime, flags.contains(ItemInfoFlags::FilesystemTimestampsNotInUTC) ))
            }
            XwfItemInfoTypes::DeletionTime => {
                Ok(( XwfItemInfoTypes::DeletionTime, flags.contains(ItemInfoFlags::FilesystemTimestampsNotInUTC) ))
            }
            XwfItemInfoTypes::InternalCreationTime => {
                Ok(( XwfItemInfoTypes::InternalCreationTime, flags.contains(ItemInfoFlags::InternalCreationTimestampsNotInUTC) ))
            }
            _ => {
                Err(XwfError::InvalidInputArgument)
            }
        };

        match res_time_config {
            Ok(time_config) => {
                let result = self.get_item_info(time_config.0).unwrap_or(0);

                if result == 0 {
                    return None;
                }

                // None for timestamps that cannot be represented (e.g. negative values)
                let time = util::filetime_to_utc(result)?;

                if time_config.1 {
                    Some(XwfDateTime::NoTimezone(time.naive_utc()))
                } else {

                    if local_time {
                        Some(XwfDateTime::Local(DateTime::from(time)))
                    } else {
                        Some(XwfDateTime::Utc(time))

                    }
                }
            },
            Err(_) => {
                None
            }

        }
    }

    pub fn get_item_info_classification(&self) -> Result<ItemInfoClassification, XwfError> {
        let result = self.get_item_info(XwfItemInfoTypes::Classification)?;
        ItemInfoClassification::try_from(result)
    }

    pub fn get_size(&self) -> usize {
        (get_raw_api!().get_item_size)(self.item_id) as usize
    }


    pub fn get_name(&self, alt_name: bool) -> Result<String, XwfError> {
        let mut item_id: DWORD = self.item_id as DWORD;
        if alt_name {
            item_id = item_id | 0x80000000;
        }

        let wchr_ptr = (get_raw_api!().get_item_name)(item_id);
        if wchr_ptr.is_null() {
            Err(XwfError::XwfFunctionCallFailed("get_item_name"))
        } else {
            let s = unsafe { winsafe::WString::from_wchars_nullt(wchr_ptr) };
            Ok(s.to_string())
        }

    }

    pub fn get_path(&self) -> Result<String, XwfError> {

        let mut path_components: Vec<String> = self.iter()
            .map(|p| p.get_name(false))
            .collect::<Result<Vec<String>, XwfError>>()?;

        path_components.pop();
        path_components.reverse();

        Ok("\\".to_string() + &path_components.join("\\"))
    }

    pub fn add_to_report_table<S: AsRef<str>>(&self, name: S, flags: AddReportTableFlags) {
        let wchar_c_str = WString::from_str(name);
        util::with_write_lock(|| (get_raw_api!().add_to_report_table)(self.item_id, wchar_c_str.as_ptr() as LPWSTR, flags.bits()));
    }

    pub fn get_parent_dir(&self) -> Option<Item> {
        self.iter().find(|i|{
            match i.get_item_info_flags() {
                Ok(flags) => flags.contains(ItemInfoFlags::IsDirectory),
                Err(_) => false,
            }
        })
    }


    pub fn get_parent_item(&self) -> Option<Item> {
        let parent_id = (get_raw_api!().get_item_parent)(self.item_id);

        if parent_id < 0 {
            None
        } else if parent_id == self.item_id {
            //if item is the parent of itself, do consider this as "no parent", as this makes no sense
            None
        }
        else {
            Some(Item::new(parent_id))
        }
    }

    pub fn get_hierarchy(&self) -> Vec<Item> {
        match self.get_parent_item() {
            Some(parent) => parent.iter().collect(),
            None => vec![],
        }
    }

    pub fn get_item_type(&self, long_desc: bool) -> Result<String, XwfError> {
        let mut buf = [0u16; 256];

        let mut flags = ItemTypeFlags::empty();

        if long_desc {
            flags = flags.bitor(ItemTypeFlags::TextualDescriptionType);
        }

        let buf_and_flags = (buf.len() as u32 ) | flags.bits();
        let _ = (get_raw_api!().get_item_type)(self.item_id, buf.as_mut_ptr(), buf_and_flags);
        Ok(winsafe::WString::from_wchars_slice(&buf).to_string())
    }


    pub fn get_report_tables(&self) -> Result<Vec<String>, XwfError> {
        let mut buf_size = BUF_SIZE_REPORTTABLE_QUERY;
        let mut current_res: Result<Vec<String>, XwfError > = Err(XwfError::GivenBufferToSmallForContent);

        while buf_size < MAX_BUF_SIZE && current_res.as_ref().is_err_and(|e| e.is_buffer_too_small()) {
            match self.get_report_tables_internal(buf_size) {
                Ok(v) => {
                    current_res = Ok(v)
                }
                Err(e) => {
                    buf_size = buf_size*2;
                    current_res = Err(e)
                }
            }
        }
        current_res
    }

    fn get_report_tables_internal(&self, buf_size: usize) -> Result<Vec<String>, XwfError> {
        let mut buf: Vec<u16> = Vec::new();
        buf.resize(buf_size, 0);

        // buffer length in characters in the lower bits, highest byte = label type filter (0 = all)
        let ret = (get_raw_api!().get_report_table_assocs)(self.item_id, buf.as_mut_ptr(), buf.len() as DWORD);

        if ret <= 0 {
            return Ok(Vec::new());
        }

        // no terminating null character -> buffer too small
        let Some(len) = buf.iter().position(|c| *c == 0) else {
            return Err(XwfError::GivenBufferToSmallForContent);
        };

        // up to v21.2 the return value is the number of labels, since v21.3 each of the lower
        // 6 bytes holds the number of labels of one label type (capped at 255)
        let byte_counts: Vec<i64> = (0..6).map(|i| (ret >> (i * 8)) & 0xFF).collect();
        let num_assocs: i64 = if ret > 0xFF && byte_counts.iter().all(|c| *c < 0xFF) {
            byte_counts.iter().sum()
        } else {
            ret
        };

        let assocs = String::from_utf16_lossy(&buf[..len]);
        match util::split_values_by_comma(&assocs, num_assocs as usize) {
            Ok(v) => Ok(v),
            // number of labels is not reliable (e.g. counts capped at 255 or label names containing ", "),
            // but the whole string has been received, so a larger buffer would not help
            Err(_) => Ok(assocs.split(", ").map(|s| s.to_string()).collect()),
        }
    }


    pub fn get_hash_sets(&self) -> Result<Vec<String>, XwfError> {
        let mut buf_size = BUF_SIZE_HASHSET_QUERY;
        let mut current_res: Result<Vec<String>, XwfError > = Err(XwfError::GivenBufferToSmallForContent);

        while buf_size < MAX_BUF_SIZE && current_res.as_ref().is_err_and(|e| e.is_buffer_too_small()) {
            match self.get_hash_sets_internal(buf_size) {
                Ok(v) => {
                    current_res = Ok(v)
                }
                Err(e) => {
                    buf_size = buf_size*2;
                    current_res = Err(e)
                }
            }
        }
        current_res
    }

    fn get_hash_sets_internal(&self, buf_size: usize) -> Result<Vec<String>, XwfError> {
        let mut buf: Vec<u16> = Vec::new();
        buf.resize(buf_size, 0);
        let expected_num_assocs = (get_raw_api!().get_hashset_assocs)(self.item_id, buf.as_mut_ptr(), buf.len() as i32);

        if expected_num_assocs < 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_hashset_assocs"));
        }

        if expected_num_assocs == 0 {
            return Ok(Vec::new());
        }

        let assocs = winsafe::WString::from_wchars_slice(&buf).to_string();

        if assocs.len() == buf.len() {
            Err(XwfError::GivenBufferToSmallForContent)

        } else {
            let vec_assocs: Vec<String> = assocs.split(", ").map(|s| String::from_str(s).unwrap()).collect();
            if vec_assocs.len() < expected_num_assocs as usize {
                Err(XwfError::GivenBufferToSmallForContent)
            } else {
                Ok(vec_assocs)
            }
        }
    }

    pub fn get_comment(&self) -> Option<String>  {
        let ptr_wstr = (get_raw_api!().get_comment)(self.item_id);

        if ptr_wstr == null_mut() {
            None
        } else {
            unsafe { Some(WString::from_wchars_nullt(ptr_wstr).to_string()) }
        }

    }

    pub fn add_comment(&self, comment: &String, flags: AddCommentFlags) -> Result<(), XwfError>  {
        let wstring = WString::from_str(comment);
        let param = flags.bits();

        let success = util::with_write_lock(|| (get_raw_api!().add_comment)(self.item_id, wstring.as_ptr() as LPWSTR, param));
        if success > 0 {
            Ok(())
        } else {
            Err(XwfError::XwfFunctionCallFailed("add_comment"))
        }
    }

    pub fn get_item_offset(&self) -> Option<(i64, i64)>{
        let mut def_ofs = 0i64;
        let mut start_sector = 0i64;

        let ptr_def_ofs: *mut i64= &mut def_ofs;
        let ptr_start_sector: *mut i64 = &mut start_sector;


        (get_raw_api!().get_item_ofs)(self.item_id,ptr_def_ofs, ptr_start_sector);

        if ( start_sector < 0 ) || (def_ofs == 0) || ((def_ofs & 0xFFFFFFFF) == 0xFFFFFFFF) {
            return None;
        }

        if def_ofs < 0 {
            def_ofs = def_ofs.abs();
        }

        

        Some((def_ofs, start_sector))
    }

    pub fn get_extracted_metadata(&self) -> Option<Vec<String>>{
        let ptr = (get_raw_api!().get_extracted_metadata)(self.item_id);

        unsafe {
            if ptr == null_mut() {
                None
            } else {
                let s = WString::from_wchars_nullt(ptr).to_string();
                Some(s.replace("\r", "")
                    .split("\n").filter(|v| !v.is_empty())
                    .map(|v| v.to_string())
                    .collect())
            }
        }
    }

    pub fn add_extracted_metadata<S: AsRef<str>>(&self, prefix: S, metadata: S, flags: AddCommentFlags) -> Result<(), XwfError>  {
        // truncate to max. 4 characters (not bytes, to avoid panics on multi-byte UTF-8 chars)
        let prefix: String = prefix.as_ref().chars().take(4).collect();
        let wstring = WString::from_str(format!("[{}] {}", prefix , metadata.as_ref()));
        let res = util::with_write_lock(|| (get_raw_api!().add_extracted_metadata)(self.item_id, wstring.as_ptr() as LPWSTR, flags.bits()));
        if res == 0 {
            Err(XwfError::XwfFunctionCallFailed("add_extracted_metadata"))
        } else {
            Ok(())
        }
    }

    pub fn get_item_category(&self) -> Result<(FileTypeStatus, FileFormatConsistency, FileTypeCategory), XwfError> {
        let mut buf = [0u16; 256];

        let flags = ItemTypeFlags::ReceiveTypeStatus.bitor(ItemTypeFlags::TextualDescriptionCategory);

        let buf_and_flags = (buf.len() as u32) | flags.bits();
        let status = (get_raw_api!().get_item_type)(self.item_id, buf.as_mut_ptr(), buf_and_flags);

        if buf[0] == 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_item_type"));
        }

        if status < 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_item_type"));
        }

        Ok(
            (   FileTypeStatus::try_from(status)?,
                FileFormatConsistency::try_from(status)?,
                FileTypeCategory::from(WString::from_wchars_slice(&buf.as_slice()).to_string())
            ),
        )
    }

    pub fn unique_id(&self, evidence: &Evidence) -> UniqueItemId {
        UniqueItemId {
            item_id: self.item_id,
            evidence_id: evidence.get_id(),
            short_ev_id: evidence.get_short_id()
        }
    }
}


#[derive(Debug)]
pub struct ItemHandle {
    item_handle: HANDLE,
    item: Item,
    // true as long as the handle was opened by the X-Tension and has not been closed yet
    shall_close: AtomicBool,
}

impl NativeHandle for ItemHandle {
    fn get_handle(&self) -> HANDLE {
        return self.item_handle;
    }
}


impl ItemHandle {

    pub fn new(item_handle: HANDLE, item: Item, shall_close: bool) -> Result<ItemHandle, XwfError> {

        if item_handle == null_mut() {
            return Err(XwfError::InputHandleIsNull);
        }

        Ok(ItemHandle {
            item_handle,
            item,
            shall_close: AtomicBool::new(shall_close)
        })
    }
    pub fn handle(&self) -> HANDLE {
        self.item_handle
    }

    


    pub fn get_metadata(&self, full_output: bool) -> Option<Vec<String>> {
        let mut flags: DWORD = 1;
        let flags_ptr: PDWORD = &mut flags;

        if full_output {
            flags = 0;
        }
        let ptr = ((get_raw_api!().get_metadata_ex))(self.item_handle, flags_ptr);

        if ptr == null_mut() {
            None
        } 
        else {
            if ( flags & 0xFF000000) != 0 {
                (get_raw_api!().release_mem)(ptr);
                return None;
            } else {

                let metadata_str;
                if (flags & 0x1) != 0 {
                    metadata_str = char_ptr_to_string(ptr as *mut u8);
                } else {
                    unsafe { metadata_str = WString::from_wchars_nullt(ptr as *const u16).to_string(); }
                }
                
                (get_raw_api!().release_mem)(ptr);

                Some(metadata_str.split('\n').map(|s| s.to_string()).collect())
            }
        }
    }

    pub fn get_prop(&self, prop_type: PropType) -> i64 {
        (get_raw_api!().get_prop)(self.item_handle, prop_type as DWORD, null_mut())
    }

    /// Returns an empty string if the name is not available.
    pub fn get_name(&self) -> String {
        self.get_prop_string(PropType::PointerName)
    }

    /// Returns an empty string if the path is not available.
    pub fn get_path(&self) -> String {
        self.get_prop_string(PropType::PointerFilePath)
    }

    fn get_prop_string(&self, prop_type: PropType) -> String {
        let ptr = self.get_prop(prop_type) as LPWSTR;
        if ptr.is_null() {
            return String::new();
        }
        unsafe { WString::from_wchars_nullt(ptr).to_string() }
    }

    pub fn get_logical_size(&self) -> Result<i64, XwfError> {
        let size = self.get_prop(PropType::LogicalSize);
        if size > 0 {
                return Ok(size);
        } else {
            return Err(XwfError::InvalidItemSize);
        };
    }

    pub fn get_physical_size(&self) -> i64 {
        self.get_prop(PropType::PhysicalSize)
    }
    /// Closes the item handle, but only if it was opened by the X-Tension (Item::open()) and has
    /// not been closed before. Handles passed by X-Ways (XT_ProcessItemEx) are never closed.
    pub fn close(&self) {
        if self.shall_close.swap(false, Ordering::SeqCst) {
            (get_raw_api!().close)(self.item_handle);
        }
    }

    pub fn item(&self) -> &Item {
        &self.item
    }

    pub fn read(&self) -> Result<Vec<u8>, XwfError>{
        let size = self.get_logical_size()?;
        if size <= 0 {
            return Err(XwfError::InvalidItemSize);
        }

        let Some(ret) = self.read_chunk(0, size as usize, size as usize) else {
            return Err(XwfError::ReadItemDataFailed)
        };

        if ret.len() == 0 {
            Err(XwfError::ReadItemDataFailed)
        } else {
            Ok(ret)
        }
    }

    pub fn read_chunk(&self, offset: usize, mut chunk_size: usize, file_size: usize ) -> Option<Vec<u8>> {
        //limit chunk size to the remaining bytes of the item
        chunk_size = std::cmp::min(chunk_size, file_size-offset);
        if chunk_size <= 0 {
            return None;
        }

        let mut byte_buf: Vec<u8> = vec![0; chunk_size];
        let r = (get_raw_api!().read)(self.item_handle, offset as __int64, byte_buf.as_mut_ptr(), chunk_size as DWORD);

        if r<= 0 {
            None
        } else if r != chunk_size as DWORD {
            byte_buf.truncate(r as usize);
            Some(byte_buf)
        } else {
            Some(byte_buf)
        }

    }


    pub fn write_to_file<P: AsRef<Path>>(&self, dest: P) -> Result<(), XwfError>{

        let mut file = File::create(dest).map_err(|e| XwfError::IoError(e) )?;

        let file_size = self.get_logical_size()? as usize;

        let mut current_offset = 0usize;

        while current_offset < file_size {
            let Some(data) = self.read_chunk(current_offset, DEFAULT_DATA_CHUNK_SIZE, file_size) else {
                return Err(XwfError::ReadItemDataFailed);
            };
            current_offset+=data.len();
            file.write_all(&data.as_slice()).map_err(|e| XwfError::IoError(e))?;
        }

        Ok(())
    }
}

impl Drop for ItemHandle {
    fn drop(&mut self) {
        self.close();
    }
}

unsafe impl Send for Item {}
unsafe impl Sync for Item {}