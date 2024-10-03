use crate::xwf_types::*;

impl Into<i32> for XtPrepareNegativeReturn {
    fn into(self) -> i32 {
        match self {
            XtPrepareNegativeReturn::StopWholeOperation => { XtPrepareNegativeReturn::StopWholeOperation as i32 }
            XtPrepareNegativeReturn::PreventFurtherUse => { XtPrepareNegativeReturn::PreventFurtherUse as i32 }
            XtPrepareNegativeReturn::ExcludeVolume => { XtPrepareNegativeReturn::ExcludeVolume as i32 }
            XtPrepareNegativeReturn::DontCallOtherFunctions => { XtPrepareNegativeReturn::DontCallOtherFunctions as i32 }
            XtPrepareNegativeReturn::JustCallXtFinalize => { XtPrepareNegativeReturn::JustCallXtFinalize as i32}
        }
    }
}


impl Into<i32> for XtPrepareReturn {
    fn into(self) -> i32 {
        match self {
            XtPrepareReturn::Negative(x) => {x.into() },
            XtPrepareReturn::Positive(x) => { x.bits() }
        }
    }
}

impl Into<i32> for XtProcessItemReturn {
    fn into(self) -> i32 {
        match self {
            XtProcessItemReturn::StopCurrentOperation => { XtProcessItemReturn::StopCurrentOperation as i32}
            XtProcessItemReturn::SkipAllOtherVSRefinements => { XtProcessItemReturn::SkipAllOtherVSRefinements as i32 }
            XtProcessItemReturn::Ok => { XtProcessItemReturn::Ok as i32}
        }
    }
}


impl Into<i32> for XtProcessItemExReturn {
    fn into(self) -> i32 {
        match self {
            XtProcessItemExReturn::StopCurrentOperation => { XtProcessItemExReturn::StopCurrentOperation as i32}
            XtProcessItemExReturn::Ok => { XtProcessItemExReturn::Ok as i32 }
        }
    }
}


impl TryFrom<i64> for ItemInfoClassification {
    type Error = XwfError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value = value & 0xFFi64;
        match value {
            x if x == Self::NormalFile as i64                       => Ok(Self::NormalFile),
            x if x == Self::HfsResourceFork as i64                  => Ok(Self::HfsResourceFork),
            x if x == Self::NtfsAlternateDataStream as i64          => Ok(Self::NtfsAlternateDataStream),
            x if x == Self::NtfsNonDirectoryIndex as i64            => Ok(Self::NtfsNonDirectoryIndex),
            x if x == Self::NtfsBitmapAttribute as i64              => Ok(Self::NtfsBitmapAttribute),
            x if x == Self::NotDocumented1 as i64                   => Ok(Self::NotDocumented1),
            x if x == Self::NtfsGeneralLoggedUtilityStream as i64   => Ok(Self::NtfsGeneralLoggedUtilityStream),
            x if x == Self::NtfsEfsLoggedUtilityStream as i64       => Ok(Self::NtfsEfsLoggedUtilityStream),
            x if x == Self::NotDocumented2 as i64                   => Ok(Self::NotDocumented2),
            x if x == Self::NotDocumented3 as i64                   => Ok(Self::NotDocumented3),
            x if x == Self::NotDocumented4 as i64                   => Ok(Self::NotDocumented4),
            x if x == Self::EmailRelated as i64                     => Ok(Self::EmailRelated),
            x if x == Self::Excerpt as i64                          => Ok(Self::Excerpt),
            x if x == Self::ManuallyAttached as i64                 => Ok(Self::ManuallyAttached),
            x if x == Self::VideoStill as i64                       => Ok(Self::VideoStill),
            x if x == Self::EmailAttachment as i64                  => Ok(Self::EmailAttachment),
            x if x == Self::EmailMessage as i64                     => Ok(Self::EmailMessage),
            x if x == Self::IdnxRecordRemnant as i64                => Ok(Self::IdnxRecordRemnant),
            _ => Ok(Self::Unknown)
        }
    }
}

impl Into<i32> for XtFinalizeReturn {
    fn into(self) -> i32 {
        match self {
            XtFinalizeReturn::RefreshDirectoryListing => {XtFinalizeReturn::RefreshDirectoryListing as i32}
            XtFinalizeReturn::Ok => {XtFinalizeReturn::Ok as i32}
        }
    }
}

impl TryFrom<u32> for XtVersion {
    type Error = ();

    fn try_from(value: u32) -> Result<XtVersion, Self::Error> {
        if value == 0 {
            return Err(());
        }
        let version = ((value & 0xFFFF0000) >> 16) as u16;
        if version == 0 {
            return Err(());
        }
        Ok(XtVersion {
            major: (version / 100),
            minor: (version % 100),
            service_release: ((value & 0x0000FF00) >> 8) as u8,
            language: ((value & 0x000000FF) >> 0) as u8,
        })
    }
}

impl TryFrom<u32> for XtPrepareOpType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == XtPrepareOpType::ActionRun as u32 => Ok(XtPrepareOpType::ActionRun),
            x if x == XtPrepareOpType::ActionVolumeSnapshotRefinement as u32 => Ok(XtPrepareOpType::ActionVolumeSnapshotRefinement),
            x if x == XtPrepareOpType::ActionLogicalSearch as u32 => Ok(XtPrepareOpType::ActionLogicalSearch),
            x if x == XtPrepareOpType::ActionPhysicalSearch as u32 => Ok(XtPrepareOpType::ActionPhysicalSearch),
            x if x == XtPrepareOpType::DirectoryBrowserContextMenu as u32 => Ok(XtPrepareOpType::DirectoryBrowserContextMenu),
            x if x == XtPrepareOpType::SearchHitListContextMenu as u32 => Ok(XtPrepareOpType::SearchHitListContextMenu),
            x if x == XtPrepareOpType::EventListContextMenu as u32 => Ok(XtPrepareOpType::EventListContextMenu),
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for FileFormatConsistency {
    type Error = XwfError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(XwfError::InvalidEnumValue(("FileFormatConsistency", value as i64)));
        }

        let val = (value & 0x0000FF00) >> 8;
        match val {
            x if x == FileFormatConsistency::Ok as i32 => Ok(FileFormatConsistency::Ok),
            x if x == FileFormatConsistency::Irregular as i32 => Ok(FileFormatConsistency::Irregular),
            x if x == FileFormatConsistency::Unknown as i32 => Ok(FileFormatConsistency::Unknown),
            x if x == FileFormatConsistency::NotDocumented as i32 => Ok(FileFormatConsistency::NotDocumented),
            _ => Err(XwfError::InvalidEnumValue(("FileFormatConsistency", value as i64)))
        }
    }
}

impl TryFrom<i64> for ItemInfoDeletion {
    type Error = XwfError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            x if x == ItemInfoDeletion::Existing as i64 => Ok(ItemInfoDeletion::Existing),
            x if x == ItemInfoDeletion::PossiblyReverable as i64 => Ok(ItemInfoDeletion::PossiblyReverable),
            x if x == ItemInfoDeletion::FirstClusterUnknown as i64 => Ok(ItemInfoDeletion::FirstClusterUnknown),
            x if x == ItemInfoDeletion::MovedPossibleRecoverable as i64 => Ok(ItemInfoDeletion::MovedPossibleRecoverable),
            x if x == ItemInfoDeletion::MovedFirstClusterUnknown as i64 => Ok(ItemInfoDeletion::MovedFirstClusterUnknown),
            x if x == ItemInfoDeletion::CarvedFile as i64 => Ok(ItemInfoDeletion::CarvedFile),
            _ => Err(XwfError::InvalidEnumValue(("ItemInfoDeletion",value)))
        }
    }
}

impl TryFrom<i32> for FileTypeStatus {
    type Error = XwfError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(XwfError::InvalidEnumValue(("FileTypeStatus", value as i64)));
        }
        let val = value & 0xFF;
        match val {
            x if x == FileTypeStatus::NotVerified as i32 => Ok(FileTypeStatus::NotVerified),
            x if x == FileTypeStatus::TooSmall as i32 => Ok(FileTypeStatus::TooSmall),
            x if x == FileTypeStatus::TotallyUnknown as i32 => Ok(FileTypeStatus::TotallyUnknown),
            x if x == FileTypeStatus::Confirmed as i32 => Ok(FileTypeStatus::Confirmed),
            x if x == FileTypeStatus::NotConfirmed as i32 => Ok(FileTypeStatus::NotConfirmed),
            x if x == FileTypeStatus::NewlyIdentified as i32 => Ok(FileTypeStatus::NewlyIdentified),
            x if x == FileTypeStatus::MismatchDetected as i32 => Ok(FileTypeStatus::MismatchDetected),
            _ => Err(XwfError::InvalidEnumValue(("FileTypeStatus", val as i64)))
        }
    }
}

impl From<String> for FileTypeCategory {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {


            "pictures" => FileTypeCategory::Picture,
            "text, word processing" => FileTypeCategory::Word,
            "e-mail" => FileTypeCategory::Email,
            "internet" => FileTypeCategory::Internet,
            "page layout" => FileTypeCategory::PageLayout,
            "spreadsheet" => FileTypeCategory::Spreadsheet,
            "misc documents" => FileTypeCategory::Misc,
            "plain text" => FileTypeCategory::Text,
            "archives/backup" => FileTypeCategory::Archive,
            "audio" => FileTypeCategory::Audio,
            "video" => FileTypeCategory::Video,
            "windows internals" => FileTypeCategory::WindowsInternal,
            "thumbnails/icons" => FileTypeCategory::Thumbnail,
            "database, finance" => FileTypeCategory::Database,
            "programs" => FileTypeCategory::Program,
            "mobile phones" => FileTypeCategory::MobilePhone,
            "chat, messaging" => FileTypeCategory::Chat,
            "address book" => FileTypeCategory::AddressBook,
            "mac os x/ios system" => FileTypeCategory::MacOsXIos,
            "cad" => FileTypeCategory::Cad,
            "various data" => FileTypeCategory::VariousData,
            "gps/navigation" => FileTypeCategory::Gps,
            "disk image" => FileTypeCategory::DiskImage,
            "source code" => FileTypeCategory::SourceCode,
            "cryptography" => FileTypeCategory::Cryptography,
            "windows registry" => FileTypeCategory::WindowsRegistry,
            "p2p" => FileTypeCategory::P2P,
            "ebook" => FileTypeCategory::Ebook,
            "3d graphics" => FileTypeCategory::Graphics3D,
            "projects" => FileTypeCategory::Projects,
            "unix/linux system" => FileTypeCategory::UnixLinux,
            "fonts" => FileTypeCategory::UnixLinux,
            "still image" => FileTypeCategory::StillImage,
            "other/unknown type" => FileTypeCategory::Unknown,
            "anderer/unbek. typ" => FileTypeCategory::Unknown,
            _ => FileTypeCategory::Other
        }
    }
}