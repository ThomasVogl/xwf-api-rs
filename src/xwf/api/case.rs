use std::ptr::null_mut;
use bitflags::Flags;
use winapi::shared::ntdef::{LONG, LPWSTR, PLONG};
use crate::xwf::api::evidence::Evidence;
use crate::xwf::api::util::wchar_ptr_to_string;
use crate::xwf::api::error::XwfError;
use crate::xwf::xwf_constants::ReportTableFlags;
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



impl Case {



    pub fn get_ev_obj(obj_id: u32) -> Result<Evidence, XwfError> {
        Evidence::new((RAW_API.get_ev_obj)(obj_id))
    }

    pub fn get_report_tables() -> Vec<ReportTable> {
        let mut optional: LONG = 0;
        let mut ret: Vec<ReportTable> = Vec::new();
        //get num of report tables
        (RAW_API.get_report_table_info)(null_mut(), -1, &mut optional as PLONG);

        let num_tables = optional;


        for i in 0..num_tables {
            optional = 0;
            let wstr_ptr = (RAW_API.get_report_table_info)(null_mut(), i, &mut optional as PLONG) as LPWSTR;
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