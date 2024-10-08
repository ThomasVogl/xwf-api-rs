use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ptr::null_mut;
use serde::Serialize;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::ntdef::{LONG, LPWSTR, PLONG};
use chrono::{DateTime, Utc};
use winsafe::WString;
use crate::application::Application;
use crate::get_raw_api;
use crate::evidence::{Evidence, EvidenceIterator};
use crate::item::Item;
use crate::error::XwfError;
use crate::xwf_types::*;
use crate::raw_api::RAW_API;



#[derive(Clone, Serialize, Debug)]
pub struct ReportTable {
    pub name: String,
    pub id: u16,
    pub flags: ReportTableFlags,
}

impl PartialEq for ReportTable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ReportTable {}

impl Hash for ReportTable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
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

#[derive(Clone)]
pub struct Case {
    report_tables: HashMap<u16, ReportTable>,
    report_tables_by_name: HashMap<String, ReportTable>,
    report_table_map: HashMap<u32, HashMap<ReportTable, HashSet<u32>>>
}

pub struct CaseInfo {
    pub id: i64,
    pub creation_date: DateTime<Utc>,
    pub title: String,
    pub examiner: String,
    pub file: String,
    pub dir: String,

}



impl Case {

    pub fn new() -> Case {
        Case {
            report_tables: HashMap::new(),
            report_tables_by_name: HashMap::new(),
            report_table_map: HashMap::new(),
        }
    }

    pub fn iterate_ext<F, R>(&self, item_consumer: F) -> Result<Vec<R>, XwfError>
    where F: Fn(&Case, &Evidence, &Item) -> Result<R,XwfError> {
        let mut ret:Vec<R> = Vec::new();

        Application::show_progress("Iterating over all evidences and items", ProgressFlags::empty());

        let mut evidence_iterator = EvidenceIterator::new();
        while let Some(ev) = evidence_iterator.next() {

            let vol = ev.open()?;
            vol.select()?;

            // get number of elements within volume
            let num_items = vol.get_item_count()?;

            // set progress description
            Application::set_progress_description(format!("processing evidence \"{}\"", ev.get_name()?));
            Application::set_progress_percentage(0, num_items as u32);


            // iterate over all items (number == item id)
            for item_id in 0..num_items {
                Application::should_stop()?;

                let item = Item::new(item_id as i32);
                ret.push(item_consumer(self, &ev, &item)?);
                Application::set_progress_percentage((item_id+1) as u32, num_items as u32);
            }
        }

        Application::hide_progress();
        Ok(ret)
    }

    pub fn iterate<F, R>(item_consumer: F) -> Result<Vec<R>, XwfError>
    where F: Fn(Item) -> Result<R,XwfError> {

        let mut ret:Vec<R> = Vec::new();

        Application::show_progress("Iterating over all evidences and items", ProgressFlags::empty());

        let mut evidence_iterator = EvidenceIterator::new();
        while let Some(ev) = evidence_iterator.next() {

            let vol = ev.open()?;
            vol.select()?;

            // get number of elements within volume
            let num_items = vol.get_item_count()?;

            // set progress description
            Application::set_progress_description(format!("processing evidence \"{}\"", ev.get_name()?));
            Application::set_progress_percentage(0, num_items as u32);


            // iterate over all items (number == item id)
            for item_id in 0..num_items {
                Application::should_stop()?;

                ret.push(item_consumer(Item::new(item_id as i32))?);
                Application::set_progress_percentage((item_id+1) as u32, num_items as u32);
            }
        }

        Application::hide_progress();
        Ok(ret)
    }

    pub fn contained_in_report_table(&self, t: &Option<&ReportTable>, evidence: &Evidence, item: &Item) -> bool {
        t.and_then(|t| {
            self.report_table_map.get(&evidence.get_id()).and_then(|tables| {
                tables.get(t).and_then(|itemset| {
                    Some(itemset.contains(&(item.item_id as u32)))
                })
            })
        }).unwrap_or(false)
    }

    pub fn get_cached_report_tables(&self, evidence: &Evidence, item: &Item) -> Vec<&ReportTable> {
        
        self.report_table_map.get(&evidence.get_id()).and_then(|report_tables| {
            Some(report_tables.iter()
            .filter(|i| i.1.contains(&(item.item_id as u32)))
            .map(|i| i.0)
            .collect())
        }).unwrap_or(vec![])
    }

    pub fn get_report_table_by_name(&self, name: &str) -> Option<&ReportTable> {
        self.report_tables_by_name.get(name)
    }

    pub fn get_report_table_by_id(&self, id: &u16) -> Option<&ReportTable> {
        self.report_tables.get(id)
    }

    pub fn compute_report_table_cache(&mut self) -> Result<(), XwfError> {

        let ev = Evidence::get_first_evidence().ok_or(XwfError::NoEvidenceAvaible)?;
        
        for i in &Case::get_report_tables() {
            self.report_tables.insert(i.id, i.clone());
            self.report_tables_by_name.insert(i.name.clone(), i.clone());
        }

        
        ev.iter().for_each(|e| {
            e.get_report_table_assocs(false).and_then(|assocs| {
                for (table_id,v) in assocs {
                    let table = self.get_report_table_by_id(&table_id).unwrap().clone();
                    let evidence_id = e.get_id();
                    let table_map = match self.report_table_map.get_mut(&evidence_id) {
                        None => {
                            self.report_table_map.insert(evidence_id, HashMap::new());
                            self.report_table_map.get_mut(&evidence_id).unwrap()
                        },
                        Some(v) => {v}
                    };

                    let id_set = match table_map.get_mut(&table) {
                        None => {
                            table_map.insert(table.clone(), HashSet::new());
                            table_map.get_mut(&table).unwrap()
                        },
                        Some(v) => {v}
                    };

                    for item_id in v {
                        id_set.insert(item_id);
                    }
                }
                Some(())
            });
        });

        Ok(())
    }

    pub fn get_case_infos() -> Result<CaseInfo, XwfError> {
        let mut buf = [0u16; 256];


        let id = (get_raw_api!().get_case_prop)(null_mut(), 0, null_mut(), 0);

        let creation = (get_raw_api!().get_case_prop)(null_mut(), 2, null_mut(), 0);
        if creation <= 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_case_prop"));
        }

        let creation_date: DateTime<Utc> = DateTime::from_timestamp( creation / 10000000 - 11644473600, 0).ok_or(XwfError::InvalidInputArgument)?;
        
        let buf_len = (get_raw_api!().get_case_prop)(null_mut(), 3, buf.as_mut_ptr() as LPVOID, buf.len() as LONG);
        if buf_len < 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_case_prop"));
        }
        let examiner: String = WString::from_wchars_slice(&buf).to_string();

        let buf_len = (get_raw_api!().get_case_prop)(null_mut(), 1, buf.as_mut_ptr() as LPVOID, buf.len() as LONG);
        if buf_len < 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_case_prop"));
        }
        let title = WString::from_wchars_slice(&buf).to_string();

        let buf_len = (get_raw_api!().get_case_prop)(null_mut(), 5, buf.as_mut_ptr() as LPVOID, buf.len() as LONG);
        if buf_len < 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_case_prop"));
        }

        let file = WString::from_wchars_slice(&buf).to_string();

        let buf_len = (get_raw_api!().get_case_prop)(null_mut(), 6, buf.as_mut_ptr() as LPVOID, buf.len() as LONG);
        if buf_len < 0 {
            return Err(XwfError::XwfFunctionCallFailed("get_case_prop"));
        }
        let dir = WString::from_wchars_slice(&buf).to_string();


        Ok(CaseInfo {
            id,
            creation_date,
            examiner,
            title,
            file,
            dir
        })

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
                unsafe {
                    ret.push(ReportTable {
                        name: WString::from_wchars_nullt(wstr_ptr).to_string(),
                        id: i as u16,
                        flags: ReportTableFlags::from_bits_truncate(optional as u32),

                    });
                }
            }

        }
        return ret;

    }
}