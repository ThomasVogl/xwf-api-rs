use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::xwf_types::XtPreparePositiveReturnFlags;

pub enum XtPrepareNegativeReturn {
    StopWholeOperation          = -4,   // if you want X-Ways Forensics to stop the whole operation (e.g. volume snapshot refinement) altogether

    PreventFurtherUse           = -3,   // if you want to prevent further use of the X-Tension for the remainder of the whole operation,
    // for example because your X-Tension is not supposed to do anything for that kind of operation
    // as indicated by nOpType or because your X-Tension expects to be applied to a particular data window (requiring hVolume to be unequal to 0)
    ExcludeVolume               = -2,   //  if you want this particular volume excluded from the operation
    DontCallOtherFunctions      = -1,   // if you don't want other functions of this X-Tension to be called for this particular volume, not even XT_Finalize()
    // Full negative return value evaluation only for XT_ACTION_RVS.
    JustCallXtFinalize          =  0,   // 0 is the default return value, if you just want XT_Finalize() to be called. Will also be assumed if you do not export XT_Prepare().
}

pub enum XtPrepareReturn {
    Negative(XtPrepareNegativeReturn),
    Positive(XtPreparePositiveReturnFlags)
}


pub enum XtProcessItemReturn {
    StopCurrentOperation        = -1, //Return -1 if you want X-Ways Forensics to stop the current operation (e.g. volume snapshot refinement),
    SkipAllOtherVSRefinements   = -2, //-2 if you want have X-Ways Forensics skip all other volume snapshot refinement operations for this file
    Ok                          =  0, //otherwise 0.
}

pub enum XtProcessItemExReturn {
    StopCurrentOperation        = -1, //Return -1 if you want X-Ways Forensics to stop the current operation (e.g. volume snapshot refinement),
    Ok                          =  0, //otherwise 0.
}

pub enum PropType {
    PhysicalSize            = 0,
    LogicalSize             = 1,
    ValidDataLength         = 2, //valid data length of a file (a.k.a. initialized size of the data stream, which may be available from NTFS, exFAT, XWFS, XWFS2)
    FileAttributes          = 4,
    PointerFilePath         = 8,
    PointerName             = 9,
    ParentVolume            = 10,
    NumberOfDataWindow      = 16,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum ItemInfoClassification {
    NormalFile                            = 0x00, //normal file
    HfsResourceFork                       = 0x04, //HFS resource fork
    NtfsAlternateDataStream               = 0x08, //NTFS alternate data stream
    NtfsNonDirectoryIndex                 = 0x0A, //NTFS non-directory index
    NtfsBitmapAttribute                   = 0x0B, //NTFS bitmap attribute
    NtfsGeneralLoggedUtilityStream        = 0x10, //NTFS general logged utility stream
    NtfsEfsLoggedUtilityStream            = 0x11, //NTFS EFS logged utility stream
    EmailRelated                          = 0xF5, //e-mail related
    Excerpt                               = 0xF6, //excerpt
    ManuallyAttached                      = 0xF7, //manually attached
    VideoStill                            = 0xF8, //video still
    EmailAttachment                       = 0xF9, //e-mail attachment
    EmailMessage                          = 0xFA, //e-mail message
    IdnxRecordRemnant                     = 0xFD, //INDX record remnant
    UnknownEnumValue
}

pub enum EvObjPropType {
    ObjNumber           = 0,	//WORD	(unused)	ev. obj. number (simply reflects the order of evidence objects in the case tree and thus may change)
    ObjId               = 1,	//DWORD	(unused)	ev. obj. ID (used to identify parent-child relationships between evidence objects)
    ParentObjId         = 2,	//DWORD	(unused)	parent ev. obj. ID (available for partitions, 0 if no parent)
    ShortEvObjId        = 3,	//WORD	(unused)	short ev. obj. ID (concatenated with the ID of items in the volume snapshot gives the so-called unique ID shown for items in the directory browser, available in v18.8 SR-14, v18.9 SR-12, v19.0 SR-11 and later)
    VsSnapshotId        = 4,	//DWORD	(unused)	volume snapshot ID (increments by 1 when a new volume snapshot is taken, available in v19.0 SR-11 and later)
    ObjTitle            = 6,	//LPWSTR	(unused)	evidence object title (e.g. "Partition 2")
    ExtObjTitle         = 7,	//LONG	LPWSTR	extended ev. obj. title (e.g. "HD123, Partition 2), buffer len: MAX_PATH, returns the string length
    AbbrevObjTitle      = 8,	//LONG	LPWSTR	abbreviated ext. ev. obj. title (e.g. "HD123, P2), buffer len: MAX_PATH, returns the string length
    InternalName        = 9,	//LPWSTR	(unused)	internal name
    Description         = 10,	//LPWSTR	(unused)	description
    ExaminerComments    = 11,	//LPWSTR	(unused)	examiner comments
    IntUsedDir          = 12,	//LONG	LPWSTR	internally used directory (buffer length: MAX_PATH), returns the string length
    OutputDir           = 13,	//LONG	LPWSTR	output directory (buffer length: MAX_PATH), returns the string length
    SizeInBytes         = 16,	//INT64	(unused)	size in bytes
    VSFileCount         = 17,	//DWORD	(unused)	volume snapshot file count
    Flags               = 18,	//INT64	(unused)	flags*
    FileSystemID        = 19,	//INT64	(unused)	file system identifier (see XWF_GetVolumeInformation for possible values)
    HashType            = 20,	//DWORD	(unused)	hash type
    HashValue           = 21,	//DWORD	LPVOID	hash value (buffer size according to hash type), returns the hash size in bytes
    CreationTime        = 32,	//FILETIME	(unused)	creation time (when the ev. obj. was added to the case)
    ModificationTime    = 33,	//FILETIME	(unused)	modification time
    HashType2           = 40,	//DWORD	(unused)	hash type #2
    HashValue2          = 41,	//DWORD	LPVOID	hash value #2 (buffer size according to hash type), returns the hash size in bytes
    NumberOfDataWindow  = 50,	//WORD	LPVOID	the number of the data window that currently represents the evidence object, or 0 if the evidence object is not open, available in v19.9 SR-7 and later
}


#[derive(Debug, PartialEq, Eq)]
pub enum XtOpType {
    ActionRun                       = 0, // simply run directly from the main menu or command line3
    ActionVolumeSnapshotRefinement  = 1, // volume snapshot refinement starting2
    ActionLogicalSearch             = 2, // logical simultaneous search starting
    ActionPhysicalSearch            = 3, // physical simultaneous search starting
    DirectoryBrowserContextMenu     = 4, // directory browser context menu command invoked1
    SearchHitListContextMenu        = 5, // search hit list context menu command invoked
    EventListContextMenu            = 6 // event list context menu command invoked (since v20.3 SR-3)
}

pub enum XwfItemInfoTypes {
    OrigId                  = 1,
    Attr                    = 2,
    Flags                   = 3,
    Deletion                = 4,
    Classification          = 5,
    LinkCount               = 6,
    ColorAnalysis           = 7,
    PixelIndex              = 8,
    FileCount               = 11,
    EmbeddedOffset          = 16,
    CreationTime            = 32,
    ModificationTime        = 33,
    LastAccessTime          = 34,
    EntryModificationTime   = 35,
    DeletionTime            = 36,
    InternalCreationTime    = 37,
    #[cfg(feature = "api_21_2")]
    CreationTimeDisplayOfs          = 48,
    #[cfg(feature = "api_21_2")]
    ModificationTimeDisplayOfs      = 49,
    #[cfg(feature = "api_21_2")]
    LastAccessTimeDisplayOfs        = 50,
    #[cfg(feature = "api_21_2")]
    EntryModificationTimeDisplayOfs = 51,
    #[cfg(feature = "api_21_2")]
    DeletionTimeDisplayOfs          = 52,
    #[cfg(feature = "api_21_2")]
    InternalCreationTimeDisplayOfs  = 53,

}

pub struct XtLicenseInfo {}
#[derive(Debug, PartialEq, Eq)]
pub enum XtInitReturn {
    PreventFurtherUseOfDll = -1,
    RunSingleThreaded = 1,
    RunMultiThreaded = 2,
}

#[derive(Debug, PartialEq, Eq)]
pub enum XtFinalizeReturn {
    RefreshDirectoryListing = 1,
    Ok = 0
}

#[derive(Debug, Copy, Clone)]
pub struct XtVersion {
    pub major: u16,
    pub minor: u16,
    pub service_release: u8,
    pub language: u8,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum FileFormatConsistency {
    Unknown             = 0,
    Ok                  = 1,
    #[cfg(not(feature = "api_20_5"))]
    CorruptOrIrregular  = 2,
    #[cfg(feature = "api_20_5")]
    Corrupt             = 2,
    #[cfg(feature = "api_20_5")]
    Irregular           = 3,
    UnknownEnumValue    = 255,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, PartialOrd, Ord)]
pub enum ItemInfoDeletion {
    Existing                    = 0,   //existing
    PossiblyReverable           = 1,   //previously existing, possibly recoverable
    FirstClusterUnknown         = 2,   //previously existing, first cluster overwritten or unknown
    MovedPossibleRecoverable    = 3,   //renamed/moved, possibly recoverable
    MovedFirstClusterUnknown    = 4,   //renamed/moved, first cluster overwritten or unknown
    CarvedFile                  = 5,    //carved file (since v19.3 SR-3, used to be 1)
    UnknownEnumValue            = 255,
}

pub enum ItemInfoColorAnalysis {
    NotAvailable,
    Error,
    Irrelevant,
    Grayscale,
    Percentage(u8)
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum FileTypeStatus {
    NotVerified             = 0,
    TooSmall                = 1,
    TotallyUnknown          = 2,
    Confirmed               = 3,
    NotConfirmed            = 4,
    NewlyIdentified         = 5,
    MismatchDetected        = 6,
    UnknownEnumValue        = 255,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[repr(u8)]
pub enum FileTypeCategory {
    Picture             = 1,
    Word                = 2,
    Email               = 3,
    Internet            = 4,
    PageLayout          = 5,
    Spreadsheet         = 6,
    Misc                = 7,
    Text                = 8,
    Archive             = 9,
    Audio               = 10,
    Video               = 11,
    WindowsInternal     = 12,
    Thumbnail           = 13,
    Database            = 14,
    Program             = 15,
    MobilePhone         = 16,
    Chat                = 17,
    AddressBook         = 18,
    MacOsXIos           = 19,
    Cad                 = 20,
    VariousData         = 21,
    Gps                 = 22,
    DiskImage           = 23,
    SourceCode          = 24,
    Cryptography        = 25,
    WindowsRegistry     = 26,
    P2P                 = 27,
    Ebook               = 28,
    Graphics3D          = 29,
    Projects            = 30,
    UnixLinux           = 31,
    Font                = 32,
    StillImage          = 33,
    Unknown             = 34,
    Other(String)       = 255,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum XwfDateTime {
    Utc(DateTime<Utc>),            //timestamp is given in UTC
    Local(DateTime<Local>),        //timestamp is given in local time zone
    NoTimezone(NaiveDateTime),     //timestamp has no timezone info
}


#[allow(dead_code)]
pub enum VsPropType {
    SpecialItemId = 10,
    HashType1 =     20,
    HashType2 =     21,
    SetHashType1 =  25,
    SetHashType2 =  26,
    #[cfg(feature = "api_20_9")]
    SetHasChanged =  30,
}

pub enum VolumeNameType {
    SHORT =  3,
    NORMAL = 2,
    LONG =   1
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum StorageLocationType {
    UserSpace                   =0,
    OtherLocation               =1,
    Thumbnail                   =2,
    TemporaryOrCache            =3,
    TrashBin                    =4,
    ApplicationOrSystem         =5,
}

impl StorageLocationType {
    pub fn new(name: &str, path: &str) -> StorageLocationType {
        let p_lower = path.to_lowercase() + "\\" + &*name.to_lowercase();

        if regex_static::static_regex!(r".*\\.?recycle.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\.?trash.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\found.?[0-9]+\\.*").is_match(&p_lower)
            || p_lower.contains("lost+found")
        {
            return StorageLocationType::TrashBin;
        }

        if p_lower.starts_with(r"\windows\")
            || p_lower.starts_with(r"\windows.old\")
            || p_lower.starts_with(r"\program files\")
            || p_lower.starts_with(r"\program files (x86)\")
            || p_lower.starts_with(r"\programdata\")
            || p_lower.starts_with(r"\steam\")
            || p_lower.starts_with(r"\steamapps\")
            || p_lower.starts_with(r"\usr\")
            || p_lower.starts_with(r"\bin\")
            || p_lower.starts_with(r"\var\lib\")
            || p_lower.starts_with(r"\dev\")
            || p_lower.starts_with(r"\proc\")
            || p_lower.contains(r"\system\")
            || p_lower.contains(r"\library\")
            || p_lower.contains(r"\applications\")
            || p_lower.contains(r"\res\")
            || regex_static::static_regex!(r".*resources?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*assets?\\.*").is_match(&p_lower)
        {
            return StorageLocationType::ApplicationOrSystem;
        }

        if regex_static::static_regex!(r".*\\thumb.+\.db.*").is_match(&p_lower)
        || regex_static::static_regex!(r".*thumbnail.*").is_match(&p_lower)
        {
            return StorageLocationType::Thumbnail;
        }

        if regex_static::static_regex!(r".*\\te?mp.*").is_match(&p_lower)
        || regex_static::static_regex!(r".*cache\\.*").is_match(&p_lower)
        || regex_static::static_regex!(r".*\\appdata\\roaming\\.*").is_match(&p_lower)
        || regex_static::static_regex!(r".*\\appdata\\local\\.*").is_match(&p_lower)
        || regex_static::static_regex!(r".*\\appdata\\locallow\\.*").is_match(&p_lower)

        {
            return StorageLocationType::TemporaryOrCache;
        }


        if p_lower.starts_with(r"\users\")
            || p_lower.starts_with(r"\user\")
            || p_lower.starts_with(r"\benutzer\")
            || p_lower.starts_with(r"\home\")
            || p_lower.contains(r"\dcim\")
            || p_lower.contains(r"\camera\")
            || p_lower.contains(r"\dropbox\")
            || p_lower.contains(r"\onedrive\")
            || p_lower.contains(r"\desktop\")
            || p_lower.contains(r"\schreibtisch\")
            || regex_static::static_regex!(r".*\\mega.?(nz)?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\google\s?drive\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\pictures?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\musi[ck]?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\audio\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\(f|ph)otos?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\videos?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\movie?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\record(ings|ing|s)?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\downloads?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\screen\s?(shot|record|capture).*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\screen\s?recordings?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\screen\s?captures?\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\do[ck]ument[es]\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\proje[ck]t[es]\\.*").is_match(&p_lower)
            || regex_static::static_regex!(r".*\\[0-9]{3,4}apple\\.*").is_match(&p_lower)
            || p_lower.contains(r"aufnahme")
            || p_lower.contains(r"bildschirm")
            || p_lower.contains(r"bilder")
            || p_lower.contains(r"filme")
            || p_lower.contains(r"serien")
            || p_lower.contains(r"spiele")
            || p_lower.contains(r"arbeit")
            || p_lower.contains(r"familie")
            || p_lower.contains(r"privat")
            || p_lower.contains(r"sammlung")
            || p_lower.contains(r"urlaub")
            || p_lower.contains(r"reisen")
            || p_lower.contains(r"feier")
            || p_lower.contains(r"album")
            || p_lower.contains(r"porn")
            || p_lower.contains(r"whatsapp")
            || p_lower.contains(r"telegram")
        {
            return StorageLocationType::UserSpace;
        }


        StorageLocationType::OtherLocation
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paths() {
        assert_eq!(StorageLocationType::new("2020_12_24_10_32_51.thumbnail", r"\filesystem2\mobile\Containers\Data\Application\F7F7510A-5D3E-4A6D-9062-DFC594D17034\Documents\videoCache"),
            StorageLocationType::Thumbnail);


        assert_eq!(StorageLocationType::new("63969157981__DD4550B6-55C6-459A-9B97-86A7B9EB24C9.largeThumbnail", r"\filesystem2\mobile\Containers\Data\Application\B140F93B-1639-4A09-9FCB-2C1E9C34D6CE\tmp"),
                   StorageLocationType::Thumbnail);

        assert_eq!(StorageLocationType::new("0C8E427FC5CB7186DB4F869D1583D82D", r"\filesystem2\mobile\Containers\Data\Application\F7F7510A-5D3E-4A6D-9062-DFC594D17034\Documents\.mediaLibrary.Cache\Thumbnail"),
                   StorageLocationType::Thumbnail);

        assert_eq!(StorageLocationType::new("64.png",r"\Users\borch\AppData\Local\Google\Chrome\User Data\Default\Web Applications\Manifest Resources\kefjledonklijopmnomlcbpllchaibag\Icons"),
                   StorageLocationType::ApplicationOrSystem);


        assert_eq!(StorageLocationType::new("f_000098",r"\Users\borch\AppData\Local\Google\Chrome\User Data\Profile 2\Cache\Cache_Data"),
                   StorageLocationType::TemporaryOrCache);

        assert_eq!(StorageLocationType::new("tray-connected.png",r"\Users\borch\AppData\Roaming\discord"),
                   StorageLocationType::TemporaryOrCache);


        assert_eq!(StorageLocationType::new("AppList.targetsize-64.png",r"\Users\borch\Downloads\PLAY_STORE_W11_TRD.rar\PLAY_STORE_W11_TRD\Images"),
                   StorageLocationType::UserSpace);

        assert_eq!(StorageLocationType::new("close_list.png",r"\Program Files (x86)\CyberLink\PowerDVD12\Custom\Skin\Standard\Photo\Media.zip\media\albumlist"),
                   StorageLocationType::ApplicationOrSystem);


        assert_eq!(StorageLocationType::new("IMG_0247.JPG",r"\filesystem2\mobile\Media\DCIM\100APPLE"),
                   StorageLocationType::UserSpace);

        assert_eq!(StorageLocationType::new("xxx.png",r"\Users\Tim\Dokumente"),
                   StorageLocationType::UserSpace);

        assert_eq!(StorageLocationType::new("xxx.mov",r"\backup\data\spielfilme"),
                   StorageLocationType::UserSpace);

        assert_eq!(StorageLocationType::new("xxx.png","\\mega"),
                   StorageLocationType::UserSpace);

    }
}
