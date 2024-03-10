#[macro_export]
macro_rules! export_xt_init {

    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "C"  fn XT_Init(nVersion: DWORD, nFlags: DWORD, hMainWnd: HANDLE, lpReserved: PVOID) -> LONG {

                    
            unsafe {
                RAW_API = xwf::raw_api::RawApi::load().ok();
            }
            
            unsafe {
                $variable = Some(<$variable_type>::create());
            }

            let xtension_version = get_lib_instance!($variable, $variable_type).xtension_version();
            xwfinfo!("X-Tension {}, Version {}.{}.{}", get_lib_instance!($variable, $variable_type).xtension_name(), xtension_version.0, xtension_version.1, xtension_version.2 );
            
            match option_env!("BUILD_DATE") {
                Some(v) => xwfinfo!("Datum des Builds: {}", v),
                _ => {}
            };
            
            xwfdebug!("XT_Init called");
            let res = get_lib_instance!($variable, $variable_type).xt_init(XtVersion::try_from(nVersion).unwrap(),
            XtInitFlags::from_bits_truncate(nFlags),
            Window::new(hMainWnd), XtLicenseInfo {});

            
            match res {
                Ok(ret) => ret as i32,
                Err(e) => {
                    xwfwarn!("{}", e);
                    XtInitReturn::PreventFurtherUseOfDll as i32
                }
            }
        }
    }

}
#[macro_export]
macro_rules! get_lib_instance {
    ($variable:ident, $variable_type:ty) => {
        unsafe {
            $variable.as_mut().unwrap()
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
            xwfdebug!("XT_Done called");
            get_lib_instance!($variable, $variable_type).xt_done();

            //uninitalize raw api
            unsafe {
                let _ = RAW_API.take();
                let _ = $variable.take();
            }
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
            xwfdebug!("XT_About called");
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
            xwfdebug!("XT_Prepare called");

            let opt_op_type = XtPrepareOpType::try_from(nOpType);
            if opt_op_type.is_err() {
                xwferror!("error in parsing nOpType argument");
                return XtPrepareReturn::Negative(XtPrepareNegativeReturn::JustCallXtFinalize).into();
            }

            let res = get_lib_instance!($variable, $variable_type).xt_prepare(
                Volume::new(hVolume).ok(),
                Evidence::new(hEvidence),
                opt_op_type.unwrap());

            match res {
                Ok(ret) => ret.into(),
                Err(e) => {
                    xwfwarn!("{}", e);
                    XtPrepareNegativeReturn::JustCallXtFinalize.into()
                }
            }

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
            xwfdebug!("XT_Finalize called");
            let opt_op_type = XtPrepareOpType::try_from(nOpType);
            if opt_op_type.is_err() {
                xwferror!("error in parsing nOpType argument");
                return XtFinalizeReturn::Ok.into();
            }

            let res = get_lib_instance!($variable, $variable_type).xt_finalize(
                Volume::new(hVolume).ok(),
                Evidence::new(hEvidence),
                opt_op_type.unwrap());

            match res {
                Ok(ret) => ret.into(),
                Err(e) => {
                    xwfwarn!("{}", e);
                    XtPrepareNegativeReturn::JustCallXtFinalize.into()
                }
            }
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

            let res = get_lib_instance!($variable, $variable_type).xt_process_item(item);

            match res {
                Ok(ret) => {
                     ret.into() 
                },
                Err(e) => {
                    xwfwarn!("{}", e);
                    XtProcessItemReturn::Ok.into()
                }
            }
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
                xwferror!("failed to parse hItem Argument");
                return XtProcessItemExReturn::StopCurrentOperation.into();
            }

            let res = get_lib_instance!($variable, $variable_type).xt_process_item_ex(res_item.unwrap());

            match res {
                Ok(ret) => ret.into(),
                Err(e) => {
                    xwfwarn!("{}", e);
                    XtProcessItemExReturn::Ok.into()
                }
            }
        }
    };
}



#[macro_export]
macro_rules! create_static_var {
    ($variable_name:ident, $variable_type:ty) => {
        static mut $variable_name: Option<$variable_type> = None;
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
        use winapi::shared::minwindef::*;
        use winapi::shared::ntdef::*;

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

