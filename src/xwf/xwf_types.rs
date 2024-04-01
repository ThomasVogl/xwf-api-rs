use std::mem::size_of;

use bitflags::bitflags;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::Serialize;
use winapi::{ctypes::__int64, shared::minwindef::{DWORD, LPVOID}};

use super::api::error::XwfError;

bitflags! {

    #[derive(Serialize, Debug, PartialEq, Eq)]
    pub struct ItemInfoFlags: u64 {
        const IsDirectory                           = 0x00000001;
        const HasChildObjects                       = 0x00000002;
        const HasSubDirectories                     = 0x00000004;
        const IsVirtualItem                         = 0x00000008;
        const HiddenByExaminer                      = 0x00000010;
        const Tagged                                = 0x00000020;
        const TaggedPartially                       = 0x00000040;
        const ViewedByExaminer                      = 0x00000080;

        const FilesystemTimestampsNotInUTC          = 0x00000100;
        const InternalCreationTimestampsNotInUTC    = 0x00000200;
        const FATTimestamps                         = 0x00000400;
        const OriginatesFromNTFS                    = 0x00000800;
        const UnixPermissionsInsteadWinAttr         = 0x00001000;
        const HasExaminerComment                    = 0x00002000;
        const HasExtractedMetaData                  = 0x00004000;
        const FileContentsTotallyUnknown            = 0x00008000;

        const FileContentsPartiallyUnknown          = 0x00010000;
        const Reserved                              = 0x00020000;
        const Hash1AlreadyComputed                  = 0x00040000;
        const HasDuplicates                         = 0x00080000;
        const Hash2AlreadyComputed                  = 0x00100000;
        const CategorizedIrrelevant                 = 0x00200000;
        const CategorizedNotable                    = 0x00400000;
        const Uncategorized                         = 0x00600000;
        const FoundInVolumeShadowCopy               = 0x00800000;

        const DeletedFilesWithKnownOriginalContents = 0x01000000;
        const FileFormatConstistencyOk              = 0x02000000;
        const FileFormatConstistencyNotOk           = 0x04000000;
        const FileArchiveExplored                   = 0x10000000;
        const EmailArchiveProcessed                 = 0x20000000;
        const EmbeddedDataUncovered                 = 0x40000000;
        const MetaDataExtractionApplied             = 0x80000000;

        const FileEmbeddedinOtherFile               = 0x100000000;
        const FileContentsStoredExternally          = 0x200000000;
        const AlternativeData                       = 0x400000000;
        // The source may set any bits
        const _ = !0;
    }

    

    pub struct ItemInfoAttributes: i64 {
        const WinAttrReadOnly           = 0x00000001; //Windows attribute read only
        const WinAttrHidden             = 0x00000002; //Windows attribute hidden
        const WinAttrSystem             = 0x00000004; //Windows attribute system
        const WinAttrArchive            = 0x00000020; //Windows attribute to be archived
        const WinProcess                = 0x00000040; //is a process (in Windows memory dumps)
        const WinRunning                = 0x00000080; //is on running list (in Windows memory dumps)
        const WinAttrTemp               = 0x00000100; //Windows attribute temporary
        const WinAttrSparse             = 0x00000200; //Windows attribute sparse
        const WinAttrReparse            = 0x00000400; //Windows attribute reparse point
        const WinAttrCompressed         = 0x00000800; //Windows attribute compressed
        const WinAttrOffline            = 0x00001000; //Windows attribute offline
        const WinAttrIndexed            = 0x00002000; //Windows attribute not indexed
        const WinAttrEncrypted          = 0x00004000; //Windows attribute encrypted
        const AdditionalHardLink        = 0x00010000; //additional hard link
        const WinAttrNoScrubData        = 0x00020000; //Windows attribute no scrub data
        const Compressed                = 0x00020000; //compressed in file archive
        const EncryptionSuspected       = 0x00040000; //encryption suspected
        const EncryptedArchive          = 0x00080000; //encrypted in file archive
        const StartSectorApproximated   = 0x00100000; //start sector only approximately correct
        const HardlinkManagmentFlag     = 0x00200000; //hardlink management flag
        const EnryptedInFilesystem      = 0x00400000; //encrypted in file system
        const FileFormatEncryption      = 0x00800000; //file format specific encryption
        const HasObjectId               = 0x01000000; //has object ID
        const CompressedInFilesystem    = 0x02000000; //compressed in file system
        const PartiallyInitialized      = 0x04000000; //partially initialized
        const SpecialStorage            = 0x08000000; //special storage; multi-purpose flag
        const EmailWithAttachment       = 0x10000000; //e-mail message with attachment
        const MultiPurpose              = 0x20000000; //multi-purpose flag
        const HasRelatedItem            = 0x40000000; //has a related item
        const PhotoDNAHashValue         = 0x80000000; //PhotoDNA hash value stored in volume snapshot
        // The source may set any bits
        const _ = !0;
    }

    pub struct OpenItemFlags: u32 {
        const OpenForAccessIncludingFileSlack   = 0x0001; //open for access including file slack
        const SuppressErrorMessages             = 0x0002; //suppress error messages in the program in case of failure
        const PreferAlternativeFileData         = 0x0008; //prefer alternative file data if available, e.g. a thumbnail created by X-Ways Forensics for a picture
        const OpenAlternativeFileDataWithFail   = 0x0010; //open carved files in Ext2/3 volumes without applying Ext block logic (at least v19.8 and later)
        const OpenCarvedFilesInExt              = 0x0080; //open carved files in Ext2/3 volumes without applying Ext block logic (at least v19.8 and later)
        const ConvertToPDF                      = 0x0200; //convert to PDF format on the fly and open PDF data (v19.9 and later, useful for certain supported file formats)*
        const ExtractPlainTextUtf8              = 0x0400; //extract plain text on the fly as UTF-8 and open textual data (v20.0 and later, useful for certain supported file formats)*
        const ExtractPlainTextUtf16             = 0x0800; //extract plain text on the fly as UTF-16 and open textual data (v20.0 and later, useful for certain supported file formats)*
        const PrependByteOrderMark              = 0x1000; //prepend byte-order mark (for 0x0400 and 0x0800)
        // The source may set any bits
        const _ = !0;
    }
    #[derive(Clone, Serialize, PartialEq, Eq, Debug)]
    pub struct ReportTableFlags: u32 {
        const HintByApplication             = 0x0001; //hint for user by application
        const CreatedByUser                 = 0x0002; //created manually by the user
        const RepresentsHashset             = 0x0004; //represents a hash set
        const SelectedForInclusion          = 0x0010; //selected for inclusion in the case report
        const SelectedForFilter             = 0x0020; //selected for the label/report table filter
        const SelectedForNewAssociations    = 0x0040; //selected for new assocations
        const RepresentsSearchTerm          = 0x0080; //represents a search term
        const NotDocumented1                = 0x0100; //flag not documented in XWF API but was observed
        const NotDocumented2                = 0x1000; //flag not documented in XWF API but was observed
        const DetectedObjectInPhoto         = 0x2000; //detected object in photo
        const RepresentsDuplicateFiles      = 0x4000; //represents a group of duplicate files
        const OfferedForSelectionInReport   = 0x8000; //offered for selection in the report
        // The source may set any bits
        const _ = !0;
    }

    pub struct OutputMessageFlags: u32 {
        const AppendWithoutLineBreak    = 0x00000001; //append without line break (will be delimited from the previous message with a space instead)
        const DontLogToMsgLog           = 0x00000002; //don't log this error message in msglog.txt even if logging is active by default
        const IsAnsiString              = 0x00000004; //lpMessage points to an ANSI string, not a Unicode string (v16.5 and later)
        const LogToOutputWindow         = 0x00000008; //output the message in the Output window instead of the Messages window (v20.6 and later), where no [XT] prefix is inserted
        const OutputAsCaseLogEntry      = 0x00000010; //output the message as an entry in the case log,
                                                      // not in the Messages window (v19.4 and later), flag is ignored if no case is active, may be combined with the 0x4 flag
        // The source may set any bits
        const _ = !0;
    }

    pub struct AddReportTableFlags: u32 {
        const CreatedByApplication          = 0x01; // show as created by application, not by examiner
        const SelectForInclusionInReport    = 0x02; // select for inclusion in report
        const SelectForFiltering            = 0x04; // select for filtering
        const SelectForManualAssocs         = 0x08; // select for future manual associations
        // The source may set any bits
        const _ = !0;
    }

    pub struct ProgressFlags: u32 {
        const NoProgressBar = 0x00000001; //show just the window, no actual progress bar
        const NoUserInterruption = 0x00000002; //do not allow the user to interrupt the operation
        const ShowWindowImmediately = 0x00000004; //show window immediately
        const DoubleConfirmAbort = 0x00000008; //double-confirm abort
        const PreventLogging = 0x00000010; //prevent logging
        // The source may set any bits
        const _ = !0;
    }

    pub struct EvObjPropFlags: u32 {
        const DataWindowActive = 0x01; //Data window active yes/no
        const DataWindowOpen = 0x02; // Data window open yes/no
        const Flagged = 0x04; // Flagged yes/no
        const SelectedForOperations = 0x08; // Selected for operations yes/no (in case of a physical, partitioned evidence object, the operation should be applied to the areas outside of explorable partitions only, as the partitions are their own evidence objects and selectable separately)
        const SelectedForRecursiveView = 0x10; // Selected for recursive view yes/no, in v19.9 SR-11, v20.0 SR-6 and later
        const ExpandedInCaseTree = 0x20; // Expanded in case tree yes/no
        const HasNoChildren = 0x40; // Has no children yes/no

        const IsFileContainer = 0x0100; // Is an evidence file container yes/no
        const IsDeletedPartition = 0x0200; // Is a deleted partition yes/no
        const IsOpticalDisk = 0x0400; // Optical disc icon yes/no
        const IsRAM = 0x0800; //RAM icon yes/no
        const IsDynamicDisk = 0x1000; //Is dynamic disk yes/no
        const IsSingleFile = 0x2000; //Evidence object is just a single file in the directory

        const IndexAvailable = 0x010000; //Index available yes/no
        const LoggingEnabled = 0x020000; //Logging enabled yes/no
        const AnnotationsHighlighted = 0x040000; //Annotations highlighted yes/no
        const WarnedOfWeirdImageSize = 0x080000; //Warned of weird image file size already yes/no
        const SuppressSizeChanged = 0x100000; //Suppress "size of evidence object has changed" yes/no
        const _ = !0;
    }

    pub struct XtInitFlags: u32 {
        const IsForensics =  0x00000001; // X-Ways Forensics (flag used reliably in releases from 2015)
        const IsWinHex = 0x00000002; // WinHex  (flag used reliably in releases from 2015)
        const IsInvestigator =  0x00000004; // X-Ways Investigator
        const IsBeta =  0x00000008; // pre-release version
        const IsQuickcheck = 0x00000020; // called just to check whether the X-Tension accepts the calling application (used by v16.5 and later)
        const IsAboutOnly =  0x00000040; // called just to prepare for XT_About() or XT_PrepareSearch() (used by v16.5 and later)
        const _ = !0;
    }

    pub struct XtPreparePositiveReturnFlags: i32 {
        const CallProcessItem = 0x01;   //if you want X-Ways Forensics to call your implementation of XT_ProcessItem[Ex]() (whichever is exported) for each item
                                        //this volume snapshot (not if the volume snapshot is not targeted, e.g. in case of XT_ACTION_RUN)

        const CallProcessItemLate = 0x02;   //for XT_ACTION_RVS specify this flag in addition to XT_PREPARE_CALLPI if you wish to receive calls of XT_ProcessItem() (not Ex),
                                            // if actually exported, after all other individual item refinement operations instead of before
                                            // (preferable for example so that you do not get called for ignorable files that were recognized
                                            // as such by hash database matching during the same volume snapshot refinement run)

        const ExpectMoreItemsToBeCreated = 0x04;       //in case of XT_ACTION_RVS, to signal XWF that you may create more items in the volume snapshot,
                                            //so that for example the user will definitely be informed of how many item were added (v16.5 and later only)

        const DoNotOmit = 0x08;             //in case of XT_ACTION_RVS, to signal XWF that you wish to receive calls for XT_ProcessItem[Ex]()
                                            //even for files that the user wants to omit for any of the possible three reasons (v18.5 and later only)

        const ProcessDirs = 0x10;           //in case of XT_ACTION_RVS, to signal XWF that you wish to receive calls for XT_ProcessItem[Ex]() even for directories, not only files,
                                            //for example because you wish to parse the file system data structures in those directories (v18.5 and later only)

        const ProcessZerorBytesFiles = 0x20;//in case of XT_ACTION_RVS, to signal XWF that you wish to receive calls for XT_ProcessItem[Ex]()
                                            //even for files that have a size of 0 bytes, which are otherwise skipped for performance reasons (v18.9 SR-7 and later only)
        const _ = !0;
    }

    pub struct ItemTypeFlags: u32 {
        const TextualDescriptionType = 0x20000000; //receive a textual description of the file type instead (e.g. “JPEG” or “Dynamic-Link Library”)
        const TextualDescriptionCategory = 0x40000000; //receive a textual designation of the category that the file type belongs to instead (e.g. “Pictures” or “Programs”)
        const ReceiveTypeStatus = 0x80000000; //receive type status as usual in the lowest byte, but file format consistency in the second-lowest byte (0=unknown, 1=OK, 2=irregular), v19.3 and later
        const _ = !0;
    }

    pub struct FileCreationFlags: u32 {
        const MoreItemsToBeCreated      = 0x00000001;
        const ExcerptFromParent         = 0x00000002;
        const AttachExternalFile        = 0x00000004;
        const KeepExternalFile          = 0x00000008;
        const FileContentsFromBuffer    = 0x00000010;
        const _ = !0;
    }


}

impl Default for ItemInfoFlags {
    fn default() -> Self {
        ItemInfoFlags::empty()
    }
}

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
pub enum XtPrepareReturn {
    Negative(XtPrepareNegativeReturn),
    Positive(XtPreparePositiveReturnFlags)
}

impl Into<i32> for XtPrepareReturn {
    fn into(self) -> i32 {
        match self {
            XtPrepareReturn::Negative(x) => {x.into() },
            XtPrepareReturn::Positive(x) => { x.bits() }
        }
    }
}

pub enum XtProcessItemReturn {
    StopCurrentOperation        = -1, //Return -1 if you want X-Ways Forensics to stop the current operation (e.g. volume snapshot refinement),
    SkipAllOtherVSRefinements   = -2, //-2 if you want have X-Ways Forensics skip all other volume snapshot refinement operations for this file
    Ok                          =  0, //otherwise 0.
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

pub enum XtProcessItemExReturn {
    StopCurrentOperation        = -1, //Return -1 if you want X-Ways Forensics to stop the current operation (e.g. volume snapshot refinement),
    Ok                          =  0, //otherwise 0.
}

impl Into<i32> for XtProcessItemExReturn {
    fn into(self) -> i32 {
        match self {
            XtProcessItemExReturn::StopCurrentOperation => { XtProcessItemExReturn::StopCurrentOperation as i32}
            XtProcessItemExReturn::Ok => { XtProcessItemExReturn::Ok as i32 }
        }
    }
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

#[derive(Debug, PartialEq, Eq, Serialize)]
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
            _ => Err(XwfError::InvalidEnumValue(("ItemInfoClassification", value)))
        }
    }
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
    InternalCreationTime    = 37

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

impl Into<i32> for XtFinalizeReturn {
    fn into(self) -> i32 {
        match self {
            XtFinalizeReturn::RefreshDirectoryListing => {XtFinalizeReturn::RefreshDirectoryListing as i32}
            XtFinalizeReturn::Ok => {XtFinalizeReturn::Ok as i32}
        }
    }
}

pub struct XtVersion {
    pub major: u16,
    pub minor: u16,
    pub service_release: u8,
    pub language: u8,
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


impl std::convert::TryFrom<u32> for XtPrepareOpType {
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
#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum FileFormatConsistency {
    Unknown = 0,
    Ok = 1,
    Irregular = 2,
    NotDocumented = 3,
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
#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum ItemInfoDeletion {
    Existing                    = 0,   //existing
    PossiblyReverable           = 1,   //previously existing, possibly recoverable
    FirstClusterUnknown         = 2,   //previously existing, first cluster overwritten or unknown
    MovedPossibleRecoverable    = 3,   //renamed/moved, possibly recoverable
    MovedFirstClusterUnknown    = 4,   //renamed/moved, first cluster overwritten or unknown
    CarvedFile                  = 5    //carved file (since v19.3 SR-3, used to be 1)
}

impl ItemInfoDeletion {
    pub fn is_existing(&self) -> bool {
        match &self {
            ItemInfoDeletion::Existing => true,
            _ => false
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


#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum FileTypeStatus {
    NotVerified = 0,
    TooSmall = 1,
    TotallyUnknown = 2,
    Confirmed=3,
    NotConfirmed=4,
    NewlyIdentified=5,
    MismatchDetected=6,
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

#[derive(Debug, PartialEq, Eq, Serialize)]
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

#[derive(Serialize, Debug)]
pub enum XwfDateTime {
    Utc(DateTime<Utc>),            //timestamp is given in UTC
    Local(DateTime<Local>),        //timestamp is given in local time zone
    NoTimezone(NaiveDateTime),     //timestamp has no timezone info
}

impl XwfDateTime {
    pub fn to_naive(&self) -> NaiveDateTime {
        match &self {
            XwfDateTime::Utc(v) => v.naive_local(),
            XwfDateTime::Local(v) => v.naive_local(),
            XwfDateTime::NoTimezone(v) => *v,
        }
    }
}

impl Ord for XwfDateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_naive().cmp(&other.to_naive())
    }
}

impl PartialOrd for XwfDateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for XwfDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.to_naive().eq(&other.to_naive())
    }
}

impl Eq for XwfDateTime {}


#[repr(packed(2))]
pub struct SrcInfo {
   pub n_struct_size: DWORD,
   pub n_buf_size: __int64 ,
   pub p_buffer: LPVOID
}

impl SrcInfo {
    pub fn from_buffer(data: &mut [u8]) -> SrcInfo {
        SrcInfo {
            n_struct_size: size_of::<SrcInfo>() as u32,
            n_buf_size: data.len() as __int64,
            p_buffer: data.as_mut_ptr() as LPVOID
        }
    }
}



