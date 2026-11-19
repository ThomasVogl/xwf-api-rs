use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::{HashMap, HashSet};
use std::ptr::null_mut;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::shared::ntdef::{HANDLE, LONG, LPWSTR, PVOID};
use winsafe::WString;
use crate::get_raw_api;

use crate::error::XwfError;
use crate::item::Item;
use crate::xwf_types::*;
use crate::raw_api::RAW_API;




pub struct ItemIterator {
    idx: i32,
    max: i32
}

impl ItemIterator {
    pub fn create(min:i32, max: i32) -> Self {
        Self {
            idx: min,
            max
        }
    }
}

impl Iterator for ItemIterator {
    type Item = Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.max {
            let i = self.idx;
            self.idx+=1;
            Some(Item::new(i))
        } else {
            None
        }
        
    }
}

pub struct Volume {
    volume_handle: HANDLE,
    // true if this volume was opened by the X-Tension and may be closed via close()
    shall_close: AtomicBool,
}


impl Volume {
    /// Wraps a volume handle that is owned by X-Ways (e.g. passed to XT_Prepare/XT_Finalize
    /// or belonging to an evidence object that was already open). close() is a no-op for it.
    pub fn new(volume_handle: HANDLE) -> Result<Volume, XwfError> {
        Volume::create(volume_handle, false)
    }

    pub(crate) fn create(volume_handle: HANDLE, shall_close: bool) -> Result<Volume, XwfError> {
        if volume_handle == null_mut() {
            return Err(XwfError::InputHandleIsNull)
        }
        Ok(Volume {
            volume_handle,
            shall_close: AtomicBool::new(shall_close),
        })
    }


    pub fn handle(&self) ->  HANDLE { self.volume_handle }

    pub fn get_name(&self, name_type: VolumeNameType) -> String {
        let mut array = [0u16;256];
        (get_raw_api!().get_volume_name)(self.volume_handle, array.as_mut_ptr(), name_type as DWORD);
        WString::from_wchars_slice(&array).to_string()
    }

    pub fn select(&self) -> Result<i32, XwfError> {

        let num_items: LONG;
        #[cfg(feature = "api_20_9")]
        {
            num_items = (get_raw_api!().select_volume_snapshot)(self.volume_handle);

        }
        #[cfg(not(feature = "api_20_9"))]
        {
            (get_raw_api!().select_volume_snapshot)(self.volume_handle);
            num_items = self._get_item_count(false)?;
        }

        if num_items < 0 {
            return Err(XwfError::XwfFunctionCallFailed("select_volume_snapshot"));
        }
        Ok(num_items)




    }

    pub fn get_hash_type(&self, get_secondary: bool) -> Option<XwfHashType> {
        Volume::current_hash_type(get_secondary)
    }

    /// Hash type of the currently selected volume snapshot (XWF_GetVSProp needs no volume handle).
    pub fn current_hash_type(get_secondary: bool) -> Option<XwfHashType> {
        let mut prop_type = VsPropType::HashType1;
        if get_secondary { prop_type = VsPropType::HashType2; }
        let ret = (get_raw_api!().get_vs_prop)(prop_type as LONG, null_mut());
        if ret <= 0 {
            return None;
        }
        XwfHashType::try_from(ret).ok()
    }

    pub fn set_hash_type(&self, hash_type: XwfHashType, set_secondary: bool) -> Result<(), XwfError>{

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

    #[cfg(feature = "api_20_9")]
    pub fn has_changed(&self){
        Volume::mark_selected_as_changed();
    }

    /// Marks the currently selected volume snapshot as changed, so that X-Ways saves it at the
    /// latest when its data window is closed (necessary for changes to other volume snapshots than
    /// the one the X-Tension was called for).
    #[cfg(feature = "api_20_9")]
    pub fn mark_selected_as_changed() {
        // pBuffer must point to a byte with the value 0xFF
        let mut data: u8 = 0xFF;
        let _ = (get_raw_api!().get_vs_prop)(VsPropType::SetHasChanged as LONG, &mut data as *mut u8 as PVOID);
    }

    #[allow(unused_variables)]
    fn _get_item_count(&self, num_via_dbc: bool) -> Result<i32, XwfError> {
        let arg: LPVOID;
        #[cfg(feature = "api_20_3")] {
            arg = match num_via_dbc {
                false => null_mut(),
                true => 1 as LPVOID
            };
        }

        #[cfg(not(feature = "api_20_3"))] {
            arg = null_mut();
        }


        let num_items = (get_raw_api!().get_item_count)(arg);

        // check if number of items exceeds max value of int32 and if it does, raise an error.
        // nItemID is handled as a LONG value (signed integer) in X-Tension C API,
        // so a cast in this case from u32 to i32 would lead to negative nItemIDs.
        // From my understanding, nItemIDs are always positive values,
        // so a negative value probably would lead to unexpected behavior...
        if num_items > i32::MAX as u32 {
            Err(XwfError::MaxItemIdExceeded)
        } else {
            Ok(num_items as i32)
        }


    }

    pub fn get_item_count(&self) -> Result<i32, XwfError> {
        self._get_item_count(false)
    }

    //get number of items selected in directory browser context menu
    #[cfg(feature = "api_20_3")]
    pub fn get_item_count_dbc(&self) -> Result<i32, XwfError> {
        self._get_item_count(true)
    }

    pub fn get_prop(&self, prop_type: PropType) -> i64 {
        (get_raw_api!().get_prop)(self.volume_handle, prop_type as DWORD, null_mut())
    }

    pub fn get_name_2(&self) -> String {
        let ptr = (get_raw_api!().get_prop)(self.volume_handle, PropType::PointerName as DWORD, null_mut()) as LPWSTR;
        if ptr.is_null() {
            return String::new();
        }
        unsafe { WString::from_wchars_nullt(ptr).to_string() }
    }

    /// Closes the volume, but only if it was opened by the X-Tension (see Evidence::open())
    /// and has not been closed before. Volumes owned by X-Ways are never closed.
    pub fn close(&self) {
        if self.shall_close.swap(false, Ordering::SeqCst) {
            (get_raw_api!().close)(self.volume_handle);
        }
    }

    pub fn iter_mut(&mut self) -> Result<ItemIterator, XwfError> {
        self.select()?;
        Ok(ItemIterator::create(0, self.get_item_count()?))
    }

    pub fn iter(&self) -> Result<ItemIterator, XwfError> {
        self.select()?;
        Ok(ItemIterator::create(0, self.get_item_count()?))
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

        let it = ItemIterator::create(parent_item.item_id, self.get_item_count()?);
        
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


        //initialize return HashMap
        parent_items.iter().for_each(|f| {
            ret.insert(*f, vec![]);
        });

        let mut it = self.iter()?;

        while let Some(i) = it.next() {

            if !pred(&i).unwrap_or(false) { continue; }

            let hierarchy = i.get_hierarchy();
            for p in hierarchy {
                if ret.contains_key(&p) {
                    ret.get_mut(&p).unwrap().push(i);
                    break;
                }
            }
        }
        Ok(ret)
    }


    pub fn get_items_with_pred<F>(&self, mut pred: F) -> Result<Vec<i32>, XwfError>
        where
            F: FnMut(Item) -> Result<bool, XwfError>
    {

        let mut ret: Vec<i32> = Vec::new();

        let num_items = self.select()?;

        for i in 0..num_items {
            let item = Item::new(i);

            if pred(item)? == true {
                ret.push(i);
            }
        }
        Ok(ret)
    }
}


unsafe impl Send for Volume {}
unsafe impl Sync for Volume {}