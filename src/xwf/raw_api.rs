use std::ffi::CStr;
use std::mem::transmute_copy;
use std::ptr::null_mut;
use cstr::cstr;
use winapi::shared::minwindef::{FARPROC, HMODULE};
use winapi::um::libloaderapi::{GetProcAddress, GetModuleHandleA};

use crate::xwf::xwf_function_types::*;

use once_cell::sync::Lazy;

pub struct RawApi {
    pub output_message: FnXwfOutputMessage,
    pub get_volume_name: FnXwfGetVolumeName,
    pub get_volume_information: FnXwfGetVolumeInformation,
    pub set_item_information: FnXwfSetItemInformation,
    pub get_item_information : FnXwfGetItemInformation,
    pub get_item_parent : FnXwfGetItemParent,
    pub get_item_size : FnXwfGetItemSize,
    pub get_item_type: FnXwfGetItemType,
    pub set_item_type: FnXwfSetItemType,
    pub read : FnXwfRead,
    pub get_size : FnXwfGetSize,
    pub get_first_ev_obj: FnXwfGetFirstEvObj,
    pub get_next_ev_obj: FnXwfGetNextEvObj,
    pub get_case_prop: FnXwfGetCaseProp,
    pub get_ev_obj_prop: FnXwfGetEvObjProp,
    pub get_ev_obj: FnXwfGetEvObj,
    pub get_report_table_info: FnXwfGetReportTableInfo,
    pub get_ev_obj_report_table_assocs: FnXwfGetEvObjReportTableAssocs,
    pub open_ev_obj: FnXwfOpenEvObj,
    pub add_to_report_table: FnXwfAddToReportTable,
    pub get_hash_value: FnXwfGetHashValue,
    pub set_hash_value: FnXwfSetHashValue,
    pub open_item: FnXwfOpenItem,
    pub get_vs_prop: FnXwfGetVsprop,
    pub select_volume_snapshot: FnXwfSelectVolumeSnapshot,
    pub get_item_name: FnXwfGetItemName,
    pub get_item_count: FnXwfGetItemCount,
    pub get_prop: FnXwfGetProp,
    pub close_ev_obj: FnXwfCloseEvObj,
    pub close: FnXwfClose,
    pub show_progress: FnXwfShowProgress,
    pub set_progress_description: FnXwfSetProgressDescription,
    pub set_progress_percentage: FnXwfSetProgressPercentage,
    pub hide_progress: FnXwfHideProgress,
    pub should_stop: FnXwfShouldStop
}

impl RawApi {

    fn load_method<T>(h_module: HMODULE, function_name: &CStr) -> Result<T, &'static str> {
        unsafe {
            let adr = GetProcAddress(h_module, function_name.as_ptr());
            if adr == null_mut() {
                return Err("could not load function")
            }
            Ok(transmute_copy::<FARPROC, T>(&adr))
        }

    }

    pub fn load_no_error() -> RawApi {
        RawApi::load().expect("unable to load RawApi")
    }
    pub fn load() -> Result<RawApi, &'static str> {
        unsafe {
            let h_module = GetModuleHandleA(std::ptr::null());
            if h_module == null_mut() {
                return Err("could not load module")
            }

            Ok(RawApi {
               output_message: RawApi::load_method(h_module, cstr!(XWF_OutputMessage))?,
               get_volume_name: RawApi::load_method(h_module, cstr!(XWF_GetVolumeName))?,
               get_volume_information: RawApi::load_method(h_module, cstr!(XWF_GetVolumeInformation))?,
               set_item_information: RawApi::load_method(h_module, cstr!(XWF_SetItemInformation))?,
               get_item_information: RawApi::load_method(h_module, cstr!(XWF_GetItemInformation))?,
               get_item_parent: RawApi::load_method(h_module, cstr!(XWF_GetItemParent))?,
               get_item_size: RawApi::load_method(h_module, cstr!(XWF_GetItemSize))?,
               get_item_type: RawApi::load_method(h_module, cstr!(XWF_GetItemType))?,
               set_item_type:  RawApi::load_method(h_module, cstr!(XWF_SetItemType))?,
               read: RawApi::load_method(h_module, cstr!(XWF_Read))?,
               get_size: RawApi::load_method(h_module, cstr!(XWF_GetSize))?,
               get_first_ev_obj: RawApi::load_method(h_module, cstr!(XWF_GetFirstEvObj))?,
               get_next_ev_obj: RawApi::load_method(h_module, cstr!(XWF_GetNextEvObj))?,
               get_case_prop: RawApi::load_method(h_module, cstr!(XWF_GetCaseProp))?,
               get_ev_obj_prop: RawApi::load_method(h_module, cstr!(XWF_GetEvObjProp))?,
               get_ev_obj: RawApi::load_method(h_module, cstr!(XWF_GetEvObj))?,
               get_report_table_info: RawApi::load_method(h_module, cstr!(XWF_GetReportTableInfo))?,
               get_ev_obj_report_table_assocs: RawApi::load_method(h_module, cstr!(XWF_GetEvObjReportTableAssocs))?,
               open_ev_obj: RawApi::load_method(h_module, cstr!(XWF_OpenEvObj))?,
               add_to_report_table: RawApi::load_method(h_module, cstr!(XWF_AddToReportTable))?,
               get_hash_value: RawApi::load_method(h_module, cstr!(XWF_GetHashValue))?,
               set_hash_value: RawApi::load_method(h_module, cstr!(XWF_SetHashValue))?,
               open_item: RawApi::load_method(h_module, cstr!(XWF_OpenItem))?,
               get_vs_prop: RawApi::load_method(h_module, cstr!(XWF_GetVSProp))?,
               select_volume_snapshot: RawApi::load_method(h_module, cstr!(XWF_SelectVolumeSnapshot))?,
               get_item_name: RawApi::load_method(h_module, cstr!(XWF_GetItemName))?,
               get_item_count: RawApi::load_method(h_module, cstr!(XWF_GetItemCount))?,
               get_prop: RawApi::load_method(h_module, cstr!(XWF_GetProp))?,
               close_ev_obj: RawApi::load_method(h_module, cstr!(XWF_CloseEvObj))?,
               close: RawApi::load_method(h_module, cstr!(XWF_Close))?,
               show_progress: RawApi::load_method(h_module, cstr!(XWF_ShowProgress))?,
               set_progress_description: RawApi::load_method(h_module, cstr!(XWF_SetProgressDescription))?,
               set_progress_percentage: RawApi::load_method(h_module, cstr!(XWF_SetProgressPercentage))?,
               hide_progress: RawApi::load_method(h_module, cstr!(XWF_HideProgress))?,
               should_stop: RawApi::load_method(h_module, cstr!(XWF_ShouldStop))?,

            })

        }


    }
}


pub static RAW_API: Lazy<RawApi> = Lazy::new(RawApi::load_no_error);
