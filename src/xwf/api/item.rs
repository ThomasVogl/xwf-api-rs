use std::fmt::{Display, Formatter};
use std::ops::BitAnd;
use std::ptr::null_mut;
use bitflags::Flags;
use log::debug;

use winapi::shared::minwindef::{BOOL, DWORD, LPVOID};
use winapi::shared::ntdef::{HANDLE, LPWSTR};
use crate::get_raw_api;
use crate::xwf::api::application::Application;
use crate::xwf::api::util::{string_to_wchar_cstr, wchar_ptr_to_string, wchar_str_to_string};
use crate::xwf::api::error::XwfError;
use crate::xwf::api::evidence::Evidence;
use crate::xwf::api::traits::NativeHandle;
use crate::xwf::raw_api::{RAW_API};
use crate::xwf::api::volume::{HashType, Volume};
use crate::xwf::xwf_types::{AddReportTableFlags, FileFormatConsistency, ItemInfoFlags, ItemTypeFlags, OpenItemFlags, PropType, FileTypeStatus, FileTypeCategory};

#[derive(Copy, Clone, Debug)]
pub struct Item {
    item_id: i32,
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UniqueItemId {
    pub item_id: i32,
    pub evidence_id: u32,
}

impl Display for UniqueItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.evidence_id, self.item_id)
    }
}

impl Item {
    pub fn new(item_id: i32) -> Item {
        Item {
            item_id,
        }
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

    pub fn get_item_info_flags(&self) -> Option<ItemInfoFlags> {
        let mut success: Box<BOOL> = Box::new(1);
        let success_ptr: *mut BOOL = &mut *success;
        let result = (get_raw_api!().get_item_information)(self.item_id, 0x3, success_ptr);

        if *success != 0 {
            Some(ItemInfoFlags::from_bits_truncate(result as u64))
        } else {
            None
        }
    }

    pub fn get_size(&self) -> usize {
        (get_raw_api!().get_item_size)(self.item_id) as usize
    }

    pub fn get_name(&self) -> String {
        let wchr_ptr = (get_raw_api!().get_item_name)(self.item_id as DWORD);
        wchar_ptr_to_string(wchr_ptr)
    }

    pub fn add_to_report_table(&self, name: &String, flags: AddReportTableFlags) {
        let wchar_c_str = string_to_wchar_cstr(&name);
        (get_raw_api!().add_to_report_table)(self.item_id, wchar_c_str, flags.bits());
    }

    pub fn get_item_type(&self, long_desc: bool) -> Result<(FileTypeStatus, FileFormatConsistency, String), ()> {


        let mut buf = [0u16; 256];



        let mut flags = ItemTypeFlags::ReceiveTypeStatus;

        if long_desc {
            flags.set(ItemTypeFlags::TextualDescriptionType, true);
        }

        let buf_and_flags = (buf.len() as u32 )& flags.bits();
        let status = (get_raw_api!().get_item_type)(self.item_id, buf.as_mut_ptr(), buf_and_flags);

        Ok(
            (   FileTypeStatus::try_from(status)?,
                FileFormatConsistency::try_from(status)?,
                wchar_str_to_string(buf.as_slice())
            ),
           )
    }
    pub fn get_item_category(&self) -> Result<(FileTypeStatus, FileFormatConsistency, FileTypeCategory), ()> {
        let mut buf = [0u16; 256];

        let flags = ItemTypeFlags::ReceiveTypeStatus | ItemTypeFlags::TextualDescriptionCategory;

        let buf_and_flags = (buf.len() as u32) | flags.bits();
        let status = (get_raw_api!().get_item_type)(self.item_id, buf.as_mut_ptr(), buf_and_flags);

        if buf[0] == 0 {
            return Err(());
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

    pub fn get_prop(&self, prop_type: PropType) -> i64 {
        (get_raw_api!().get_prop)(self.item_handle, prop_type as DWORD, null_mut())
    }

    pub fn get_name(&self) -> String {
        wchar_ptr_to_string(self.get_prop(PropType::PointerName) as LPWSTR)
    }

    pub fn get_path(&self) -> String {
        wchar_ptr_to_string(self.get_prop(PropType::PointerFilePath) as LPWSTR)
    }

    pub fn get_logical_size(&self) -> i64 {
        self.get_prop(PropType::LogicalSize)
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

    pub fn read(&self) -> Result<Vec<u8>, ()>{
        let size = self.get_logical_size();

        let mut num_bytes_to_read = size;

        if size <= 0 {
            return Err(());
        }

        let mut ret: Vec<u8> = Vec::with_capacity(size as usize);
        ret.resize(size as usize, 0u8);

        while num_bytes_to_read  > 0 {

            let bytes_read = size-num_bytes_to_read;
            let mut buf_ptr: *mut u8 = null_mut();

            let mut chunk_size = 1024 * 1024;

            if chunk_size > num_bytes_to_read {
                chunk_size = num_bytes_to_read;
            }

            unsafe {
                buf_ptr = ret.as_mut_ptr().add((size-num_bytes_to_read) as usize);
            }

            let r = (get_raw_api!().read)(self.item_handle, bytes_read, buf_ptr, chunk_size as DWORD);

            if r<= 0 {
                return Err(());
            }

            num_bytes_to_read-=r as i64;


        }

        Ok(ret)
    }


}

unsafe impl Send for Item {}
unsafe impl Sync for Item {}