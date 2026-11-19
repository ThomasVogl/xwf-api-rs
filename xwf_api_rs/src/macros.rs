/// Executes the body of an exported XT_* function. A panic must never unwind into X-Ways
/// (this aborts the whole X-Ways process), so it is caught, logged and `$fallback` is returned.
#[doc(hidden)]
#[macro_export]
macro_rules! xt_guarded_call {
    ($fn_name:expr, $fallback:expr, $body:block) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(ret) => ret,
            Err(e) => {
                $crate::xwferror!("{}: unexpected error (panic): {}", $fn_name, $crate::instance::panic_message(&e));
                $fallback
            }
        }
    }};
}

#[macro_export]
macro_rules! export_xt_init {

    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "system" fn XT_Init(nVersion: $crate::winapi::shared::minwindef::DWORD,
                                       nFlags: $crate::winapi::shared::minwindef::DWORD,
                                       hMainWnd: $crate::winapi::shared::ntdef::HANDLE,
                                       lpReserved: $crate::winapi::shared::ntdef::PVOID) -> $crate::winapi::shared::ntdef::LONG {

            // without the X-Ways API functions, not even an error message can be shown
            if !$crate::raw_api::RawApi::is_available() {
                return $crate::xwf_types::XtInitReturn::PreventFurtherUseOfDll as i32;
            }

            $crate::xt_guarded_call!("XT_Init", $crate::xwf_types::XtInitReturn::PreventFurtherUseOfDll as i32, {
                let xt_version = match $crate::xwf_types::XtVersion::try_from(nVersion) {
                    Ok(v) => v,
                    Err(e) => {
                        $crate::xwferror!("XT_Init: invalid version number: {}", e);
                        return $crate::xwf_types::XtInitReturn::PreventFurtherUseOfDll as i32;
                    }
                };

                let lic_info = $crate::xwf_types::XtLicenseInfo {};
                let flags = $crate::xwf_types::XtInitFlags::from_bits_truncate(nFlags);
                let window = $crate::window::Window::new(hMainWnd);

                let context = match $crate::context::ExecutionContext::create(xt_version, flags, lic_info, window) {
                    Ok(c) => c,
                    Err(e) => {
                        $crate::xwferror!("Could not create ExecutionContext: {}", e);
                        return $crate::xwf_types::XtInitReturn::PreventFurtherUseOfDll as i32;
                    }
                };

                if let Err(e) = $variable.set(<$variable_type>::create(context)) {
                    $crate::xwferror!("XT_Init: {}", e);
                    return $crate::xwf_types::XtInitReturn::PreventFurtherUseOfDll as i32;
                }

                $crate::xwfdebug!("XT_Init called");

                $crate::xwfinfo!("X-Tension \"{}\" Version {} started", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION") );
                $crate::xwfinfo!("powered by xwf-api-rs library version 2 (https://github.com/ThomasVogl/xwf-api-rs)");

                match $variable.with(|xt| xt.xt_init()) {
                    Ok(Ok(ret)) => ret as i32,
                    Ok(Err(e)) => {
                        $crate::xwferror!("XT_Init: {}", e);
                        $crate::xwf_types::XtInitReturn::PreventFurtherUseOfDll as i32
                    },
                    Err(e) => {
                        $crate::xwferror!("XT_Init: {}", e);
                        $crate::xwf_types::XtInitReturn::PreventFurtherUseOfDll as i32
                    }
                }
            })
        }
    }

}
#[macro_export]
macro_rules! export_xt_done {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "system" fn XT_Done(lpReserved: $crate::winapi::shared::ntdef::PVOID)
            -> $crate::winapi::shared::ntdef::LONG {
            $crate::xt_guarded_call!("XT_Done", 0, {
                $crate::xwfdebug!("XT_Done called");

                match $variable.with(|xt| xt.xt_done()) {
                    Ok(Ok(_)) => {},
                    Ok(Err(e)) => { $crate::xwferror!("XT_Done: {}", e); },
                    Err(e) => {
                        // do not drop the instance while it is still in use (reentrant call)
                        $crate::xwferror!("XT_Done: {}", e);
                        return 0;
                    }
                }

                $crate::xwfinfo!("X-Tension finished.");

                // drop the instance of the X-Tension
                let _ = $variable.take();
                0
            })
        }
    };
}
#[macro_export]
macro_rules! export_xt_about {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "system" fn XT_About(hParentWnd: $crate::winapi::shared::ntdef::HANDLE,
                                        lpReserved: $crate::winapi::shared::ntdef::PVOID)
            -> $crate::winapi::shared::ntdef::LONG {
            $crate::xt_guarded_call!("XT_About", 0, {
                $crate::xwfdebug!("XT_About called");
                match $variable.with(|xt| xt.xt_about($crate::window::Window::new(hParentWnd))) {
                    Ok(Ok(_)) => 0,
                    Ok(Err(e)) => {
                        $crate::xwferror!("XT_About: {}", e);
                        0
                    },
                    Err(e) => {
                        $crate::xwferror!("XT_About: {}", e);
                        0
                    }
                }
            })
        }
    };
}
#[macro_export]
macro_rules! export_xt_prepare {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "system" fn XT_Prepare(hVolume: $crate::winapi::shared::ntdef::HANDLE,
                                          hEvidence: $crate::winapi::shared::ntdef::HANDLE,
                                          nOpType: $crate::winapi::shared::minwindef::DWORD,
                                          lpReserved: $crate::winapi::shared::ntdef::PVOID
        ) -> $crate::winapi::shared::ntdef::LONG {
            let fallback: i32 = $crate::xwf_types::XtPrepareNegativeReturn::PreventFurtherUse.into();

            $crate::xt_guarded_call!("XT_Prepare", fallback, {
                $crate::xwfdebug!("XT_Prepare called");

                let res_op_type = $crate::xwf_types::XtOpType::try_from(nOpType);
                if res_op_type.is_err() {
                    $crate::xwferror!("error in parsing nOpType argument");
                    return $crate::xwf_types::XtPrepareReturn::Negative($crate::xwf_types::XtPrepareNegativeReturn::JustCallXtFinalize).into();
                }

                let res = $variable.with(|xt| {
                    // volume and evidence object are owned by X-Ways
                    xt.get_context_mut().set(
                        $crate::evidence::Evidence::new(hEvidence),
                        $crate::volume::Volume::new(hVolume).ok(),
                        res_op_type.ok()
                    );
                    xt.xt_prepare()
                });

                match res {
                    Ok(Ok(ret)) => ret.into(),
                    Ok(Err(e)) => {
                        $crate::xwferror!("XT_Prepare: {}", e);
                        $crate::xwf_types::XtPrepareNegativeReturn::JustCallXtFinalize.into()
                    },
                    Err(e) => {
                        $crate::xwferror!("XT_Prepare: {}", e);
                        fallback
                    }
                }
            })
        }
    };
}
#[macro_export]
macro_rules! export_xt_finalize {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "system" fn XT_Finalize(hVolume: $crate::winapi::shared::ntdef::HANDLE,
                                           hEvidence: $crate::winapi::shared::ntdef::HANDLE,
                                           nOpType: $crate::winapi::shared::minwindef::DWORD,
                                           lpReserved: $crate::winapi::shared::ntdef::PVOID
        ) -> $crate::winapi::shared::ntdef::LONG {
            $crate::xt_guarded_call!("XT_Finalize", $crate::xwf_types::XtFinalizeReturn::Ok.into(), {
                $crate::xwfdebug!("XT_Finalize called");
                let res_op_type = $crate::xwf_types::XtOpType::try_from(nOpType);
                if res_op_type.is_err() {
                    $crate::xwferror!("error in parsing nOpType argument");
                    return $crate::xwf_types::XtFinalizeReturn::Ok.into();
                }

                let res = $variable.with(|xt| {
                    // volume and evidence object are owned by X-Ways
                    xt.get_context_mut().set(
                        $crate::evidence::Evidence::new(hEvidence),
                        $crate::volume::Volume::new(hVolume).ok(),
                        res_op_type.ok()
                    );
                    let ret = xt.xt_finalize();
                    // handles are only valid during this call
                    xt.get_context_mut().reset_volume_evidence();
                    ret
                });

                match res {
                    Ok(Ok(ret)) => ret.into(),
                    Ok(Err(e)) => {
                        $crate::xwferror!("XT_Finalize: {}", e);
                        $crate::xwf_types::XtFinalizeReturn::Ok.into()
                    },
                    Err(e) => {
                        $crate::xwferror!("XT_Finalize: {}", e);
                        $crate::xwf_types::XtFinalizeReturn::Ok.into()
                    }
                }
            })
        }
    };
}
#[macro_export]
macro_rules! export_xt_process_item {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "system" fn XT_ProcessItem(nItemID: $crate::winapi::shared::ntdef::LONG,
                                              lpReserved: $crate::winapi::shared::ntdef::PVOID) -> $crate::winapi::shared::ntdef::LONG {
            $crate::xt_guarded_call!("XT_ProcessItem", $crate::xwf_types::XtProcessItemReturn::StopCurrentOperation.into(), {
                let item = $crate::item::Item::new(nItemID);

                // in multi-threaded mode, calls are serialized by the instance lock
                match $variable.with(|xt| xt.xt_process_item(item)) {
                    Ok(Ok(ret)) => ret.into(),
                    Ok(Err(e)) => {
                        $crate::xwferror!("XT_ProcessItem: error occurred in processing item id {}", nItemID);
                        $crate::xwferror!("XT_ProcessItem: {}", e);
                        $crate::xwferror!("XT_ProcessItem: stopping operation due to previous error");
                        $crate::xwf_types::XtProcessItemReturn::StopCurrentOperation.into()
                    },
                    Err(e) => {
                        $crate::xwferror!("XT_ProcessItem: {}", e);
                        $crate::xwf_types::XtProcessItemReturn::StopCurrentOperation.into()
                    }
                }
            })
        }
    };
}
#[macro_export]
macro_rules! export_xt_process_item_ex {
    ($variable:ident, $variable_type:ty) => {
        #[no_mangle]
        #[allow(non_snake_case, unused_variables)]
        pub extern "system" fn XT_ProcessItemEx(nItemID: $crate::winapi::shared::ntdef::LONG,
                                                hItem: $crate::winapi::shared::ntdef::HANDLE,
                                                lpReserved: $crate::winapi::shared::ntdef::PVOID) -> $crate::winapi::shared::ntdef::LONG {
            $crate::xt_guarded_call!("XT_ProcessItemEx", $crate::xwf_types::XtProcessItemExReturn::StopCurrentOperation.into(), {
                // hItem is owned by X-Ways and must not be closed by the X-Tension
                let res_item = $crate::item::ItemHandle::new(hItem, $crate::item::Item::new(nItemID), false);
                let item_handle = match res_item {
                    Ok(h) => h,
                    Err(_) => {
                        $crate::xwferror!("failed to parse hItem Argument");
                        $crate::xwferror!("XT_ProcessItemEx: stopping operation due to previous error");
                        return $crate::xwf_types::XtProcessItemExReturn::StopCurrentOperation.into();
                    }
                };

                // in multi-threaded mode, calls are serialized by the instance lock
                match $variable.with(|xt| xt.xt_process_item_ex(item_handle)) {
                    Ok(Ok(ret)) => ret.into(),
                    Ok(Err(e)) => {
                        $crate::xwferror!("XT_ProcessItemEx: error occurred in processing item id {}", nItemID);
                        $crate::xwferror!("XT_ProcessItemEx: {}", e);
                        $crate::xwferror!("XT_ProcessItemEx: stopping operation due to previous error");
                        $crate::xwf_types::XtProcessItemExReturn::StopCurrentOperation.into()
                    },
                    Err(e) => {
                        $crate::xwferror!("XT_ProcessItemEx: {}", e);
                        $crate::xwf_types::XtProcessItemExReturn::StopCurrentOperation.into()
                    }
                }
            })
        }
    };
}



#[macro_export]
macro_rules! create_static_var {
    ($variable_name:ident, $variable_type:ty) => {
        static $variable_name: $crate::instance::XtInstance<$variable_type> = $crate::instance::XtInstance::new();
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

        $crate::create_static_var!($variable_name, $variable_type);

        $crate::export_xt_init!($variable_name, $variable_type);
        $crate::export_xt_prepare!($variable_name, $variable_type);
        $crate::export_xt_finalize!($variable_name, $variable_type);
        $crate::export_xt_done!($variable_name, $variable_type);
        $crate::export_xt_about!($variable_name, $variable_type);
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
