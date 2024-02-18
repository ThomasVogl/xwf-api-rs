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
    fn xt_init(&mut self, _: XtVersion, _: XtInitFlags, _: Option<Window>, _: XtLicenseInfo) -> Result<XtInitReturn, Self::XTensionError> {
        Ok(XtInitReturn::RunSingleThreaded)
    }
    fn xt_done(&mut self) {
    }
    fn xt_about(&mut self, _: Option<Window>) {

    }
    fn xt_prepare(&mut self, _: Option<Volume>, _: Option<Evidence>, _: XtPrepareOpType) -> Result<XtPrepareReturn, Self::XTensionError> {
        Ok(XtPrepareReturn::Positive(XtPreparePositiveReturnFlags::CallProcessItemLate))
    }
    fn xt_process_item(&mut self, _: Item) -> Result<XtProcessItemReturn, Self::XTensionError> {
        Ok(XtProcessItemReturn::Ok)
    }
    fn xt_process_item_ex(&mut self, _: ItemHandle) -> Result<XtProcessItemExReturn, Self::XTensionError> {
        Ok(XtProcessItemExReturn::Ok)
    }

    fn xt_finalize(&mut self, _: Option<Volume>, _: Option<Evidence>, _: XtPrepareOpType) -> Result<XtFinalizeReturn, Self::XTensionError> {
        Ok(XtFinalizeReturn::Ok)
    }

    //TODO: implement XT_PrepareSearch and XT_ProcessSearchHit

}


pub trait XTensionBase: XTension{



}