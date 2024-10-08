use winapi::shared::minwindef::{BOOL, BYTE, DWORD, LPBOOL, LPLONG, LPVOID, PDWORD};
use winapi::shared::ntdef::{LONG, HANDLE, WCHAR, LPWCH, PVOID, PLONG, LPWSTR};
use winapi::ctypes::{__int64};

type LPINT64 = *mut i64;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfOutputMessage = extern "stdcall" fn(lpMessage: *const WCHAR, nFlags: DWORD);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetVolumeName = extern "stdcall" fn(
    hVolume: HANDLE,
    lpString: LPWSTR,
    nType: DWORD);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetVolumeInformation = extern "stdcall" fn(
    hVolume: HANDLE,
    lpFileSystem: LPLONG,
    nBytesPerSector: *mut DWORD,
    nSectorsPerCluster: *mut DWORD,
    nClusterCount: *mut __int64,
    nFirstClusterSectorNo: *mut __int64
) -> BOOL;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfRead = extern "stdcall" fn(
    hVolumeOrItem: HANDLE,
    nOffset: __int64,
    lpBuffer: *mut BYTE,
    nNumberOfBytesToRead: DWORD,
) -> DWORD;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemSize = extern "stdcall" fn(
    nItemID: LONG,
) -> __int64;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemInformation = extern "stdcall" fn(
    nItemID: LONG,
    nInfoType: LONG,
    lpSuccess: LPBOOL
) -> __int64;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetItemInformation = extern "stdcall" fn(
    nItemID: LONG,
    nInfoType: LONG,
    nInfoValue: __int64
) -> BOOL;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemType = extern "stdcall" fn(
    nItemId: LONG,
    lpTypeDescr: LPWSTR,
    nBufferLenAndFlags: DWORD
) -> LONG;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetItemType = extern "stdcall" fn(
    lpTypeDescr: LPWCH,
    nTypeStatus: LONG
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemParent = extern "stdcall" fn(
    nItemID: LONG
) -> LONG;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetNextEvObj = extern "stdcall" fn(
    hPrevEvidence: HANDLE,
    pReserved: LPVOID) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetFirstEvObj = extern "stdcall" fn(
    pReserved: LPVOID) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetCaseProp = extern "stdcall" fn(
    pReserved: LPVOID ,
    nPropType: LONG,
    pBuffer: LPVOID,
    nBufLen: LONG
) -> __int64;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetEvObjProp = extern "stdcall" fn(
    hEvidence: HANDLE,
    nPropType: DWORD,
    pBuffer: PVOID,
) -> __int64;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetEvObj = extern "stdcall" fn(
    nEvObjID: DWORD
) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetReportTableInfo = extern "stdcall" fn(
    pReserved: LPVOID,
    nReportTableID: LONG,
    lpOptional: PLONG
) -> LPVOID;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetReportTableAssocs = extern "stdcall" fn(
    nItemID: LONG,
    lpBuffer: LPWSTR,
    nBufferLen: LONG
) -> DWORD;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetHashSetAssocs = extern "stdcall" fn(
    nItemID: LONG,
    lpBuffer: LPWSTR,
    nBufferLen: LONG,
) -> LONG;



#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetEvObjReportTableAssocs = extern "stdcall" fn(
    hEvidence: HANDLE,
    nFlags: LONG,
    lpValue: PLONG
) -> LPVOID;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfOpenEvObj = extern "stdcall" fn(
    hEvidence: HANDLE,
    nFlags: DWORD
) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfAddToReportTable = extern "stdcall" fn(
    nItemID: LONG,
    lpReportTableName: LPWSTR,
    nFlags: DWORD
) -> LONG;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetHashValue = extern "stdcall" fn(
    nItemID: LONG,
    lpBuffer: LPVOID
) -> BOOL;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetHashValue = extern "stdcall" fn(
    nItemID: LONG,
    lpHash: LPVOID,
    nParam: DWORD
) -> BOOL;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfShowProgress = extern "stdcall" fn(
    lpCaption: LPWSTR,
    nFlags: DWORD,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfOpenItem = extern "stdcall" fn(
    hVolume: HANDLE,
    nItemID: LONG,
    nFlags: DWORD,
) -> HANDLE;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetVsprop = extern "stdcall" fn(
    nPropType: LONG,
    pBuffer: PVOID,
) -> __int64;


#[cfg(feature="api_20_9")]
#[allow(non_snake_case, unused_variables)]
pub type FnXwfSelectVolumeSnapshot = extern "stdcall" fn(
    hVolume: HANDLE,
) -> LONG;

#[cfg(not(feature="api_20_9"))]
#[allow(non_snake_case, unused_variables)]
pub type FnXwfSelectVolumeSnapshot = extern "stdcall" fn(
    hVolume: HANDLE,
);


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemName = extern "stdcall" fn(
    nItemID: DWORD
) -> LPWSTR;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemCount = extern "stdcall" fn(
    pTarget: LPVOID
) -> DWORD;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetProp = extern "stdcall" fn(
    hVolumeOrItem: HANDLE,
    nPropType: DWORD ,
    lpBuffer: PVOID,
) -> __int64;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfCloseEvObj = extern "stdcall" fn(
    hEvidence: HANDLE,
);
#[allow(non_snake_case, unused_variables)]
pub type FnXwfClose = extern "stdcall" fn(
    hVolumeOrItem: HANDLE,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetProgressDescription = extern "stdcall" fn(
    lpStr: LPWSTR,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetProgressPercentage = extern "stdcall" fn(
    nPercent: DWORD,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfHideProgress = extern "stdcall" fn(
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfShouldStop = extern "stdcall" fn(
) -> BOOL;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetUserInput = extern "stdcall" fn(
    lpMessage: LPWSTR,
    lpBuffer: LPWSTR,
    nBufferLen: DWORD,
    nFlags: DWORD
) -> __int64;



#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetExtractedMetadata = extern "stdcall" fn(
    nItemID: LONG,
) -> LPWSTR;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetMetadataEx = extern "stdcall" fn(
    hItem: HANDLE,
    lpnFlags: PDWORD
) -> LPVOID;



#[allow(non_snake_case, unused_variables)]
pub type FnXwfReleaseMem = extern "stdcall" fn(
    lpBuffer: PVOID,
) -> BOOL;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetItemOfs = extern "stdcall" fn(
    nItemID: LONG ,
    nDefOfs: LPINT64,
    nStartSector: LPINT64,
);

#[allow(non_snake_case, unused_variables)]
pub type FnXwfGetComment = extern "stdcall" fn(
    nItemID: LONG ,
) -> LPWSTR;


#[allow(non_snake_case, unused_variables)]
pub type FnXwfCreateFile = extern "stdcall" fn(
    pName: LPWSTR,
    nCreationFlags: DWORD,
    nParentItemID: LONG,
    pSourceInfo: PVOID
) -> LONG;

#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetItemSize = extern "stdcall" fn(
    nItemID: LONG,
    nSize: __int64,
);


#[allow(non_snake_case, unused_variables)]
pub type FnXwfSetItemParent = extern "stdcall" fn(
    nChildItemID: LONG,
    nParentItemID: LONG
);




