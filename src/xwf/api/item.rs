use std::ptr::null_mut;
use bitflags::Flags;

use winapi::shared::minwindef::{BOOL, DWORD, LPVOID};
use winapi::shared::ntdef::{HANDLE, LPWSTR};
use crate::xwf::api::util::{string_to_wchar_cstr, wchar_ptr_to_string};
use crate::xwf::api::error::XwfError;
use crate::xwf::raw_api::{RAW_API};
use crate::xwf::api::volume::{HashType, Volume};
use crate::xwf::xwf_constants::{AddReportTableFlags, ItemInfoFlags, OpenItemFlags, PropType};

#[derive(Copy, Clone)]
pub struct Item {
    item_id: i32,
}

impl Item {
    pub fn new(item_id: i32) -> Item {
        Item {
            item_id,
        }
    }

    pub fn open(&self, volume: &Volume, flags: Vec<OpenItemFlags>) -> Result<ItemHandle, XwfError> {
        let handle = (RAW_API.open_item)(
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

        let ret = (RAW_API.get_hash_value)(self.item_id, buf.as_mut_ptr() as LPVOID);

        if ret != 0 {
            buf.resize(hash_size, 0u8);
            return Some(buf);
        } else {
            return None;
        }
    }

    pub fn get_item_info_flags(&self) -> Option<ItemInfoFlags> {
        let mut success: Box<BOOL> = Box::new(1);
        let success_ptr: *mut BOOL = &mut *success;
        let result = (RAW_API.get_item_information)(self.item_id, 0x3, success_ptr);

        if *success != 0 {
            Some(ItemInfoFlags::from_bits_truncate(result as u64))
        } else {
            None
        }
    }

    pub fn get_size(&self) -> usize {
        (RAW_API.get_item_size)(self.item_id) as usize
    }

    pub fn get_name(&self) -> String {
        let wchr_ptr = (RAW_API.get_item_name)(self.item_id as DWORD);
        wchar_ptr_to_string(wchr_ptr)
    }

    pub fn add_to_report_table(&self, name: &String, flags: AddReportTableFlags) {
        let wchar_c_str = string_to_wchar_cstr(&name);
        (RAW_API.add_to_report_table)(self.item_id, wchar_c_str, flags.bits());
    }
}



pub struct ItemHandle {
    item_handle: HANDLE,
    item: Item
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
        (RAW_API.get_prop)(self.item_handle, prop_type as DWORD, null_mut())
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
        (RAW_API.close)(self.item_handle);
    }

    pub fn read(&self) {
        
    }


}