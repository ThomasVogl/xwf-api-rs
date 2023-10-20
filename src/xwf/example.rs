
use crate::xwf::api::traits::XTension;


pub struct ExampleLib {}

impl XTension for ExampleLib {



    fn xtension_version(&self) -> (u8, u8, u8) { (0,1,0) }

    fn xtension_name(&self) -> String { format!("Example Extension") }

}


impl ExampleLib {
    pub const fn create() -> ExampleLib {
        ExampleLib {}
    }
}