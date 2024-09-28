use xwf_api_rs::{
    xwferror,
    export_all_functions,
    xwf_types::*,
    traits::XTension,
    error::XwfError,
    window::Window,
    evidence::Evidence,
    volume::Volume,
    application::Application
};

// define a custom structure representing your extension
// could also have attributes of course
pub struct HelloWorldXTension {}

// implement XTension Trait
// most of the function have a default and empty implementation already
impl XTension for HelloWorldXTension {

    // define your error type here. You can also define you own error type or use predefined "XwfError"
    type XTensionError = XwfError;

    // function to create an instance of your XTension struct
    fn create() -> HelloWorldXTension {
        HelloWorldXTension {}
    }

    //function to initialize the X-Tension. Wraps XT_Init() Function from C API
    fn xt_init(&mut self, _version: XtVersion, _: XtInitFlags, _: Option<Window>, _: XtLicenseInfo) -> Result<XtInitReturn, Self::XTensionError> {
        Ok(XtInitReturn::RunSingleThreaded)
    }

    //prepare function wraps XT_Prepare() Function from C API
    //please refer to X-Ways X-Tension API doc for details regarding calling logic
    fn xt_prepare(&mut self, _: Option<Volume>, _: Option<Evidence>, op_type: XtPrepareOpType) -> Result<XtPrepareReturn, Self::XTensionError> {


        //check if we were called from main menu
        //if not then we will do nothing...
        if op_type != XtPrepareOpType::ActionRun {
            xwferror!("Operation Mode not supported");
            return Ok(XtPrepareReturn::Negative(XtPrepareNegativeReturn::JustCallXtFinalize));
        }

        //request a string input from user, use default string if nothing entered
        let title = Application::get_user_input_str("enter a title".to_string(), true)
            .or(Some("HelloWorld".to_string())).unwrap();

        //show progress bar
        Application::show_progress(title, ProgressFlags::empty());


        //request integer number from user and compute number of rounds
        let num_rounds = (Application::get_user_input_integer("seconds to run".to_string())
            .or(Some(1000)).unwrap()*100) as u32;

        //iterate over number of rounds
        for i in 0..num_rounds as u32 {

            //check if user wants to stop operation
            if Application::should_stop().is_err() {
                break;
            }

            //just wait 10 microseconds
            std::thread::sleep(std::time::Duration::from_millis(10));

            //set progress and description text of progress bar
            Application::set_progress_percentage(i, num_rounds as u32);
            Application::set_progress_description(format!("{:.2} seconds have passed", i as f32/100.0f32));
        }

        //hide progress bar again
        Application::hide_progress();

        //exit with negative return code as we just want to
        return Ok(XtPrepareReturn::Negative(XtPrepareNegativeReturn::JustCallXtFinalize));
    }
}

//macro to automatically define/register all low-level C functions for dynamic library. always needed
//first argument represents the name of a static variable. Can be anything.
//second argument is the name of your XTension struct
//also a similar export_all_functions_ex! macro available,
//that defines the XT_ProcessItemEx() function instead of XT_ProcessItem()
export_all_functions!(HELLO_WORLD_LIB, HelloWorldXTension);