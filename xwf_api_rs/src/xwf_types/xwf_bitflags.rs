use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Serialize, Debug, PartialEq, Eq, Clone, Copy)]
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
    #[derive(Clone, Serialize, PartialEq, Eq, Debug, Deserialize)]
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
        #[cfg(feature = "api_20_6")]
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

        const TargetDirs = 0x10;           //in case of XT_ACTION_RVS, to signal XWF that you wish to receive calls for XT_ProcessItem[Ex]() even for directories, not only files,
                                            //for example because you wish to parse the file system data structures in those directories (v18.5 and later only)

        const TargetZerorBytesFiles = 0x20;  //in case of XT_ACTION_RVS, to signal XWF that you wish to receive calls for XT_ProcessItem[Ex]()
                                             //even for files that have a size of 0 bytes, which are otherwise skipped for performance reasons (v18.9 SR-7 and later only)
        #[cfg(feature = "api_21_2")]
        const TargetFilesWithUnknownData = 0x40; //in case of XT_ACTION_RVS, to signal XWF that you wish to receive calls for XT_ProcessItem[Ex]() even for files that of which only metadata are known, which are otherwise skipped for performance reasons (v21.2 SR-5 and later only)
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