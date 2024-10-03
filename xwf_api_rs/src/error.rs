use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;
use crate::xwf_types::XtVersion;

#[derive(Debug)]
pub enum XwfError {
    InvalidInputArgument,
    InputHandleIsNull,
    XwfFunctionCallFailed(&'static str),
    FailedToGetObjectHandle,
    FailedToSelectVolume,
    InvalidEnumValue((&'static str, i64)),
    InvalidItemSize,
    ReadItemDataFailed,
    HashValueNotAvailable,
    GivenBufferToSmallForContent,
    NoEvidenceAvaible,
    OperationAbortedByUser,
    MaxItemIdExceeded,
    InvalidVersionNumber,
    IncompatibleXwfVersion(XtVersion, (u16, u16)),
    IoError(io::Error),
}


impl Display for XwfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            XwfError::InvalidInputArgument => write!(f, "invalid input for argument"),
            XwfError::InputHandleIsNull => write!(f, "input handle is null"),
            XwfError::XwfFunctionCallFailed(func) => write!(f, "XWF function call {} failed", func),
            XwfError::FailedToGetObjectHandle => write!(f, "failed to get object handle"),
            XwfError::FailedToSelectVolume => write!(f, "failed to select volume"),
            XwfError::InvalidEnumValue(x) => write!(f, "invalid/undefined enum value {} for type {}", x.1, x.0),
            XwfError::InvalidItemSize => write!(f, "invalid item size"),
            XwfError::ReadItemDataFailed => write!(f, "failed to read item data"),
            XwfError::HashValueNotAvailable => write!(f, "expected hash value is not available"),
            XwfError::GivenBufferToSmallForContent => write!(f, "given buffer to XWF was to small for the content"),
            XwfError::NoEvidenceAvaible => write!(f, "expected an evidence for processing"),
            XwfError::OperationAbortedByUser => write!(f, "current operation aborted by user"),
            XwfError::MaxItemIdExceeded => write!(f, "maximum ItemID exceeded (max of int32). This should never happen, but if it does anyway, it would probably lead to unexpected behaviour"),
            XwfError::InvalidVersionNumber => write!(f, "invalid X-Ways Version Number detected"),
            XwfError::IncompatibleXwfVersion(version, expected) => write!(f, "XWF Incompatibility detected. \
            Current Version {}.{} SR-{}, minimal required version {}.{} \
            Consider upgrading XWF or downgrading API level of xwf-api-rs (feature \"api_<major>_<minor>\")", version.major, version.minor, version.service_release,  expected.0, expected.1),
            XwfError::IoError(e) => write!(f, "io error occurred: {}", e),
        }
    }
}

impl Error for XwfError {}