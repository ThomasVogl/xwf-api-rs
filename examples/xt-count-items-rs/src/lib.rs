use xwf_api_rs::{
    export_all_functions,
    xwfinfo,
    xwf_types::*,
    traits::XTension,
    error::XwfError,
    window::Window,
    evidence::Evidence,
    case::Case,
    item::Item,
    volume::Volume
};

// define a custom structure representing your extension
// could also have attributes of course
pub struct CountItemsXTension {
    case: Case
}

impl CountItemsXTension {
    pub fn get_item_category(&self, item: Item) -> Result<Option<FileTypeCategory>, XwfError> {
        let item_flags = item.get_item_info_flags()?;
        let (_status, _consistency, category) = item.get_item_category()?;
        let item_class = item.get_item_info_classification()?;

        if item_flags.contains(ItemInfoFlags::IsVirtualItem) {
            return Ok(None)
        }

        if item_class == ItemInfoClassification::VideoStill {
            Ok(None)
        } else {
            Ok(Some(category))
        }
    }
}

// implement XTension Trait
// most of the function have a default and empty implementation already
impl XTension for CountItemsXTension {

    // define your error type here. You can also define you own error type or use predefined "XwfError"
    type XTensionError = XwfError;

    // function to create an instance of your XTension struct
    fn create() -> CountItemsXTension {
        CountItemsXTension {
            case: Case::new()
        }
    }

    //function to initialize the X-Tension. Wraps XT_Init() Function from C API
    fn xt_init(&mut self, _version: XtVersion, _: XtInitFlags, _: Option<Window>, _: XtLicenseInfo) -> Result<XtInitReturn, Self::XTensionError> {
        // compute cache for report table assignments
        // optimizes requests for getting report tables for single item
        self.case.compute_report_table_cache()?;
        Ok(XtInitReturn::RunSingleThreaded)
    }

    //prepare function wraps XT_Prepare() Function from C API
    //please refer to X-Ways X-Tension API doc for details regarding calling logic
    fn xt_prepare(&mut self, _: Option<Volume>, _: Option<Evidence>, op_type: XtPrepareOpType) -> Result<XtPrepareReturn, Self::XTensionError> {

        if op_type == XtPrepareOpType::ActionRun {
            let categories:Vec<FileTypeCategory> = Case::iterate(|i| self.get_item_category(i))?
                .iter()
                .filter_map(|&c| c ).collect();

            let num_videos = categories.iter().filter(|&c| *c == FileTypeCategory::Video).count();
            let num_images = categories.iter().clone().filter(|&c| *c == FileTypeCategory::Picture).count();
            let num_other = categories.iter().filter(|&c| *c != FileTypeCategory::Video && *c != FileTypeCategory::Picture).count();

            xwfinfo!("number of regular files in the case:");
            xwfinfo!("-> {:<7} Videos", num_videos);
            xwfinfo!("-> {:<7} Images", num_images);
            xwfinfo!("-> {:<7} Other Files", num_other);

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
export_all_functions!(HELLO_WORLD_LIB, CountItemsXTension);