
use std::ptr::null_mut;

use crate::xwf::api::{traits::XTension, volume::Volume};


pub struct ExampleLib {}

impl XTension for ExampleLib {

    type XTensionError = &'static str;

    fn create() -> ExampleLib {
        ExampleLib {}
    }

    fn xtension_version(&self) -> (u8, u8, u8) { (0,1,0) }

    fn xtension_name(&self) -> String { format!("Example Extension")
     }

    

}