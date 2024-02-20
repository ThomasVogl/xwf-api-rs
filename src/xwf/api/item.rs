use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::ops::BitOr;
use std::path::Path;
use std::ptr::null_mut;
use std::str::FromStr;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use std::hash::{Hash, Hasher};

use winapi::shared::minwindef::{BOOL, DWORD, LPVOID, PDWORD};
use winapi::shared::ntdef::{HANDLE, LPWSTR};
use serde::{Deserialize, Serialize};
use crate::get_raw_api;
use crate::xwf::api::util::{string_to_wchar_cstr, wchar_ptr_to_string, wchar_str_to_string, wchar_str_to_string_expect_term};
use crate::xwf::api::error::XwfError;
use crate::xwf::api::evidence::Evidence;
use crate::xwf::api::traits::NativeHandle;
use crate::xwf::raw_api::RAW_API;
use crate::xwf::api::volume::{HashType, Volume};
use crate::xwf::xwf_types::{AddReportTableFlags, FileFormatConsistency, FileTypeCategory, FileTypeStatus, ItemInfoClassification, ItemInfoDeletion, ItemInfoFlags, ItemTypeFlags, OpenItemFlags, PropType, XwfDateTime, XwfItemInfoTypes};

use super::util::char_ptr_to_string;

const CHUNK_SIZE: i64 = 65536;

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


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct UniqueItemId {
    pub item_id: i32,
    pub evidence_id: u32,
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
    pub fn new(evidence_id: u32, item_id: u32) -> UniqueItemId {
        UniqueItemId {
            item_id: item_id as i32,
            evidence_id: evidence_id,
        }
    }
    pub fn item(&self) -> Item {
        Item::new(self.item_id)
    }

    pub fn evidence(&self) -> Evidence {
        Evidence::new((get_raw_api!().get_ev_obj)(self.evidence_id)).unwrap()
    }
}

impl From<i64> for UniqueItemId {
    fn from(value: i64) -> Self {
        UniqueItemId { 
            item_id: (value & 0x00000000FFFFFFFFi64) as i32 , 
            evidence_id: (value >> 32) as u32 
        }
    }
}

impl Into<i64> for UniqueItemId {
    fn into(self) -> i64 {
        (self.evidence_id as i64) << 32 | (self.item_id as i64)
    }
}

impl Display for UniqueItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.evidence_id, self.item_id)
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



    pub fn open(&self, volume: &Volume, flags: Vec<OpenItemFlags>) -> Result<ItemHandle, XwfError> {
        let handle = (get_raw_api!().open_item)(
            volume.handle(),
            self.item_id,
            OpenItemFlags::from_iter(flags).bits());

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

    pub fn set_hash_value(&self, hash_value: &Vec<u8>, set_secondary: bool) -> Result<(), ()>{
        let mut param: u32 = 1;

        if set_secondary {
            param = 2;
        }

        let x = (get_raw_api!().set_hash_value)(self.item_id, hash_value.as_ptr() as LPVOID, param);

        if x != 0 {
            Ok(())
        } else {
            Err(())
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
                    let time = NaiveDateTime::from_timestamp_opt(unix_epoch_sec, nsec_fraction)?;
                    return Some(XwfDateTime::NoTimezone(time));
                } else {
                    let time = Utc.timestamp_opt(unix_epoch_sec, nsec_fraction).unwrap();

                    if local_time {
                        return Some(XwfDateTime::Local(DateTime::from(time)));
                    } else {
                        return Some(XwfDateTime::Utc(time));

                    }
                }
            },
            Err(_) => {
                return None;
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
        wchar_ptr_to_string(wchr_ptr)
    }

    pub fn get_path(&self) -> String {

        let mut path_components: Vec<String> = self.iter().map(|p| p.get_name()).collect();
        path_components.pop();
        path_components.reverse();

        "\\".to_string() + &path_components.join("\\")
    }

    pub fn add_to_report_table(&self, name: &String, flags: AddReportTableFlags) {
        let wchar_c_str = string_to_wchar_cstr(&name);
        (get_raw_api!().add_to_report_table)(self.item_id, wchar_c_str, flags.bits());
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

        Ok(wchar_str_to_string(buf.as_slice()))
    }
    pub fn get_report_tables(&self) -> Result<Vec<String>, XwfError> {
        let mut buf = [0u16; 4096];
        let num_assocs = (get_raw_api!().get_report_table_assocs)(self.item_id, buf.as_mut_ptr(), buf.len() as i32);

        if num_assocs == 0 {
            return Ok(Vec::new());
        }

        let assocs = wchar_str_to_string_expect_term(&buf).ok_or(XwfError::GivenBufferToSmallForContent)?;
        let vec_assocs: Vec<String> = assocs.split(", ").map(|s| String::from_str(s).unwrap()).collect();

        if vec_assocs.len() != num_assocs as usize {
            Err(XwfError::GivenBufferToSmallForContent)
        } else {
            Ok(vec_assocs)
        }

    }

    pub fn get_hash_sets(&self) -> Result<Vec<String>, XwfError> {
        let mut buf = [0u16; 4096];
        let num_assocs = (get_raw_api!().get_hashset_assocs)(self.item_id, buf.as_mut_ptr(), buf.len() as i32);

        if num_assocs < 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_hashset_assocs"));
        }

        if num_assocs == 0 {
            return Ok(Vec::new());
        }

        let assocs = wchar_str_to_string_expect_term(&buf).ok_or(XwfError::GivenBufferToSmallForContent)?;
        let vec_assocs: Vec<String> = assocs.split(", ").map(|s| String::from_str(s).unwrap()).collect();


        if vec_assocs.len() != num_assocs as usize {
            Err(XwfError::GivenBufferToSmallForContent)
        } else {
            Ok(vec_assocs)
        }

    }

    pub fn get_extracted_metadata(&self) -> Option<Vec<String>>{
        let ptr = (get_raw_api!().get_extracted_metadata)(self.item_id);
        
        if ptr == null_mut() {
            None
        } else {
            Some(wchar_ptr_to_string(ptr).replace("\r", "")
            .split("\n").filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .collect())
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
                FileTypeCategory::from(wchar_str_to_string(buf.as_slice()))
            ),
        )
    }

    pub fn unique_id(&self, evidence: &Evidence) -> UniqueItemId {
        UniqueItemId {
            item_id: self.item_id,
            evidence_id: evidence.get_id()
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
                    metadata_str = wchar_ptr_to_string(ptr as LPWSTR);
                }
                
                (get_raw_api!().release_mem)(ptr);

                return Some(metadata_str.split('\n').map(|s| s.to_string()).collect());
            }
        }
    }

    pub fn get_prop(&self, prop_type: PropType) -> i64 {
        (get_raw_api!().get_prop)(self.item_handle, prop_type as DWORD, null_mut())
    }

    pub fn get_name(&self) -> String {
        wchar_ptr_to_string(self.get_prop(PropType::PointerName) as LPWSTR)
    }

    pub fn get_path(&self) -> String {
        wchar_ptr_to_string(self.get_prop(PropType::PointerFilePath) as LPWSTR)
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

        let mut num_bytes_to_read = size;


        let mut ret: Vec<u8> = Vec::with_capacity(size as usize);
        ret.resize(size as usize, 0u8);

        while num_bytes_to_read  > 0 {

            let bytes_read = size-num_bytes_to_read;
            

            let mut chunk_size = CHUNK_SIZE;

            if chunk_size > num_bytes_to_read {
                chunk_size = num_bytes_to_read;
            }
            let buf_ptr;
            unsafe {
                buf_ptr = ret.as_mut_ptr().add((size-num_bytes_to_read) as usize);
            }

            let r = (get_raw_api!().read)(self.item_handle, bytes_read, buf_ptr, chunk_size as DWORD);

            if r<= 0 {
                return Err(XwfError::ReadItemDataFailed);
            }

            num_bytes_to_read-=r as i64;
        }

        Ok(ret)
    }


    pub fn write_to_file(&self, dest: &Path) -> Result<(), XwfError>{

        let mut file = File::create(dest).map_err(|e| XwfError::IoError(e) )?;

        let size = self.get_logical_size()?;

        let mut num_bytes_to_read = size;


        while num_bytes_to_read  > 0 {

            let bytes_read = size-num_bytes_to_read;

            let mut chunk_size = CHUNK_SIZE;

            if chunk_size > num_bytes_to_read {
                chunk_size = num_bytes_to_read;
            }

            let mut buf: Vec<u8> = Vec::new();
            buf.resize(CHUNK_SIZE as usize, 0);


            let r = (get_raw_api!().read)(self.item_handle, bytes_read, buf.as_mut_ptr(), chunk_size as DWORD);

            if r<= 0 {
                return Err(XwfError::ReadItemDataFailed);
            }

            if r != CHUNK_SIZE as u32 {
                buf.resize(r as usize, 0);
            }

            file.write_all(buf.as_slice()).map_err(|e| XwfError::IoError(e))?;


            num_bytes_to_read-=r as i64;
        }

        Ok(())
    }


}

unsafe impl Send for Item {}
unsafe impl Sync for Item {}