use log::info;
use winapi::shared::ntdef::HANDLE;
use crate::xwf::api::evidence::Evidence;
use crate::xwf::api::item::{Item, ItemHandle};
use crate::xwf::api::volume::Volume;
use crate::xwf::api::window::Window;
use crate::xwf::xwf_types::{XtFinalizeReturn, XtInitFlags, XtInitReturn, XtLicenseInfo, XtPrepareOpType, XtPreparePositiveReturnFlags, XtPrepareReturn, XtProcessItemExReturn, XtProcessItemReturn, XtVersion};

pub trait NativeHandle {
    fn get_handle(&self) -> HANDLE;
}



pub trait XTension {

    type XTensionError;

    fn create() -> Self;

    fn xtension_version(&self) -> (u8, u8, u8);
    fn xtension_name(&self) -> String;

    //TODO implement LicenseInfo argument, currently it is empty
    fn xt_init(&mut self, version: XtVersion, flags: XtInitFlags, main_window: Option<Window>, license_info: XtLicenseInfo) -> Result<XtInitReturn, Self::XTensionError> {
        Ok(XtInitReturn::RunSingleThreaded)
    }
    fn xt_done(&mut self) {
    }
    fn xt_about(&mut self, parent_window: Option<Window>) {

    }
    fn xt_prepare(&mut self, volume: Option<Volume>, evidence: Option<Evidence>, op_type: XtPrepareOpType) -> Result<XtPrepareReturn, Self::XTensionError> {
        Ok(XtPrepareReturn::Positive(XtPreparePositiveReturnFlags::CallProcessItemLate))
    }
    fn xt_process_item(&mut self, item: Item) -> Result<XtProcessItemReturn, Self::XTensionError> {
        Ok(XtProcessItemReturn::Ok)
    }
    fn xt_process_item_ex(&mut self, item_handle: ItemHandle) -> Result<XtProcessItemExReturn, Self::XTensionError> {
        Ok(XtProcessItemExReturn::Ok)
    }

    fn xt_finalize(&mut self, volume: Option<Volume>, evidence: Option<Evidence>, op_type: XtPrepareOpType) -> Result<XtFinalizeReturn, Self::XTensionError> {
        Ok(XtFinalizeReturn::Ok)
    }

    //TODO: implement XT_PrepareSearch and XT_ProcessSearchHit

}


pub trait XTensionBase: XTension{



}