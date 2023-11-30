use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum XwfError {
    InvalidInputArgument,
    InputHandleIsNull,
    XwfFunctionCallFailed,
    FailedToGetObjectHandle,
    FailedToSelectVolume,
    InvalidEnumValue,
    InvalidItemSize,
    ReadItemDataFailed,
}


impl Display for XwfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XwfError::InvalidInputArgument => write!(f, "invalid input for argument"),
            XwfError::InputHandleIsNull => write!(f, "input handle is null"),
            XwfError::XwfFunctionCallFailed => write!(f, "XWF function call failed"),
            XwfError::FailedToGetObjectHandle => write!(f, "failed to get object handle"),
            XwfError::FailedToSelectVolume => write!(f, "failed to select volume"),
            XwfError::InvalidEnumValue => write!(f, "invalid/undefined enum value"),
            XwfError::InvalidItemSize => write!(f, "invalid item size"),
            XwfError::ReadItemDataFailed => write!(f, "failed to read item data"),
        }
    }
}

impl Error for XwfError {}