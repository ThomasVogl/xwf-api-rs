
use crate::xwf_types::*;
use winapi::shared::ntdef::HANDLE;
use crate::context::ExecutionContext;
use crate::item::{Item, ItemHandle};
use crate::window::Window;


pub trait NativeHandle {
    fn get_handle(&self) -> HANDLE;
}

pub trait XTension {

    type XTensionError;

    fn create(context: ExecutionContext) -> Self;

    fn get_context_mut(&mut self) -> &mut ExecutionContext;

    fn get_context(&self) -> &ExecutionContext;


    fn xt_init(&mut self) -> Result<XtInitReturn, Self::XTensionError> {
        Ok(XtInitReturn::RunSingleThreaded)
    }

    fn xt_done(&mut self) -> Result<(), Self::XTensionError> {
        Ok(())
    }
    fn xt_about(&mut self, _: Option<Window>) -> Result<(), Self::XTensionError> {
        Ok(())
    }
    fn xt_prepare(&mut self) -> Result<XtPrepareReturn, Self::XTensionError> {
        Ok(XtPrepareReturn::Positive(XtPreparePositiveReturnFlags::CallProcessItemLate))
    }
    fn xt_process_item(&mut self, _item: Item) -> Result<XtProcessItemReturn, Self::XTensionError> {
        Ok(XtProcessItemReturn::Ok)
    }
    fn xt_process_item_ex(&mut self, _handle: ItemHandle) -> Result<XtProcessItemExReturn, Self::XTensionError> {
        Ok(XtProcessItemExReturn::Ok)
    }

    fn xt_finalize(&mut self) -> Result<XtFinalizeReturn, Self::XTensionError> {
        Ok(XtFinalizeReturn::Ok)
    }
}