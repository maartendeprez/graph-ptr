use std::ptr::NonNull;

use crate::id::StoreId;

#[derive(Debug)]
pub struct Ref<T> {
    pub(crate) id: StoreId,
    pub(crate) ptr: NonNull<Option<T>>,
}

#[derive(Clone, Debug)]
pub struct RefBy<K, V> {
    pub(crate) key: K,
    pub(crate) ptr: Ref<V>,
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            ptr: self.ptr,
        }
    }
}

impl<T> Ref<T> {
    pub(crate) fn new(id: StoreId, ptr: &mut Option<T>) -> Self {
        Self {
            id,
            ptr: unsafe { NonNull::new_unchecked(ptr) },
        }
    }

    pub fn dangling() -> Self {
        Self {
            id: StoreId::invalid(),
            ptr: NonNull::dangling(),
        }
    }
}

impl<K, V> RefBy<K, V> {
    pub fn dangling(key: K) -> Self {
        Self {
            key,
            ptr: Ref::dangling(),
        }
    }

    pub(crate) fn resolve(&mut self, ptr: Ref<V>) {
        self.ptr = ptr;
    }
}
