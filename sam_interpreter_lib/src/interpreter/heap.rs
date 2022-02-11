use std::collections::HashMap;

use uuid::Uuid;

use crate::{SamError, errors::ErrorWithMessage};

use super::data_types::SamObject;

pub struct SamHeap { 
    heap: HashMap<Uuid, SamHeapSlot> 
}

#[derive(Clone, Debug)]
struct SamHeapSlot {
    pub(crate) value: SamObject,
}

impl SamHeapSlot {
    pub fn new(object: SamObject) -> SamHeapSlot {
        SamHeapSlot {
            value: object,
        }
    }
}

impl SamHeap {
    pub fn new() -> SamHeap {
        SamHeap {
            heap: HashMap::new(),
        }
    }

    pub fn alloc(&mut self, object: SamObject) -> Uuid {
        let uuid = Uuid::new_v4();
        let slot = SamHeapSlot::new(object);
        self.heap.insert(uuid, slot);
        uuid
    }

    pub fn get_reference(&mut self, uuid: Uuid) -> Result<&mut SamObject, SamError> {
        let reference = &mut self.heap.get_mut(&uuid)
            .ok_or(ErrorWithMessage::new_box(format!("Undefined reference {:?}", &uuid).as_str()))?
            .value;
        Ok(reference)
    }

    fn free(&mut self, uuid: &Uuid) {
        self.heap.remove(uuid);
    }
}