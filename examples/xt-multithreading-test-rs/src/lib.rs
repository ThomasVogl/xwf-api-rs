use std::fs;
use std::path::{Path, PathBuf};
use xwf_api_rs::{xwferror, xwf_types::*, traits::XTension, error::XwfError, export_all_functions_ex};
use xwf_api_rs::case::Case;
use xwf_api_rs::context::ExecutionContext;
use xwf_api_rs::item::{ItemHandle};
use xwf_api_rs::volume::HashType;

fn workload() {
    let mut x = 0.0001f64;
    for i in 0..10_000_000 {
        x = (i as f64) * x.sin().cos().tan().exp().ln();
    }
}

// define a custom structure representing your extension
// could also have attributes of course
pub struct MultiThreadXtension {
    context: ExecutionContext,
    current_output_path: Option<PathBuf>
}

// implement XTension Trait
// most of the function have a default and empty implementation already
impl XTension for MultiThreadXtension {

    // define your error type here. You can also define you own error type or use predefined "XwfError"
    type XTensionError = XwfError;

    // function to create an instance of your XTension struct
    fn create(context: ExecutionContext) -> MultiThreadXtension {
        MultiThreadXtension {
            context,
            current_output_path: None
        }
    }

    fn get_context_mut(&mut self) -> &mut ExecutionContext {
        &mut self.context
    }

    fn get_context(&self) -> &ExecutionContext {
        &self.context
    }

    //function to initialize the X-Tension. Wraps XT_Init() Function from C API
    fn xt_init(&mut self) -> Result<XtInitReturn, Self::XTensionError> {
        Ok(XtInitReturn::RunMultiThreaded)
    }

    //prepare function wraps XT_Prepare() Function from C API
    //please refer to X-Ways X-Tension API doc for details regarding calling logic
    fn xt_prepare(&mut self) -> Result<XtPrepareReturn, Self::XTensionError> {

        // compute current output path based on case directory by using X-Tension name and current evidence name
        let case_infos = Case::get_case_infos()?;
        let mut output_path = Path::new(&case_infos.dir).join(env!("CARGO_PKG_NAME"));
        if let Some(ev) = self.context.get_evidence() {
            output_path = output_path.join(ev.get_name()?);
        }
        self.current_output_path = Some(output_path);

        // create output path if not existent
        if !self.current_output_path.as_ref().unwrap().exists() {
            fs::create_dir_all(self.current_output_path.as_ref().unwrap()).unwrap()
        }



        // do not do anything if not called via volume snapshot refinement or directory browser context menu
        if !self.context.is_supported_operation(&[XtOpType::ActionVolumeSnapshotRefinement, XtOpType::DirectoryBrowserContextMenu]) {
            xwferror!("Operation Mode not supported. Please run the plugin via volume snapshot refinement \
            or via context menu of directory browser");
            return Ok(XtPrepareReturn::Negative(XtPrepareNegativeReturn::JustCallXtFinalize));
        }

        //exit with positive return code and signalize that we want XT_ProcessItem(Ex) to be called
        Ok(XtPrepareReturn::Positive(
            XtPreparePositiveReturnFlags::CallProcessItem
                .union(XtPreparePositiveReturnFlags::CallProcessItemLate)
        ))
    }

    fn xt_process_item_ex(&mut self, _handle: ItemHandle) -> Result<XtProcessItemExReturn, Self::XTensionError> {
        let uid = _handle.item().unique_id(self.context.get_evidence().unwrap());

        let _hash = _handle.item().get_hash_value(HashType::MD5, false).unwrap_or(Vec::new());
        let p = Path::new(self.current_output_path.as_ref().unwrap()).join(uid.to_string());
        _handle.write_to_file(p)?;

        workload();


        Ok(XtProcessItemExReturn::Ok)

    }

    fn xt_finalize(&mut self) -> Result<XtFinalizeReturn, Self::XTensionError> {
        self.current_output_path = None;

        Ok(XtFinalizeReturn::Ok)
    }
}

//macro to automatically define/register all low-level C functions for dynamic library. always needed
//first argument represents the name of a static variable. Can be anything.
//second argument is the name of your XTension struct
//also a similar export_all_functions_ex! macro available,
//that defines the XT_ProcessItemEx() function instead of XT_ProcessItem()
export_all_functions_ex!(MULTITHREAD_XTENSION, MultiThreadXtension);