use winapi::um::processenv::ExpandEnvironmentStringsA;
use crate::xwf::example::ExampleLib;

pub mod xwf;

needed_use_declarations!();
export_all_functions_ex!(EXAMPLE_LIB, ExampleLib);












