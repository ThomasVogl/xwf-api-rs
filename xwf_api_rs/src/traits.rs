use crate::xwf_types::*;
use winapi::shared::ntdef::HANDLE;
use crate::evidence::Evidence;
use crate::item::{Item, ItemHandle};
use crate::volume::Volume;
use crate::window::Window;


pub trait NativeHandle {
    fn get_handle(&self) -> HANDLE;
}

pub trait XTension {

    type XTensionError;

    fn create() -> Self;

    fn xt_init(&mut self, _version: XtVersion, _flags: XtInitFlags, _window: Option<Window>, _lic_info: XtLicenseInfo) -> Result<XtInitReturn, Self::XTensionError> {
        Ok(XtInitReturn::RunSingleThreaded)
    }

    fn xt_done(&mut self) {
    }
    fn xt_about(&mut self, _: Option<Window>) {

    }
    fn xt_prepare(&mut self, _volume: Option<Volume>, _evidence: Option<Evidence>, _op_type: XtPrepareOpType) -> Result<XtPrepareReturn, Self::XTensionError> {
        Ok(XtPrepareReturn::Positive(XtPreparePositiveReturnFlags::CallProcessItemLate))
    }
    fn xt_process_item(&mut self, _item: Item) -> Result<XtProcessItemReturn, Self::XTensionError> {
        Ok(XtProcessItemReturn::Ok)
    }
    fn xt_process_item_ex(&mut self, _handle: ItemHandle) -> Result<XtProcessItemExReturn, Self::XTensionError> {
        Ok(XtProcessItemExReturn::Ok)
    }

    fn xt_finalize(&mut self, _volume: Option<Volume>, _evidence: Option<Evidence>, _op_type: XtPrepareOpType) -> Result<XtFinalizeReturn, Self::XTensionError> {
        Ok(XtFinalizeReturn::Ok)
    }
}