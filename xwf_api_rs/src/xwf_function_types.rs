use winapi::shared::minwindef::{BOOL, BYTE, DWORD, LPBOOL, LPLONG, LPVOID, PDWORD};
use winapi::shared::ntdef::{LONG, HANDLE, WCHAR, LPWCH, PVOID, PLONG, LPWSTR};
use winapi::ctypes::{__int64};

type LPINT64 = *mut i64;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfOutputMessage = extern "system" fn(lpMessage: *const WCHAR, nFlags: DWORD);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetVolumeName = extern "system" fn(
    hVolume: HANDLE,
    lpString: LPWSTR,
    nType: DWORD);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetVolumeInformation = extern "system" fn(
    hVolume: HANDLE,
    lpFileSystem: LPLONG,
    nBytesPerSector: *mut DWORD,
    nSectorsPerCluster: *mut DWORD,
    nClusterCount: *mut __int64,
    nFirstClusterSectorNo: *mut __int64
) -> BOOL;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfRead = extern "system" fn(
    hVolumeOrItem: HANDLE,
    nOffset: __int64,
    lpBuffer: *mut BYTE,
    nNumberOfBytesToRead: DWORD,
) -> DWORD;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemSize = extern "system" fn(
    nItemID: LONG,
) -> __int64;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemInformation = extern "system" fn(
    nItemID: LONG,
    nInfoType: LONG,
    lpSuccess: LPBOOL
) -> __int64;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetItemInformation = extern "system" fn(
    nItemID: LONG,
    nInfoType: LONG,
    nInfoValue: __int64
) -> BOOL;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemType = extern "system" fn(
    nItemId: LONG,
    lpTypeDescr: LPWSTR,
    nBufferLenAndFlags: DWORD
) -> LONG;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetItemType = extern "system" fn(
    lpTypeDescr: LPWCH,
    nTypeStatus: LONG
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemParent = extern "system" fn(
    nItemID: LONG
) -> LONG;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetNextEvObj = extern "system" fn(
    hPrevEvidence: HANDLE,
    pReserved: LPVOID) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetFirstEvObj = extern "system" fn(
    pReserved: LPVOID) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetCaseProp = extern "system" fn(
    pReserved: LPVOID ,
    nPropType: LONG,
    pBuffer: LPVOID,
    nBufLen: LONG
) -> __int64;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetEvObjProp = extern "system" fn(
    hEvidence: HANDLE,
    nPropType: DWORD,
    pBuffer: PVOID,
) -> __int64;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetEvObj = extern "system" fn(
    nEvObjID: DWORD
) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetReportTableInfo = extern "system" fn(
    pReserved: LPVOID,
    nReportTableID: LONG,
    lpOptional: PLONG
) -> LPVOID;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetReportTableAssocs = extern "system" fn(
    nItemID: LONG,
    lpBuffer: LPWSTR,
    nBufferLen: LONG
) -> DWORD;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetHashSetAssocs = extern "system" fn(
    nItemID: LONG,
    lpBuffer: LPWSTR,
    nBufferLen: LONG,
) -> LONG;



#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetEvObjReportTableAssocs = extern "system" fn(
    hEvidence: HANDLE,
    nFlags: LONG,
    lpValue: PLONG
) -> LPVOID;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfOpenEvObj = extern "system" fn(
    hEvidence: HANDLE,
    nFlags: DWORD
) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfAddToReportTable = extern "system" fn(
    nItemID: LONG,
    lpReportTableName: LPWSTR,
    nFlags: DWORD
) -> LONG;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetHashValue = extern "system" fn(
    nItemID: LONG,
    lpBuffer: LPVOID
) -> BOOL;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetHashValue = extern "system" fn(
    nItemID: LONG,
    lpHash: LPVOID,
    nParam: DWORD
) -> BOOL;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfShowProgress = extern "system" fn(
    lpCaption: LPWSTR,
    nFlags: DWORD,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfOpenItem = extern "system" fn(
    hVolume: HANDLE,
    nItemID: LONG,
    nFlags: DWORD,
) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetVsprop = extern "system" fn(
    nPropType: LONG,
    pBuffer: PVOID,
) -> __int64;


#[cfg(feature="api_20_9")]
#[allow(non_snake_case, unused_variables)]
pub type FnXwfSelectVolumeSnapshot = extern "system" fn(
    hVolume: HANDLE,
) -> LONG;

#[cfg(not(feature="api_20_9"))]
#[allow(non_snake_case, unused_variables)]
pub type FnXwfSelectVolumeSnapshot = extern "system" fn(
    hVolume: HANDLE,
);


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemName = extern "system" fn(
    nItemID: DWORD
) -> LPWSTR;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemCount = extern "system" fn(
    pTarget: LPVOID
) -> DWORD;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetProp = extern "system" fn(
    hVolumeOrItem: HANDLE,
    nPropType: DWORD ,
    lpBuffer: PVOID,
) -> __int64;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfCloseEvObj = extern "system" fn(
    hEvidence: HANDLE,
);
#[allow(non_snake_case, unused_variables)]
pub type FnXwfClose = extern "system" fn(
    hVolumeOrItem: HANDLE,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetProgressDescription = extern "system" fn(
    lpStr: LPWSTR,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetProgressPercentage = extern "system" fn(
    nPercent: DWORD,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfHideProgress = extern "system" fn(
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfShouldStop = extern "system" fn(
) -> BOOL;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetUserInput = extern "system" fn(
    lpMessage: LPWSTR,
    lpBuffer: LPWSTR,
    nBufferLen: DWORD,
    nFlags: DWORD
) -> __int64;



#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetExtractedMetadata = extern "system" fn(
    nItemID: LONG,
) -> LPWSTR;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfAddExtractedMetadata = extern "system" fn(
    nItemID: LONG,
    lpComment: LPWSTR,
    nFlagsHowToAdd: DWORD,
) -> BOOL;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetMetadataEx = extern "system" fn(
    hItem: HANDLE,
    lpnFlags: PDWORD
) -> LPVOID;



#[allow(non_snake_case, unused_variables)]
pub type FnXwfReleaseMem = extern "system" fn(
    lpBuffer: PVOID,
) -> BOOL;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemOfs = extern "system" fn(
    nItemID: LONG ,
    nDefOfs: LPINT64,
    nStartSector: LPINT64,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetComment = extern "system" fn(
    nItemID: LONG ,
) -> LPWSTR;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfAddComment = extern "system" fn(
    nItemID: LONG ,
    lpComment: LPWSTR,
    nFlagsHowToAdd: DWORD,
) -> BOOL;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfCreateFile = extern "system" fn(
    pName: LPWSTR,
    nCreationFlags: DWORD,
    nParentItemID: LONG,
    pSourceInfo: PVOID
) -> LONG;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetItemSize = extern "system" fn(
    nItemID: LONG,
    nSize: __int64,
);


#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetItemParent = extern "system" fn(
    nChildItemID: LONG,
    nParentItemID: LONG
);




