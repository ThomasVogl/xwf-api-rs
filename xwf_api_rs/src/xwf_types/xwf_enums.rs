use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::Serialize;
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


#[derive(Debug, PartialEq, Eq, Serialize, Clone, Copy)]
pub enum ItemInfoClassification {
    NormalFile                            = 0x00, //normal file
    HfsResourceFork                       = 0x04, //HFS resource fork
    NtfsAlternateDataStream               = 0x08, //NTFS alternate data stream
    NtfsNonDirectoryIndex                 = 0x0A, //NTFS non-directory index
    NtfsBitmapAttribute                   = 0x0B, //NTFS bitmap attribute
    NotDocumented1                        = 0x0E, //not documented but occuring
    NtfsGeneralLoggedUtilityStream        = 0x10, //NTFS general logged utility stream
    NtfsEfsLoggedUtilityStream            = 0x11, //NTFS EFS logged utility stream
    NotDocumented2                        = 0xF2, //not documented but occuring
    NotDocumented3                        = 0xF3, //not documented but occuring
    NotDocumented4                        = 0xF4, //not documented but occuring
    EmailRelated                          = 0xF5, //e-mail related
    Excerpt                               = 0xF6, //excerpt
    ManuallyAttached                      = 0xF7, //manually attached
    VideoStill                            = 0xF8, //video still
    EmailAttachment                       = 0xF9, //e-mail attachment
    EmailMessage                          = 0xFA, //e-mail message
    IdnxRecordRemnant                     = 0xFD, //INDX record remnant
    Unknown = 0xFF
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
pub enum XtPrepareOpType {
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

#[derive(Debug)]
pub struct XtVersion {
    pub major: u16,
    pub minor: u16,
    pub service_release: u8,
    pub language: u8,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Copy)]
pub enum FileFormatConsistency {
    Unknown = 0,
    Ok = 1,
    Irregular = 2,
    NotDocumented = 3,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Copy)]
pub enum ItemInfoDeletion {
    Existing                    = 0,   //existing
    PossiblyReverable           = 1,   //previously existing, possibly recoverable
    FirstClusterUnknown         = 2,   //previously existing, first cluster overwritten or unknown
    MovedPossibleRecoverable    = 3,   //renamed/moved, possibly recoverable
    MovedFirstClusterUnknown    = 4,   //renamed/moved, first cluster overwritten or unknown
    CarvedFile                  = 5    //carved file (since v19.3 SR-3, used to be 1)
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Copy)]
pub enum FileTypeStatus {
    NotVerified = 0,
    TooSmall = 1,
    TotallyUnknown = 2,
    Confirmed=3,
    NotConfirmed=4,
    NewlyIdentified=5,
    MismatchDetected=6,
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone, Copy)]
pub enum FileTypeCategory {
    Picture,
    Word,
    Email,
    Internet,
    PageLayout,
    Spreadsheet,
    Misc,
    Text,
    Archive,
    Audio,
    Video,
    WindowsInternal,
    Thumbnail,
    Database,
    Program,
    MobilePhone,
    Chat,
    AddressBook,
    MacOsXIos,
    Cad,
    VariousData,
    Gps,
    DiskImage,
    SourceCode,
    Cryptography,
    WindowsRegistry,
    P2P,
    Ebook,
    Graphics3D,
    Projects,
    UnixLinux,
    Font,
    StillImage,
    Unknown,
    Other,
}

#[derive(Serialize, Debug, Clone)]
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