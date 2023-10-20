use std::ptr::{null_mut};
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::{HANDLE, LONG, LPWSTR};
use crate::xwf::api::util::{wchar_ptr_to_string, wchar_str_to_string};

use crate::xwf::api::error::XwfError;
use crate::xwf::xwf_types::PropType;
use crate::xwf::raw_api::RAW_API;


enum VsPropType {
    SpecialItemId = 10,
    HashType1 =     20,
    HashType2 =     21,
    SetHashType1 =  25,
    SetHashType2 =  26
}

pub enum VolumeNameType {
    SHORT =  1,
    NORMAL = 2,
    LONG =   3
}
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i64> for $name {
            type Error = ();

            fn try_from(v: i64) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i64 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
    #[derive(Copy, Clone)]
    pub enum HashType {
    CS8 = 1,
    CS16 = 2,
    CS32 = 3,
    CS64 = 4,
    CRC16 = 5,
    CRC32 = 6,
    MD5 = 7,
    SHA1 = 8,
    SHA256 = 9,
    RIPEMD128 = 10,
    RIPEMD160 = 11,
    MD4 = 12,
    ED2K = 13,
    ADLER32 = 14,
    TigerTreeHash = 15,
    Tiger128 = 16,
    Tiger160 = 17,
    Tiger192 = 18,
}
}


impl HashType {
    pub fn get_hash_size(&self) -> usize {
        match self {
            HashType::CS8 => 1,
            HashType::CS16 => 2,
            HashType::CS32 => 4,
            HashType::CS64 => 8,
            HashType::CRC16 => 2,
            HashType::CRC32 => 4,
            HashType::MD5 => 16,
            HashType::SHA1 => 20,
            HashType::SHA256 => 32,
            HashType::RIPEMD128 => 16,
            HashType::RIPEMD160 => 20,
            HashType::MD4 => 16,
            HashType::ED2K => 16,
            HashType::ADLER32 => 4,
            HashType::TigerTreeHash => 24,
            HashType::Tiger128 => 16,
            HashType::Tiger160 => 20,
            HashType::Tiger192 => 24,
        }
    }
}

pub struct Volume {
    volume_handle: HANDLE,
}


impl Volume {
    pub fn new(volume_handle: HANDLE) -> Option<Volume> {
        if volume_handle == null_mut() {
            return None
        }
        Some(Volume {
            volume_handle
        })
    }


    pub fn handle(&self) ->  HANDLE { self.volume_handle }

    pub fn get_name(&self, name_type: VolumeNameType) -> String {
        let mut array = [0u16;256];
        (RAW_API.get_volume_name)(self.volume_handle, array.as_mut_ptr(), name_type as DWORD);
        wchar_str_to_string(&array)
    }

    pub fn select(&self) -> Result<i32, XwfError> {
        let ret = (RAW_API.select_volume_snapshot)(self.volume_handle);
        if ret < 0 {
            return Err(XwfError::FailedToSelectVolume);
        }
        Ok(ret)
    }

    pub fn get_hash_type(&self, get_secondary: bool) -> Option<HashType> {
        let mut prop_type = VsPropType::HashType1;
        if get_secondary { prop_type = VsPropType::HashType2; }
        let ret = (RAW_API.get_vs_prop)(prop_type as LONG, null_mut());
        if ret <= 0 {
            return None;
        }
        Some(HashType::try_from(ret).unwrap())
    }

    pub fn get_item_count(&self) -> u32 {
        (RAW_API.get_item_count)(null_mut())
    }

    pub fn get_prop(&self, prop_type: PropType) -> i64 {
        (RAW_API.get_prop)(self.volume_handle, prop_type as DWORD, null_mut())
    }

    pub fn get_name_2(&self) -> String {
        wchar_ptr_to_string((RAW_API.get_prop)(self.volume_handle, PropType::PointerName as DWORD, null_mut()) as LPWSTR)
    }

    pub fn close(&self) {
        (RAW_API.close)(self.volume_handle);
    }
}