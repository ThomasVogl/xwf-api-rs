use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Write};
use std::ops::BitOr;
use std::path::Path;
use std::ptr::null_mut;
use std::str::FromStr;
use chrono::{DateTime, TimeZone, Utc};

use std::hash::{Hash, Hasher};

use winapi::shared::minwindef::{BOOL, DWORD, LPVOID, PDWORD};
use winapi::shared::ntdef::{HANDLE, LPWSTR, PVOID};
use serde::{Deserialize, Serialize};
use crate::{get_raw_api, util};
use crate::error::XwfError;
use crate::evidence::Evidence;
use crate::traits::NativeHandle;
use crate::raw_api::RAW_API;
use crate::volume::{HashType, Volume};
use crate::xwf_types::*;
use regex::Regex;
use winapi::ctypes::__int64;
use winsafe::WString;
use crate::application::Application;
use crate::util::char_ptr_to_string;

const DEFAULT_DATA_CHUNK_SIZE: usize = 1*1024*1024;
const BUF_SIZE_REPORT_TABLE_QUERY: usize = 8192;
const BUF_SIZE_REPORT_HASHSET_QUERY: usize = 4096;

pub struct ItemIterator {
    cur_item: Option<Item>,

}

impl ItemIterator {
    fn create(item: &Item) -> Self{
        ItemIterator {
            cur_item: Some(*item),
        }
    }
}

impl Iterator for ItemIterator {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {

        match self.cur_item {
            Some(i) => {
                self.cur_item = i.get_parent_item();
                Some(i)
            },
            None => None,
        }
    }
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


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UniqueItemId {
    pub item_id: i32,
    pub evidence_id: u32,
    pub short_ev_id: u16,
}

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

    pub fn evidence(&self) -> Evidence {
        Evidence::new((get_raw_api!().get_ev_obj)(self.evidence_id)).unwrap()
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
            return Err(XwfError::FailedToGetObjectHandle);
        }

        ItemHandle::new(handle, *self)
    }



    pub fn get_hash_value(&self, hash_type: HashType, get_secondary: bool) -> Option<Vec<u8>>{
        let hash_size = hash_type.get_hash_size();
        let mut buf_size = hash_size;
        if buf_size < 4 {
            buf_size = 4;
        }

        let mut flags = 0x01u32;

        if get_secondary {
            flags = 0x02u32;
        }

        let mut buf: Vec<u8> = Vec::with_capacity(buf_size);
        buf.resize(buf_size,0);
        buf[0..4].clone_from_slice(&flags.to_le_bytes());

        let ret = (get_raw_api!().get_hash_value)(self.item_id, buf.as_mut_ptr() as LPVOID);

        if ret != 0 {
            buf.resize(hash_size, 0u8);
            return Some(buf);
        } else {
            return None;
        }
    }

    pub fn set_hash_value(&self, hash_value: &Vec<u8>, set_secondary: bool) -> Result<(), XwfError>{
        let mut param: u32 = 1;

        if set_secondary {
            param = 2;
        }

        let x = (get_raw_api!().set_hash_value)(self.item_id, hash_value.as_ptr() as LPVOID, param);

        if x != 0 {
            Ok(())
        } else {
            Err(XwfError::XwfFunctionCallFailed("set_hash_value"))
        }
    }

    pub fn set_item_info_flags(&self, flags: ItemInfoFlags, remove_flags: bool) -> Result<(), XwfError> {
        let n_info_type = if remove_flags { 65 } else { 64 };

        let result = (get_raw_api!().set_item_information)(self.item_id, n_info_type, flags.bits() as i64);

        if result != 0 {
            Ok(())
        } else {
            Err(XwfError::XwfFunctionCallFailed("set_item_information"))
        }
    }


    pub fn set_item_info_classification(&self, classification: ItemInfoClassification) -> Result<(), XwfError> {

        let result = (get_raw_api!().set_item_information)(self.item_id, 5, classification as i64);

        if result != 0 {
            Ok(())
        } else {
            Err(XwfError::XwfFunctionCallFailed("set_item_information"))
        }
    }


    pub fn create_file(&self, name: &String, creation_flags: FileCreationFlags, src_info: &mut SrcInfo) -> Result<Item, XwfError> {
        let wstr = WString::from_str(name);

        let p_src_info: *mut SrcInfo = src_info;

        let result = (get_raw_api!().create_file)(wstr.as_ptr() as LPWSTR, creation_flags.bits(), self.item_id, p_src_info as PVOID);

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

    pub fn get_item_info_flags(&self) -> Result<ItemInfoFlags, XwfError> {
        let result = self.get_item_info(XwfItemInfoTypes::Flags)?;
        Ok(ItemInfoFlags::from_bits_truncate(result as u64))
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

                let unix_epoch_sec = ( result / 10_000_000 ) - 11644473600i64;
                let nsec_fraction = (( result % 10_000_000) * 100) as u32;


                if time_config.1 {
                    let time = DateTime::from_timestamp(unix_epoch_sec, nsec_fraction)?.naive_utc();
                    Some(XwfDateTime::NoTimezone(time))
                } else {
                    let time = Utc.timestamp_opt(unix_epoch_sec, nsec_fraction).unwrap();

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

    pub fn get_name(&self) -> String {
        let wchr_ptr = (get_raw_api!().get_item_name)(self.item_id as DWORD);
        let s = unsafe { winsafe::WString::from_wchars_nullt(wchr_ptr) };
        s.to_string()
    }

    pub fn get_path(&self) -> String {

        let mut path_components: Vec<String> = self.iter().map(|p| p.get_name()).collect();
        path_components.pop();
        path_components.reverse();

        "\\".to_string() + &path_components.join("\\")
    }

    pub fn add_to_report_table<S: AsRef<str>>(&self, name: S, flags: AddReportTableFlags) {
        let wchar_c_str = WString::from_str(name);
        (get_raw_api!().add_to_report_table)(self.item_id, wchar_c_str.as_ptr() as LPWSTR, flags.bits());
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
        } else {
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
    pub fn __get_report_tables(&self) -> Result<Vec<String>, XwfError> {
        let mut buf = [0u16; BUF_SIZE_REPORT_TABLE_QUERY];
        let num_assocs = (get_raw_api!().get_report_table_assocs)(self.item_id, buf.as_mut_ptr(), buf.len() as i32);

        if num_assocs == 0 {
            return Ok(Vec::new());
        }

        let assocs = winsafe::WString::from_wchars_slice(&buf).to_string().to_string();

        if assocs.len() == buf.len() {
            Err(XwfError::GivenBufferToSmallForContent)
        } else {
            util::split_values_by_comma(&assocs, num_assocs as usize)
        }
    }

    pub fn get_hash_sets(&self) -> Result<Vec<String>, XwfError> {
        let mut buf = [0u16; BUF_SIZE_REPORT_HASHSET_QUERY];
        let num_assocs = (get_raw_api!().get_hashset_assocs)(self.item_id, buf.as_mut_ptr(), buf.len() as i32);

        if num_assocs < 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_hashset_assocs"));
        }

        if num_assocs == 0 {
            return Ok(Vec::new());
        }

        let assocs = winsafe::WString::from_wchars_slice(&buf).to_string();

        if assocs.len() == buf.len() {
            Err(XwfError::GivenBufferToSmallForContent)

        } else {
            let vec_assocs: Vec<String> = assocs.split(", ").map(|s| String::from_str(s).unwrap()).collect();
            if vec_assocs.len() != num_assocs as usize {
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

    pub fn get_item_category(&self) -> Result<(FileTypeStatus, FileFormatConsistency, FileTypeCategory), XwfError> {
        let mut buf = [0u16; 256];

        let flags = ItemTypeFlags::ReceiveTypeStatus.bitor(ItemTypeFlags::TextualDescriptionCategory);

        let buf_and_flags = (buf.len() as u32) | flags.bits();
        let status = (get_raw_api!().get_item_type)(self.item_id, buf.as_mut_ptr(), buf_and_flags);

        if buf[0] == 0 {
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
    item: Item
}

impl NativeHandle for ItemHandle {
    fn get_handle(&self) -> HANDLE {
        return self.item_handle;
    }
}


impl ItemHandle {

    pub fn new(item_handle: HANDLE, item: Item) -> Result<ItemHandle, XwfError> {

        if item_handle == null_mut() {
            return Err(XwfError::InputHandleIsNull);
        }

        Ok(ItemHandle {
            item_handle,
            item
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
                if flags == 0x1 {
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

    pub fn get_name(&self) -> String {
        unsafe { WString::from_wchars_nullt(self.get_prop(PropType::PointerName) as LPWSTR).to_string() }
    }

    pub fn get_path(&self) -> String {
        unsafe { WString::from_wchars_nullt(self.get_prop(PropType::PointerFilePath) as LPWSTR).to_string() }
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
    pub fn close(&self) {
        (get_raw_api!().close)(self.item_handle);
    }

    pub fn item(&self) -> &Item {
        &self.item
    }

    pub fn read(&self) -> Result<Vec<u8>, XwfError>{
        let size = self.get_logical_size()?;
        if size <= 0 {
            return Err(XwfError::InvalidItemSize);
        }
        let mut ret: Vec<u8> = Vec::with_capacity(size as usize);

        while let Some(mut data) = self.read_chunk(ret.len(), DEFAULT_DATA_CHUNK_SIZE as usize) {
            Application::should_stop()?;
            ret.append(&mut data);
        }

        if ret.len() == 0 {
            Err(XwfError::ReadItemDataFailed)
        } else {
            Ok(ret)
        }
    }

    pub fn read_chunk(&self, offset: usize, chunk_size: usize ) -> Option<Vec<u8>> {
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

        let mut current_offset = 0usize;

        while let Some(data) = self.read_chunk(current_offset, DEFAULT_DATA_CHUNK_SIZE as usize) {
            current_offset+=data.len();
            file.write_all(&data.as_slice()).map_err(|e| XwfError::IoError(e))?;
        }

        Ok(())
    }


}

unsafe impl Send for Item {}
unsafe impl Sync for Item {}