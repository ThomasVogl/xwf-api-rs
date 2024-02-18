use std::collections::HashMap;
use std::ptr::{null, null_mut};
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{HANDLE, LONG};
use winapi::um::winnt::{LPWSTR, PVOID};
use crate::get_raw_api;
use crate::xwf::api::volume::Volume;
use crate::xwf::api::error::XwfError;
use crate::xwf::api::util::{wchar_ptr_to_string, wchar_str_to_string};
use crate::xwf::raw_api::RAW_API;
use crate::xwf::xwf_types::{EvObjPropFlags, EvObjPropType};

#[derive(Clone)]
pub struct Evidence {
    evidence_handle: HANDLE,
    child_evidence_id: Option<u32>,
}

#[repr(packed(1))]
#[derive(Copy,Clone)]
struct ReportTableListItemPacked {
    pub report_table_id: u16,
    pub item_id: u32,
}
pub struct ReportTableListItem {
    pub report_table_id: u16,
    pub item_id: u32,
}




impl ReportTableListItem {
    pub fn insert_to_map(ptr: *const u8, num_elements: i32, table_map: &mut ReportTableMap) {
        let list_ptr = ptr as *const ReportTableListItemPacked;
        let mut idx: i32 = 0;
        while idx < num_elements {
            unsafe {
                let item = *(list_ptr.add(idx as usize));
                let report_table_id = item.report_table_id;
                let item_id = item.item_id;
                match table_map.get_mut(&report_table_id) {
                    Some(x) => { x.push(item_id); },
                    None => {table_map.insert(report_table_id, vec![item_id]);}

                }
            }
            idx+=1;

        }
    }
}

pub type ReportTableMap = HashMap<u16, Vec<u32>>;

pub struct EvidenceIterator {
    current_ev: HANDLE,
}

impl Iterator for EvidenceIterator {
    type Item = Evidence;

    fn next(&mut self) -> Option<Self::Item> {

        let ev = Evidence::new(self.current_ev);

        if self.current_ev != null_mut() {
            let next_ev_obj = (get_raw_api!().get_next_ev_obj)(self.current_ev, null_mut());
            self.current_ev = next_ev_obj;
        }

        ev
        
    }
}




impl Evidence {
    pub fn new(evidence_handle: HANDLE) -> Option<Evidence> {
        if evidence_handle == null_mut() {
            return None
        }

        Some(Evidence{
            evidence_handle,
            child_evidence_id: None,
        })  
    }

    pub fn iter(&self) -> EvidenceIterator {
        EvidenceIterator { current_ev: self.handle() }
    }

    pub fn handle(&self) -> HANDLE {
        return self.evidence_handle
    }

    pub fn open(&self) -> Result<Volume, XwfError> {
        let handle = (get_raw_api!().open_ev_obj)(self.evidence_handle, 0);
        Volume::new(handle)
    }

    pub fn get_first_evidence() -> Option<Evidence> {
        let first_ev_obj = (get_raw_api!().get_first_ev_obj)(null_mut());

        if first_ev_obj == null_mut() {
            return None;
        }

        Evidence::new(first_ev_obj)
    }

    pub fn get_evidences() -> Option<Vec<Evidence>> {
        let mut ret: Vec<Evidence> = Vec::new();
        let mut parent_ids: Vec<u32> = Vec::new();

        let mut ev_iterator = Evidence::get_first_evidence()?.iter();

        while let Some(evidence) = ev_iterator.next() {
    
            let opt_parent_id = evidence.get_parent_id();
            if opt_parent_id.is_some() {
                parent_ids.push(opt_parent_id.unwrap())
            }
            ret.push(evidence);
        }

        for pid in parent_ids {
            for i in &mut ret {
                if i.get_id() == pid {
                    i.set_child(pid)
                }
            }
        }

        Some(ret)
       }

    pub fn close(&self) {
        (get_raw_api!().close_ev_obj)(self.evidence_handle);
    }

    pub fn get_report_table_assocs(&self, sorted: bool) -> Option<ReportTableMap> {
        let mut flags: LONG = 0;
        let mut num_pairs: LONG = 0;
        let mut ret = ReportTableMap::new();
        if sorted { flags = 0x1; }

        let ptr_list = (get_raw_api!().get_ev_obj_report_table_assocs)(self.evidence_handle,flags,&mut num_pairs) as *const u8;

        if ptr_list == null() {
            return None;
        }

        ReportTableListItem::insert_to_map(ptr_list, num_pairs, &mut ret);
        Some(ret)
    }

    pub fn get_id(&self) -> u32 {
        let ret = (get_raw_api!().get_ev_obj_prop)(self.evidence_handle, EvObjPropType::ObjId as DWORD, null_mut());
        ret as u32
    }

    pub fn get_parent_id(&self) -> Option<u32> {
        let ret = (get_raw_api!().get_ev_obj_prop)(self.evidence_handle, EvObjPropType::ParentObjId as DWORD, null_mut());
        if ret > 0 { Some(ret as u32)}
        else { None }

    }

    pub fn get_parent(&self) -> Option<Evidence> {
        let ret = (get_raw_api!().get_ev_obj_prop)(self.evidence_handle, EvObjPropType::ParentObjId as DWORD, null_mut());
        if ret > 0 {
            Evidence::get_ev_obj(ret as u32)
        } else {
            None
        }
    }

    pub fn get_ev_obj(id: u32) -> Option<Evidence> {
        let handle = (get_raw_api!().get_ev_obj)(id) ;
        if  handle != null_mut() {
            Some(Evidence::new(handle).unwrap())
        } else {
            None
        }
    }

    pub fn get_flags(&self) -> EvObjPropFlags {
        let ret = (get_raw_api!().get_ev_obj_prop)(self.evidence_handle, EvObjPropType::Flags as DWORD, null_mut());
        EvObjPropFlags::from_bits_truncate(ret as u32)
    }

    pub fn get_name(&self) -> Result<String, XwfError> {
        let mut buf = [0u16; 256];
        let ret = (get_raw_api!().get_ev_obj_prop)(self.evidence_handle, EvObjPropType::AbbrevObjTitle as DWORD, buf.as_mut_ptr() as PVOID);
        if ret == -1 {
            Err(XwfError::XwfFunctionCallFailed("get_ev_obj_prop"))
        } else {
            Ok(wchar_str_to_string(&buf))
        }
    }

    pub fn get_description(&self) -> Option<String> {
        let ret = (get_raw_api!().get_ev_obj_prop)(self.evidence_handle, EvObjPropType::Description as DWORD, null_mut() as PVOID);
        if ret == -1 || ret == 0 {
            None
        } else {
            Some(wchar_ptr_to_string(ret as LPWSTR))
        }

    }

    pub fn get_comments(&self) -> Option<String> {
        let ret = (get_raw_api!().get_ev_obj_prop)(self.evidence_handle, EvObjPropType::ExaminerComments as DWORD, null_mut() as PVOID);
        if ret == -1 || ret == 0 {
            None
        } else {
            Some(wchar_ptr_to_string(ret as LPWSTR))
        }
    }

    pub fn child(&self) -> Option<Evidence> {
        if self.child_evidence_id.is_none() {
            return None;
        }
        Evidence::get_ev_obj(self.child_evidence_id.unwrap())
    }

    pub fn has_child(&self) -> bool {
        self.child_evidence_id.is_some()
    }

    pub fn set_child(&mut self, evidence_id: u32) {
        self.child_evidence_id = Some(evidence_id)
    }
}

unsafe impl Send for Evidence {}
unsafe impl Sync for Evidence {}