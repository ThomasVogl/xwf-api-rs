#[macro_export]
macro_rules! export_xt_init {

    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "C"  fn XT_Init(nVersion: DWORD, nFlags: DWORD, hMainWnd: HANDLE, lpReserved: PVOID) -> LONG {

            let lic_info = XtLicenseInfo {};
            let xt_version = XtVersion::try_from(nVersion).unwrap();
            let flags = XtInitFlags::from_bits_truncate(nFlags);
            let window = $crate::window::Window::new(hMainWnd);


            let context = ExecutionContext::create(xt_version, flags, lic_info, window);
            if context.is_err() {
                $crate::xwferror!("Could not create ExecutionContext: {}", context.err().unwrap());
                return XtInitReturn::PreventFurtherUseOfDll as i32;
            }


            unsafe {
                $variable = Some(<$variable_type>::create(context.unwrap()));
            }

            $crate::xwfdebug!("XT_Init called");

            $crate::xwfinfo!("X-Tension \"{}\" Version {} started", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION") );
            $crate::xwfinfo!("powered by xwf-api-rs library version 2 (https://github.com/ThomasVogl/xwf-api-rs)");


            let res = $crate::get_lib_instance!($variable, $variable_type).xt_init();


            match res {
                Ok(ret) => ret as i32,
                Err(e) => {
                    $crate::xwferror!("XT_Init: {}", e);
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
            $crate::xwfdebug!("XT_Done called");
            let res = $crate::get_lib_instance!($variable, $variable_type).xt_done();

            if res.is_err() {
                $crate::xwferror!("XT_Done: {}", res.err().unwrap());
            }

            $crate::xwfinfo!("X-Tension finished.");

            //uninitalize raw api
            unsafe {
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
            $crate::xwfdebug!("XT_About called");
            let res = $crate::get_lib_instance!($variable, $variable_type).xt_about($crate::window::Window::new(hParentWnd));
            match res {
                Ok(_) => {
                     0
                },
                Err(e) => {
                    $crate::xwferror!("XT_About: {}", e);
                    0
                }
            }

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
            $crate::xwfdebug!("XT_Prepare called");

            let res_op_type = XtOpType::try_from(nOpType);
            if res_op_type.is_err() {
                $crate::xwferror!("error in parsing nOpType argument");
                return XtPrepareReturn::Negative(XtPrepareNegativeReturn::JustCallXtFinalize).into();
            }

            let ctx = $crate::get_lib_instance!($variable, $variable_type).get_context_mut();

            ctx.set(
                $crate::evidence::Evidence::new(hEvidence),
                $crate::volume::Volume::new(hVolume).ok(),
                res_op_type.ok()
            );

            let res = $crate::get_lib_instance!($variable, $variable_type).xt_prepare();

            match res {
                Ok(ret) => ret.into(),
                Err(e) => {
                    $crate::xwferror!("XT_Prepare: {}", e);
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
            $crate::xwfdebug!("XT_Finalize called");
            let res_op_type = XtOpType::try_from(nOpType);
            if res_op_type.is_err() {
                $crate::xwferror!("error in parsing nOpType argument");
                return XtFinalizeReturn::Ok.into();
            }

            let ctx = $crate::get_lib_instance!($variable, $variable_type).get_context_mut();
            ctx.set(
                $crate::evidence::Evidence::new(hEvidence),
                $crate::volume::Volume::new(hVolume).ok(),
                res_op_type.ok()
            );

            let res = $crate::get_lib_instance!($variable, $variable_type).xt_finalize();
            match res {
                Ok(ret) => {
                    let ctx = $crate::get_lib_instance!($variable, $variable_type).get_context_mut();
                    ctx.reset_volume_evidence();
                    ret.into()
                },
                Err(e) => {
                    $crate::xwferror!("XT_Finalize: {}", e);
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
            let item = $crate::item::Item::new(nItemID);

            let res = $crate::get_lib_instance!($variable, $variable_type).xt_process_item(item);

            match res {
                Ok(ret) => {
                     ret.into() 
                },
                Err(e) => {
                    $crate::xwferror!("XT_ProcessItem: error occurred in processing item id {}", nItemID);
                    $crate::xwferror!("XT_ProcessItem: {}", e);
                    $crate::xwferror!("XT_ProcessItem: stopping operation due to previous error");
                    XtProcessItemReturn::StopCurrentOperation.into()
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
            let res_item = $crate::item::ItemHandle::new(hItem, $crate::item::Item::new(nItemID), false);
            if res_item.is_err() {
                $crate::xwferror!("failed to parse hItem Argument");
                $crate::xwferror!("XT_ProcessItemEx: stopping operation due to previous error");
                return XtProcessItemExReturn::StopCurrentOperation.into();
            }

            let res = $crate::get_lib_instance!($variable, $variable_type).xt_process_item_ex(res_item.unwrap());

            match res {
                Ok(ret) => ret.into(),
                Err(e) => {
                    $crate::xwferror!("XT_ProcessItemEx: error occurred in processing item id {}", nItemID);
                    $crate::xwferror!("XT_ProcessItemEx: {}", e);
                    $crate::xwferror!("XT_ProcessItemEx: stopping operation due to previous error");
                    XtProcessItemExReturn::StopCurrentOperation.into()

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
        use $crate::winapi::shared::minwindef::{DWORD, LPVOID};
        use $crate::winapi::shared::ntdef::{PVOID, LONG, HANDLE};
        use $crate::raw_api::RAW_API;

        $crate::create_static_var!($variable_name, $variable_type);

        $crate::export_xt_init!($variable_name, $variable_type);
        $crate::export_xt_prepare!($variable_name, $variable_type);
        $crate::export_xt_finalize!($variable_name, $variable_type);
        $crate::export_xt_done!($variable_name, $variable_type);
        $crate::export_xt_about!($variable_name, $variable_type);
        $crate::export_xt_process_item!($variable_name, $variable_type);
    };
}


#[macro_export]
macro_rules! export_without_process_item {
    ($variable_name:ident, $variable_type:ty) => {

        create_static_var!($variable_name, $variable_type);

        export_xt_init!($variable_name, $variable_type);
        export_xt_prepare!($variable_name, $variable_type);
        export_xt_finalize!($variable_name, $variable_type);
        export_xt_done!($variable_name, $variable_type);
        export_xt_about!($variable_name, $variable_type);
    };
}

#[macro_export]
macro_rules! export_all_functions_ex {
    ($variable_name:ident, $variable_type:ty) => {
        use $crate::winapi::shared::minwindef::{DWORD, LPVOID};
        use $crate::winapi::shared::ntdef::{PVOID, LONG, HANDLE};

        $crate::create_static_var!($variable_name, $variable_type);

        $crate::export_xt_init!($variable_name, $variable_type);
        $crate::export_xt_prepare!($variable_name, $variable_type);
        $crate::export_xt_finalize!($variable_name, $variable_type);
        $crate::export_xt_done!($variable_name, $variable_type);
        $crate::export_xt_about!($variable_name, $variable_type);
        $crate::export_xt_process_item_ex!($variable_name, $variable_type);
    };
}
