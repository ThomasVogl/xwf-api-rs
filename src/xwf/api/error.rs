use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum XwfError {
    InvalidInputArgument,
    InputHandleIsNull,
    XwfFunctionCallFailed,
    FailedToGetObjectHandle,
    FailedToSelectVolume
}


impl Display for XwfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XwfError::InvalidInputArgument => write!(f, "invalid input for argument"),
            XwfError::InputHandleIsNull => write!(f, "input handle is null"),
            XwfError::XwfFunctionCallFailed => write!(f, "XWF function call failed"),
            XwfError::FailedToGetObjectHandle => write!(f, "failed to get object handle"),
            XwfError::FailedToSelectVolume => write!(f, "failed to select volume"),
        }
    }
}

impl Error for XwfError {}