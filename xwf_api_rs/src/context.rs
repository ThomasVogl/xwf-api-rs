use crate::case::Case;
use crate::error::XwfError;
use crate::evidence::Evidence;
use crate::volume::Volume;
use crate::window::Window;
use crate::xwf_types::{XtInitFlags, XtLicenseInfo, XtOpType, XtVersion};
use crate::{xwferror, xwfinfo};

pub struct ExecutionContext {
    case: Case,
    evidence: Option<Evidence>,
    volume: Option<Volume>,
    operation_type: Option<XtOpType>,

    pub version: XtVersion,
    pub flags: XtInitFlags,
    pub license_info: XtLicenseInfo,
    pub window: Option<Window>,
}


impl ExecutionContext {

    pub fn create(version: XtVersion, flags: XtInitFlags, license_info: XtLicenseInfo, window: Option<Window>) -> Result<Self, XwfError> {
        let mut ret = Self{
            case: Case::new(),
            evidence: None,
            volume: None,
            operation_type: None,
            version,
            flags,
            license_info,
            window,
        };
        ret.check_supported_xwf_version()?;
        ret.case.compute_report_table_cache()?;

        Ok(ret)
    }

    pub fn set(&mut self, evidence: Option<Evidence>, volume: Option<Volume>, op_type: Option<XtOpType>) {
        self.evidence = evidence;
        self.volume = volume;
        self.operation_type = op_type;
    }

    pub fn is_set(&self) -> bool {
        self.evidence.is_some() && self.volume.is_some() && self.operation_type.is_some()
    }

    pub fn reset(&mut self) {
        self.evidence = None;
        self.volume = None;
        self.operation_type = None;
    }

    pub fn reset_volume_evidence(&mut self) {
        self.evidence = None;
        self.volume = None;
    }

    pub fn get_evidence(&self) -> Option<&Evidence> {
        self.evidence.as_ref()
    }

    pub fn get_volume(&self) -> Option<&Volume> {
        self.volume.as_ref()
    }

    pub fn get_operation_type(&self) -> Option<&XtOpType> {
        self.operation_type.as_ref()
    }

    pub fn get_case(&self) -> &Case {
        &self.case
    }

    pub fn is_real_run(&self) -> bool {
        !self.flags.contains(XtInitFlags::IsAboutOnly)
            && !self.flags.contains(XtInitFlags::IsQuickcheck)
    }

    pub fn is_supported_operation(&self, vec: &[XtOpType]) -> bool {
        if let Some(op_type) = &self.operation_type {
            for i in vec {
                if *i == *op_type {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }


    fn check_supported_xwf_version(&self) -> Result<(), XwfError> {
        const MIN_VERSION: (u16, u16) = {
            #[cfg(feature = "api_21_6")]
            { (21, 6) }
            #[cfg(feature = "api_21_5")]
            { (21, 5) }
            #[cfg(feature = "api_21_4")]
            { (21, 4) }
            #[cfg(feature = "api_21_3")]
            { (21, 3) }
            #[cfg(not(feature = "api_21_3"))]
            #[cfg(feature = "api_21_2")]
            { (21, 2) }
            #[cfg(not(feature = "api_21_2"))]
            #[cfg(feature = "api_21_1")]
            { (21, 1) }
            #[cfg(not(feature = "api_21_1"))]
            #[cfg(feature = "api_21_0")]
            { (21, 0) }
            #[cfg(not(feature = "api_21_0"))]
            #[cfg(feature = "api_20_9")]
            { (20, 9) }
            #[cfg(not(feature = "api_20_9"))]
            #[cfg(feature = "api_20_8")]
            { (20, 8) }
            #[cfg(not(feature = "api_20_8"))]
            #[cfg(feature = "api_20_7")]
            { (20, 7) }
            #[cfg(not(feature = "api_20_7"))]
            #[cfg(feature = "api_20_6")]
            { (20, 6) }
            #[cfg(not(feature = "api_20_6"))]
            #[cfg(feature = "api_20_5")]
            { (20, 5) }
            #[cfg(not(feature = "api_20_5"))]
            #[cfg(feature = "api_20_4")]
            { (20, 4) }
            #[cfg(not(feature = "api_20_4"))]
            #[cfg(feature = "api_20_3")]
            { (20, 3) }
            #[cfg(not(feature = "api_20_3"))]
            #[cfg(feature = "api_20_2")]
            { (20, 2) }
            #[cfg(not(feature = "api_20_2"))]
            #[cfg(feature = "api_20_1")]
            { (20, 1) }
            #[cfg(not(feature = "api_20_1"))]
            { (20, 0) }
        };


        #[cfg(not(feature = "api_20_1"))] {
            xwfwarn!("No API level defined. Assuming minimum supported XWF Version 20.0 of xwf-api-rs crate.");
            xwfwarn!("You should define your minimal required API Level of xwf-api-rs (via feature api_<major>_<minor>).");
            xwfwarn!("For differences of XWF API, also consult official X-Tension API Documentation \
        https://www.x-ways.net/forensics/x-tensions/api.html");
        }

        if self.version.major < MIN_VERSION.0 || (self.version.major == MIN_VERSION.0 && self.version.minor < MIN_VERSION.1) {
            xwferror!("X-Tension API version check failed. ({}.{} < {}.{})", self.version.major, self.version.minor, MIN_VERSION.0, MIN_VERSION.1, );
            Err(XwfError::IncompatibleXwfVersion(self.version.clone(), MIN_VERSION))
        } else {
            xwfinfo!("X-Tension API version check successful. ({}.{} >= {}.{})", self.version.major, self.version.minor, MIN_VERSION.0, MIN_VERSION.1, );
            Ok(())
        }
    }
}