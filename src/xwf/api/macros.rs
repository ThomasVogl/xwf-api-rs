use std::sync::Mutex;
use crate::xwf::raw_api::{RAW_API, RawApi};
#[macro_export]
macro_rules! export_xt_init {

    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "C"  fn XT_Init(nVersion: DWORD, nFlags: DWORD, hMainWnd: HANDLE, lpReserved: PVOID) -> LONG {
            unsafe {
                $variable.get_or_init(|| <$variable_type>::create());
            }

            let logger = WriteLogger::init(LevelFilter::Debug, Config::default(), Application::new());

            let xtension_version = get_lib_instance!($variable, $variable_type).xtension_version();
            info!("X-Tension {}, Version {}.{}.{}", get_lib_instance!($variable, $variable_type).xtension_name(), xtension_version.0, xtension_version.1, xtension_version.2 );
            debug!("XT_Init called");
            get_lib_instance!($variable, $variable_type).xt_init(XtVersion::try_from(nVersion).unwrap(),
            XtInitFlags::from_bits_truncate(nFlags),
            Window::new(hMainWnd), XtLicenseInfo {}) as i32
        }
    }

}
#[macro_export]
macro_rules! get_lib_instance {
    ($variable:ident, $variable_type:ty) => {
        unsafe {
            $variable.get_mut().unwrap()
        }

    }
}
#[macro_export]
macro_rules! export_xt_done {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "C" fn XT_Done(lpReserved: PVOID)
            -> LONG {
            debug!("XT_Done called");
            get_lib_instance!($variable, $variable_type).xt_done();
            0
        }
    };
}
#[macro_export]
macro_rules! export_xt_about {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "C" fn XT_About(hParentWnd: HANDLE, lpReserved: PVOID)
            -> LONG {
            debug!("XT_About called");
            get_lib_instance!($variable, $variable_type).xt_about(Window::new(hParentWnd));
            0
        }
    };
}
#[macro_export]
macro_rules! export_xt_prepare {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "C" fn XT_Prepare(hVolume: HANDLE, hEvidence: HANDLE,  nOpType: DWORD, lpReserved: PVOID
        ) -> LONG {
            debug!("XT_Prepare called");

            let opt_op_type = XtPrepareOpType::try_from(nOpType);
            if opt_op_type.is_err() {
                error!("error in parsing nOpType argument");
                return XtPrepareReturn::Negative(XtPrepareNegativeReturn::JustCallXtFinalize).into();
            }

            get_lib_instance!($variable, $variable_type).xt_prepare(
                Volume::new(hVolume),
                Evidence::new(hEvidence),
                opt_op_type.unwrap()).into()

        }
    };
}
#[macro_export]
macro_rules! export_xt_finalize {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "C" fn XT_Finalize(hVolume: HANDLE, hEvidence: HANDLE,  nOpType: DWORD, lpReserved: PVOID
        ) -> LONG {
            debug!("XT_Finalize called");
            let opt_op_type = XtPrepareOpType::try_from(nOpType);
            if opt_op_type.is_err() {
                error!("error in parsing nOpType argument");
                return XtFinalizeReturn::Ok.into();
            }

            get_lib_instance!($variable, $variable_type).xt_finalize(
                Volume::new(hVolume),
                Evidence::new(hEvidence),
                opt_op_type.unwrap()).into()
        }
    };
}
#[macro_export]
macro_rules! export_xt_process_item {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "C" fn XT_ProcessItem(nItemID: LONG,  lpReserved: PVOID) -> LONG {
            let item = Item::new(nItemID);

            get_lib_instance!($variable, $variable_type).xt_process_item(item).into()
        }
    };
}
#[macro_export]
macro_rules! export_xt_process_item_ex {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "C" fn XT_ProcessItemEx(nItemID: LONG, hItem: HANDLE,  lpReserved: PVOID) -> LONG {
            let res_item = ItemHandle::new(hItem, Item::new(nItemID));
            if res_item.is_err() {
                error!("error in parsing hItem argument");
            }
            get_lib_instance!($variable, $variable_type).xt_process_item_ex(res_item.unwrap()).into()
        }
    };
}



#[macro_export]
macro_rules! create_static_var {
    ($variable_name:ident, $variable_type:ty) => {
        static mut $variable_name: OnceLock<$variable_type> = OnceLock::new();
    }
}

#[macro_export]
macro_rules! export_all_functions {
    ($variable_name:ident, $variable_type:ty) => {

        create_static_var!($variable_name, $variable_type);

        export_xt_init!($variable_name, $variable_type);
        export_xt_prepare!($variable_name, $variable_type);
        export_xt_finalize!($variable_name, $variable_type);
        export_xt_done!($variable_name, $variable_type);
        export_xt_about!($variable_name, $variable_type);
        export_xt_process_item!($variable_name, $variable_type);
    };
}

#[macro_export]
macro_rules! export_all_functions_ex {
    ($variable_name:ident, $variable_type:ty) => {
        create_static_var!($variable_name, $variable_type);

        export_xt_init!($variable_name, $variable_type);
        export_xt_prepare!($variable_name, $variable_type);
        export_xt_finalize!($variable_name, $variable_type);
        export_xt_done!($variable_name, $variable_type);
        export_xt_about!($variable_name, $variable_type);
        export_xt_process_item_ex!($variable_name, $variable_type);
    };
}

#[macro_export]
macro_rules! needed_use_declarations {
    () => {
        use bitflags::Flags;
        use winapi::shared::minwindef::*;
        use winapi::shared::ntdef::*;
        use simplelog::{WriteLogger, LevelFilter, Config};
        use std::sync::Mutex;
        use std::sync::OnceLock;

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

