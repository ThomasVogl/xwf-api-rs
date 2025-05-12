use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xwf_api_rs::{export_all_functions, xwf_types::*, traits::XTension, error::XwfError, evidence::Evidence, case::Case, item::Item};
use xwf_api_rs::context::ExecutionContext;

// define a custom structure representing your extension
// could also have attributes of course
pub struct CountItemsXTension {
    context: ExecutionContext,
    case: Case
}

pub fn parse_classification(_case: &Case, evidence: &Evidence, item: &Item, log_file: &RefCell<File>) -> Result<(), XwfError> {

    let item_type = match item.get_item_type(false) {
        Ok(type_str) => {
            type_str
        }
        Err(e) => {
            e.to_string()
        }
    };

    let classification =match item.get_item_info_classification() {
        Ok(class) => {
            class.to_string()
        }

        Err(e) => {
            e.to_string()

        }
    };

    let deletion =match item.get_item_info_deletion() {
        Ok(deletion) => {
            deletion.to_string()
        }

        Err(e) => {
            e.to_string()

        }
    };

    let category = match item.get_item_category() {
        Ok(cat) => {
            format!("{:?}", cat)
        }
        Err(e) => {
            e.to_string()
        }
    };

    let flags = match item.get_item_info_flags() {
        Ok(flags) => {
            format!("{:?}", flags)
        }
        Err(e) => {
            e.to_string()
        }
    };


    log_file.borrow_mut().write_fmt(format_args!("{};{};{};{};{};{};{};{}\r\n",
                                                 item.unique_id(evidence),
                                                 item.get_path()?,
                                                 item_type,
                                                 item.get_size(),
                                                 classification,
                                                 deletion,
                                                 category,
                                                 flags))
        .map_err(|e| {XwfError::IoError(e)})?;

    Ok(())
}

impl CountItemsXTension {

}

// implement XTension Trait
// most of the function have a default and empty implementation already
impl XTension for CountItemsXTension {

    // define your error type here. You can also define you own error type or use predefined "XwfError"
    type XTensionError = XwfError;

    // function to create an instance of your XTension struct
    fn create(context: ExecutionContext) -> CountItemsXTension {
        CountItemsXTension {
            context,
            case: Case::new()
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
        Ok(XtInitReturn::RunSingleThreaded)
    }

    //prepare function wraps XT_Prepare() Function from C API
    //please refer to X-Ways X-Tension API doc for details regarding calling logic
    fn xt_prepare(&mut self) -> Result<XtPrepareReturn, Self::XTensionError> {
        let case_infos =Case::get_case_infos()?;
        let log_file_path = Path::new(&case_infos.dir)
            .parent().unwrap()
            .join(case_infos.title + "_items")
            .with_extension("csv");

        if log_file_path.exists() {
            std::fs::remove_file(log_file_path.clone())
                .map_err(|e| XwfError::IoError(e))?;
        }

        let log_file = RefCell::new(File::create(log_file_path.clone()).map_err(|e| XwfError::IoError(e))?);
        log_file.borrow_mut().write("UniqueId;Path;FileType;FileSize;ItemInfoClassification;ItemInfoDeletion;(FileTypeStatus,FileFormatConsistency,FileTypeCategory);ItemInfoFlags;\r\n".as_bytes())
            .map_err(|e| XwfError::IoError(e))?;

        if self.context.is_supported_operation(&[XtOpType::ActionRun]) {
            let _ = self.case.iterate_ext(|case, evidence, item| {parse_classification(&case, evidence, item, &log_file)})?;

        }

        //exit with negative return code as we just want to
        Ok(XtPrepareReturn::Negative(XtPrepareNegativeReturn::JustCallXtFinalize))
    }

}

//macro to automatically define/register all low-level C functions for dynamic library. always needed
//first argument represents the name of a static variable. Can be anything.
//second argument is the name of your XTension struct
//also a similar export_all_functions_ex! macro available,
//that defines the XT_ProcessItemEx() function instead of XT_ProcessItem()
export_all_functions!(COUNT_ITEMS_XTENSION, CountItemsXTension);