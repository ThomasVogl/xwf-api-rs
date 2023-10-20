use std::ptr::{null, null_mut};
use bitflags::Flags;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::ntdef::{LONG, LPWSTR, PLONG};
use crate::get_raw_api;
use crate::xwf::api::evidence::Evidence;
use crate::xwf::api::util::{wchar_ptr_to_string, wchar_str_to_string};
use crate::xwf::api::error::XwfError;
use crate::xwf::xwf_types::ReportTableFlags;
use crate::xwf::raw_api::RAW_API;

pub struct ReportTable {
    pub name: String,
    pub id: u16,
    pub flags: ReportTableFlags,
}

impl ReportTable {
    pub fn find_by_name(list: &Vec<ReportTable>, name: String) -> Option<&ReportTable> {
        for i in list {
            if i.name == name {
                return Some(i);
            }
        }
        None
    }

    pub fn find_by_id(list: &Vec<ReportTable>, id: u16) -> Option<&ReportTable> {
        for i in list {
            if i.id == id {
                return Some(&i);
            }
        }
        None
    }
}
pub struct Case {

}

pub struct CaseInfo {
    pub id: i64,
    pub creation: i64,
    pub title: String,
    pub examiner: String,
    pub file: String,
    pub dir: String,

}



impl Case {

    pub fn get_case_infos() -> CaseInfo {
        let mut buf = [0u16; 256];


        let id = (get_raw_api!().get_case_prop)(null_mut(), 0, null_mut(), 0);
        let creation = (get_raw_api!().get_case_prop)(null_mut(), 2, null_mut(), 0);

        let _ = (get_raw_api!().get_case_prop)(null_mut(), 3, buf.as_mut_ptr() as LPVOID, buf.len() as LONG);
        let examiner = wchar_str_to_string(&buf);

        let _ = (get_raw_api!().get_case_prop)(null_mut(), 1, buf.as_mut_ptr() as LPVOID, buf.len() as LONG);
        let title = wchar_str_to_string(&buf);

        let _ = (get_raw_api!().get_case_prop)(null_mut(), 5, buf.as_mut_ptr() as LPVOID, buf.len() as LONG);
        let file = wchar_str_to_string(&buf);

        let _ = (get_raw_api!().get_case_prop)(null_mut(), 6, buf.as_mut_ptr() as LPVOID, buf.len() as LONG);
        let dir = wchar_str_to_string(&buf);


        CaseInfo {
            id,
            creation,
            examiner,
            title,
            file,
            dir
        }

    }



    pub fn get_ev_obj(obj_id: u32) -> Option<Evidence> {
        Evidence::new((get_raw_api!().get_ev_obj)(obj_id))
    }

    pub fn get_report_tables() -> Vec<ReportTable> {
        let mut optional: LONG = 0;
        let mut ret: Vec<ReportTable> = Vec::new();
        //get num of report tables
        (get_raw_api!().get_report_table_info)(null_mut(), -1, &mut optional as PLONG);

        let num_tables = optional;


        for i in 0..num_tables {
            optional = 0;
            let wstr_ptr = (get_raw_api!().get_report_table_info)(null_mut(), i, &mut optional as PLONG) as LPWSTR;
            if wstr_ptr != null_mut() {
                ret.push(ReportTable {
                    name: wchar_ptr_to_string(wstr_ptr),
                    id: i as u16,
                    flags: ReportTableFlags::from_bits_truncate(optional as u32),

                });
            }

        }
        return ret;

    }
}