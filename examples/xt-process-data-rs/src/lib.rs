use std::fs;
use std::path::{Path, PathBuf};
use xwf_api_rs::{xwferror, xwf_types::*, traits::XTension, error::XwfError, window::Window, evidence::Evidence, volume::Volume, export_all_functions_ex};
use xwf_api_rs::case::Case;
use xwf_api_rs::item::ItemHandle;


const JPG_HEADER: [u8;10] = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46];

// define a custom structure representing your extension
// could also have attributes of course
pub struct ProcessDataXtension {
    current_evidence: Option<Evidence>,
    current_volume: Option<Volume>,
    current_output_path: Option<PathBuf>
}

// implement XTension Trait
// most of the function have a default and empty implementation already
impl XTension for ProcessDataXtension {

    // define your error type here. You can also define you own error type or use predefined "XwfError"
    type XTensionError = XwfError;

    // function to create an instance of your XTension struct
    fn create() -> ProcessDataXtension {
        ProcessDataXtension {
            current_evidence: None,
            current_volume: None,
            current_output_path: None
        }
    }

    //function to initialize the X-Tension. Wraps XT_Init() Function from C API
    fn xt_init(&mut self, _version: XtVersion, _: XtInitFlags, _: Option<Window>, _: XtLicenseInfo) -> Result<XtInitReturn, Self::XTensionError> {

        Ok(XtInitReturn::RunSingleThreaded)
    }

    //prepare function wraps XT_Prepare() Function from C API
    //please refer to X-Ways X-Tension API doc for details regarding calling logic
    fn xt_prepare(&mut self, volume: Option<Volume>, evidence: Option<Evidence>, op_type: XtPrepareOpType) -> Result<XtPrepareReturn, Self::XTensionError> {

        // store current evidence and volume as we need this during xt_process_item_ex() function...
        self.current_volume = volume;
        self.current_evidence = evidence;

        // compute current output path based on case directory by using X-Tension name and current evidence name
        let case_infos = Case::get_case_infos()?;
        let mut output_path = Path::new(&case_infos.dir).join(env!("CARGO_PKG_NAME"));
        if let Some(ev) = &self.current_evidence {
            output_path = output_path.join(ev.get_name()?);
        }
        self.current_output_path = Some(output_path);

        // create output path if not existent
        if !self.current_output_path.as_ref().unwrap().exists() {
            fs::create_dir_all(self.current_output_path.as_ref().unwrap()).unwrap()
        }


        // do not do anything if not called via volume snapshot refinement or directory browser context menu
        if op_type != XtPrepareOpType::ActionVolumeSnapshotRefinement && op_type != XtPrepareOpType::DirectoryBrowserContextMenu {
            xwferror!("Operation Mode not supported. Please run the plugin via volume snapshot refinement \
            or via context menu of directory browser");
            return Ok(XtPrepareReturn::Negative(XtPrepareNegativeReturn::JustCallXtFinalize));
        }

        //exit with negative return code as we just want to
        Ok(XtPrepareReturn::Positive(
            XtPreparePositiveReturnFlags::CallProcessItem
                .union(XtPreparePositiveReturnFlags::CallProcessItemLate)
        ))
    }

    fn xt_process_item_ex(&mut self, handle: ItemHandle) -> Result<XtProcessItemExReturn, Self::XTensionError> {
        // check if current_evidence and current_output_path were set by xt_prepare
        // this should always be the case, but just to be sure...
        if let Some(ev) = self.current_evidence.as_ref() {
            if let Some(output_path) = self.current_output_path.as_ref() {

                //get item object from handle
                let item = handle.item();

                // get unique id from items
                // unique id contains also evidence id to be able to uniquely identify item across evidences
                let item_uid = item.unique_id(ev);

                // get file type
                let file_type = item.get_item_type(false)?;

                // get first bytes of file to check if contains a JPG header
                if let Some(header) = handle.read_chunk(0, JPG_HEADER.len()) {
                    if header.eq(&JPG_HEADER) {
                        // construct destination path by unique id and item type
                        let output_file = output_path.join(item_uid.to_string() + "." + &file_type);

                        // write item data to output file
                        handle.write_to_file(output_file)?
                    }
                }
            }

        };

        Ok(XtProcessItemExReturn::Ok)

    }

    fn xt_finalize(&mut self, _volume: Option<Volume>, _evidence: Option<Evidence>, _op_type: XtPrepareOpType) -> Result<XtFinalizeReturn, Self::XTensionError> {
        self.current_volume = None;
        self.current_evidence = None;
        self.current_output_path = None;

        Ok(XtFinalizeReturn::Ok)
    }
}

//macro to automatically define/register all low-level C functions for dynamic library. always needed
//first argument represents the name of a static variable. Can be anything.
//second argument is the name of your XTension struct
//also a similar export_all_functions_ex! macro available,
//that defines the XT_ProcessItemEx() function instead of XT_ProcessItem()
export_all_functions_ex!(PROCESS_DATA_XTENSION, ProcessDataXtension);