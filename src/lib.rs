mod xwf;

use std::collections::{HashMap, HashSet};
use std::ffi::c_void;
use bitflags::Flags;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{HANDLE, LONG};

use crate::xwf::api::application::Application;
use crate::xwf::api::case::{Case, ReportTable};
use crate::xwf::api::evidence::{Evidence, ReportTableMap};
use crate::xwf::api::item::{Item};
use crate::xwf::xwf_constants::{AddReportTableFlags, ItemInfoFlags, ProgressFlags};

use simplelog::{WriteLogger, LevelFilter, Config};
use log::{debug, error, info};
use crate::xwf::api::volume::{HashType, VolumeNameType};


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn XT_Init(nVersion: DWORD, nFlags: DWORD, hMainWnd: HANDLE, lpReserved: *mut c_void) -> LONG {
    let logger = WriteLogger::init(LevelFilter::Debug, Config::default(), Application::new());
    debug!("XT_Init called");
    1
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn XT_Done(lpReserved: *mut c_void)
    -> LONG {
    debug!("XT_Done called");
    0
}


#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn XT_About(hParentWnd: HANDLE, lpReserved: *mut c_void)
    -> LONG {
    debug!("XT_About called");




    0
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn XT_Prepare(hVolume: HANDLE, hEvidence: HANDLE,  nOpType: DWORD, lpReserved: *mut c_void
) -> LONG {
    debug!("XT_Prepare called");

    0
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn XT_Finalize(hVolume: HANDLE, hEvidence: HANDLE,  nOpType: DWORD, lpReserved: *mut c_void
) -> LONG {
    debug!("XT_Finalize called");

    0
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn XT_ProcessItem(nItemID: LONG,  lpReserved: *mut c_void) -> LONG {
    debug!("XT_ProcessItem called");
    0
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn XT_ProcessItemEx(nItemID: LONG, hItem: HANDLE,  lpReserved: *mut c_void) -> LONG {
    debug!("XT_ProcessItemEx called");
    0
}

