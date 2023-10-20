use crate::xwf::raw_api::{RAW_API, RawApi};
#[macro_export]
macro_rules! export_xt_init {

    ($variable:ident) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub unsafe extern "C"  fn XT_Init(nVersion: DWORD, nFlags: DWORD, hMainWnd: HANDLE, lpReserved: PVOID) -> LONG {
            RAW_API.lock().unwrap().insert(RawApi::load_no_error());
            let logger = WriteLogger::init(LevelFilter::Debug, Config::default(), Application::new());
            let xtension_version = $variable.xtension_version();
            info!("X-Tension {}, Version {}.{}.{}", $variable.xtension_name(), xtension_version.0, xtension_version.1, xtension_version.2 );
            debug!("XT_Init called");
            $variable.xt_init(XtVersion::try_from(nVersion).unwrap(), XtInitFlags::from_bits_truncate(nFlags), Window::new(hMainWnd), XtLicenseInfo {}) as i32
        }
    }

}
#[macro_export]
macro_rules! export_xt_done {
    ($variable:ident) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub unsafe extern "C" fn XT_Done(lpReserved: PVOID)
            -> LONG {
            debug!("XT_Done called");
            $variable.xt_done();
            0
        }
    };
}
#[macro_export]
macro_rules! export_xt_about {
    ($variable:ident) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub unsafe extern "C" fn XT_About(hParentWnd: HANDLE, lpReserved: PVOID)
            -> LONG {
            debug!("XT_About called");
            $variable.xt_about(Window::new(hParentWnd));
            0
        }
    };
}
#[macro_export]
macro_rules! export_xt_prepare {
    ($variable:ident) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub unsafe extern "C" fn XT_Prepare(hVolume: HANDLE, hEvidence: HANDLE,  nOpType: DWORD, lpReserved: PVOID
        ) -> LONG {
            debug!("XT_Prepare called");

            let opt_op_type = XtPrepareOpType::try_from(nOpType);
            if opt_op_type.is_err() {
                error!("error in parsing nOpType argument");
                return XtPrepareReturn::Negative(XtPrepareNegativeReturn::JustCallXtFinalize).into();
            }

            $variable.xt_prepare(
                Volume::new(hVolume),
                Evidence::new(hEvidence),
                opt_op_type.unwrap()).into()

        }
    };
}
#[macro_export]
macro_rules! export_xt_finalize {
    ($variable:ident) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub unsafe extern "C" fn XT_Finalize(hVolume: HANDLE, hEvidence: HANDLE,  nOpType: DWORD, lpReserved: PVOID
        ) -> LONG {
            debug!("XT_Finalize called");
            let opt_op_type = XtPrepareOpType::try_from(nOpType);
            if opt_op_type.is_err() {
                error!("error in parsing nOpType argument");
                return XtFinalizeReturn::Ok.into();
            }

            $variable.xt_finalize(
                Volume::new(hVolume),
                Evidence::new(hEvidence),
                opt_op_type.unwrap()).into()
        }
    };
}
#[macro_export]
macro_rules! export_xt_process_item {
    ($variable:ident) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub unsafe extern "C" fn XT_ProcessItem(nItemID: LONG,  lpReserved: PVOID) -> LONG {
            let item = Item::new(nItemID);

            $variable.xt_process_item(item).into()
        }
    };
}
#[macro_export]
macro_rules! export_process_item_ex {
    ($variable:ident) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub unsafe extern "C" fn XT_ProcessItemEx(nItemID: LONG, hItem: HANDLE,  lpReserved: PVOID) -> LONG {
            debug!("XT_ProcessItemEx called");
            let res_item = ItemHandle::new(hItem, Item::new(nItemID));
            if res_item.is_err() {
                error!("error in parsing hItem argument");
            }
            $variable.xt_process_item_ex(res_item.unwrap()).into()
        }
    };
}

#[macro_export]
macro_rules! export_all_functions_ex {
    ($variable:ident) => {
        export_xt_init!($variable);
        export_xt_prepare!($variable);
        export_xt_finalize!($variable);
        export_xt_done!($variable);
        export_xt_about!($variable);
        export_xt_process_item_ex!($variable)
    };
}

#[macro_export]
macro_rules! export_all_functions {
    ($variable:ident) => {
        export_xt_init!($variable);
        export_xt_prepare!($variable);
        export_xt_finalize!($variable);
        export_xt_done!($variable);
        export_xt_about!($variable);
        export_xt_process_item!($variable);
    };
}

#[macro_export]
macro_rules! needed_use_declarations {
    () => {
        use bitflags::Flags;
        use winapi::shared::minwindef::*;
        use winapi::shared::ntdef::*;
        use simplelog::{WriteLogger, LevelFilter, Config};

        use log::{debug, error, info};
        use $crate::xwf::*;
        use $crate::xwf::raw_api::*;
        use $crate::xwf::api::evidence::*;
        use $crate::xwf::api::item::*;
        use $crate::xwf::api::volume::*;
        use $crate::xwf::api::application::*;
        use $crate::xwf::api::traits::*;
        use $crate::xwf::api::window::*;
        use $crate::xwf::xwf_types::*;
    };
}

