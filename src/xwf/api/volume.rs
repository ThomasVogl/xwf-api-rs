use std::collections::{HashMap, HashSet};
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::shared::ntdef::{HANDLE, LONG, LPWSTR, PVOID};
use crate::get_raw_api;
use crate::xwf::api::util::{wchar_ptr_to_string, wchar_str_to_string};

use crate::xwf::api::error::XwfError;
use crate::xwf::api::item::Item;
use crate::xwf::xwf_types::{ItemInfoFlags, PropType};
use crate::xwf::raw_api::RAW_API;

#[allow(dead_code)]
enum VsPropType {
    SpecialItemId = 10,
    HashType1 =     20,
    HashType2 =     21,
    SetHashType1 =  25,
    SetHashType2 =  26
}

pub enum VolumeNameType {
    SHORT =  3,
    NORMAL = 2,
    LONG =   1
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

pub struct ItemIterator {
    idx: u32,
    max: u32
}

impl ItemIterator {
    pub fn create(min:u32, max: u32) -> Self {
        Self {
            idx: min,
            max: max
        }
    }
}

impl Iterator for ItemIterator {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.max {
            let i = self.idx as i32;
            self.idx+=1;
            Some(Item::new(i))
        } else {
            None
        }
        
    }
}

pub struct Volume {
    volume_handle: HANDLE,
}


impl Volume {
    pub fn new(volume_handle: HANDLE) -> Result<Volume, XwfError> {
        if volume_handle == null_mut() {
            return Err(XwfError::InputHandleIsNull)
        }
        Ok(Volume {
            volume_handle
        })
    }


    pub fn handle(&self) ->  HANDLE { self.volume_handle }

    pub fn get_name(&self, name_type: VolumeNameType) -> String {
        let mut array = [0u16;256];
        (get_raw_api!().get_volume_name)(self.volume_handle, array.as_mut_ptr(), name_type as DWORD);
        wchar_str_to_string(&array)
    }

    pub fn select(&self) -> Result<i32, XwfError> {
        let ret = (get_raw_api!().select_volume_snapshot)(self.volume_handle);
        if ret < 0 {
            return Err(XwfError::XwfFunctionCallFailed("select_volume_snapshot"));
        }
        Ok(ret)
    }

    pub fn get_hash_type(&self, get_secondary: bool) -> Option<HashType> {
        let mut prop_type = VsPropType::HashType1;
        if get_secondary { prop_type = VsPropType::HashType2; }
        let ret = (get_raw_api!().get_vs_prop)(prop_type as LONG, null_mut());
        if ret <= 0 {
            return None;
        }
        Some(HashType::try_from(ret).unwrap())
    }

    pub fn set_hash_type(&self, hash_type: HashType, set_secondary: bool) -> Result<(), XwfError>{

        let mut prop_type = VsPropType::SetHashType1;
        if set_secondary { prop_type = VsPropType::SetHashType2; }

        let _buf_hash_type = hash_type as u32;

        let ret = (get_raw_api!().get_vs_prop)(prop_type as LONG, _buf_hash_type.to_le_bytes().as_ptr() as PVOID);

        if ret < 0 {
            Err(XwfError::XwfFunctionCallFailed("get_vs_prop"))
        } else {
            Ok(())
        }
    }

    pub fn get_item_count(&self) -> u32 {
        (get_raw_api!().get_item_count)(null_mut())
    }

    pub fn get_item_count_dbc(&self) -> u32 {
        (get_raw_api!().get_item_count)(1 as LPVOID)
    }

    pub fn get_prop(&self, prop_type: PropType) -> i64 {
        (get_raw_api!().get_prop)(self.volume_handle, prop_type as DWORD, null_mut())
    }

    pub fn get_name_2(&self) -> String {
        wchar_ptr_to_string((get_raw_api!().get_prop)(self.volume_handle, PropType::PointerName as DWORD, null_mut()) as LPWSTR)
    }

    pub fn close(&self) {
        (get_raw_api!().close)(self.volume_handle);
    }

    pub fn iter_mut(&mut self) -> Result<ItemIterator, XwfError> {
        self.select()?;
        Ok(ItemIterator::create(0, self.get_item_count()))
    }

    pub fn iter(&self) -> Result<ItemIterator, XwfError> {
        self.select()?;
        Ok(ItemIterator::create(0, self.get_item_count()))
    }

    pub fn get_parent_dirs(&self, items: &Vec<u32>) -> HashSet<u32> {
        items.iter()
        .map(|i: &u32| Item::new(*i as i32).get_parent_dir())
        .filter(|i| i.is_some())
        .map(|i| i.unwrap().item_id as u32)
        .collect()
    }

    pub fn get_parent_items(&self, items: &HashSet<Item>) -> HashMap<Item, Vec<Item>> 
    {

        let mut ret: HashMap<Item, Vec<Item>> = HashMap::new();

        items.iter()
        .filter_map(|i| i.get_parent_item().map(|parent| (parent, i)))
        .for_each(|i| {
            match &mut ret.get_mut(&i.0) {
                Some(v) => {v.push(*i.1);},
                None => { ret.insert(i.0, vec![*i.1]);},
            };
        });

        ret
    }


    pub fn get_child_items_with_pred<F>(&self, parent_items: &HashSet<Item>,  mut pred: F) -> Result<HashMap<Item, Vec<Item>>, XwfError>
        where
            F: FnMut(&Item) -> bool
    {
        let mut ret: HashMap<Item, Vec<Item>> = HashMap::new();
        
        parent_items.iter().for_each(|f| {
            ret.insert(*f, vec![]);
        });

        let parent_items: HashSet<&Item> = parent_items.iter().filter(|p| {
            p.get_item_info_flags().unwrap_or_default().contains(ItemInfoFlags::HasChildObjects)
        }).collect();

        self.iter()?
        .filter(|i| pred(i))
        .filter_map(|i| i.get_parent_item().map(|r: Item| (r, i)))
        .filter(|i| parent_items.contains(&i.0))
        .for_each(|i| {
            match &mut ret.get_mut(&i.0) {
                Some(v) => {v.push(i.1);},
                None => { ret.insert(i.0, vec![i.1]);},
            };
        });

        Ok(ret)
    }


    pub fn get_child_items_single_with_pred<F>(&self, parent_item: &Item,  mut pred: F) -> Result<Vec<Item>, XwfError>
        where
            F: FnMut(&Item) -> bool
    {
        let mut ret: Vec<Item> = Vec::new();

        if parent_item.get_item_info_flags().is_ok_and(|f| !f.contains(ItemInfoFlags::HasChildObjects)) {
            return Ok(ret);
        }

        let it = ItemIterator::create(parent_item.item_id as u32, self.get_item_count());
        
        it
        .filter(|i| pred(i))
        .filter(|i| i.get_parent_item().is_some_and(|i| i == *parent_item) )
        .for_each(|i| {
            ret.push(i)
        });

        Ok(ret)
    }


    pub fn get_recursive_child_items_with_pred<F>(&self, parent_items: &HashSet<Item>,  mut pred: F) -> Result<HashMap<Item, Vec<Item>>, XwfError>
        where
            F: FnMut(&Item) -> Result<bool, XwfError>
    {

        let mut ret: HashMap<Item, Vec<Item>> = HashMap::new();

        parent_items.iter().for_each(|f| {
            ret.insert(*f, vec![]);
        });

        let parent_items: HashSet<&Item> = parent_items.iter().filter(|p| {
            p.get_item_info_flags().unwrap_or_default().contains(ItemInfoFlags::HasChildObjects)
        }).collect();

        self.iter()?
        .flat_map(|item| item.get_hierarchy().iter().map(|ancestor| (*ancestor, item)).collect::<Vec<(Item, Item)>>())
        .filter_map(|i| parent_items.get(&i.0).map(|r: &&Item| -> (Item, Item) {(**r, i.1)}))
        .filter( |i| pred(&i.1).unwrap_or(false))
        .for_each(|f| {
            match ret.get_mut(&f.0) {
                Some(v) => {v.push(f.1);},
                None => {ret.insert(f.0, vec![f.1]);}
            };
        });

        Ok(ret)
    }


    pub fn get_items_with_pred<F>(&self, mut pred: F) -> Result<Vec<u32>, XwfError>
        where
            F: FnMut(Item) -> Result<bool, XwfError>
    {

        let mut ret: Vec<u32> = Vec::new();

        self.select()?;

        for i in 0..self.get_item_count() {
            let item = Item::new(i as i32);

            if pred(item)? == true {
                ret.push(i);
            }
        }
        Ok(ret)
    }
}


unsafe impl Send for Volume {}
unsafe impl Sync for Volume {}