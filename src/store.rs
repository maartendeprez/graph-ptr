use std::ops::{Index, IndexMut};

use typed_arena::Arena;

use crate::{id::StoreId, refs::Ref};

pub struct Store<T> {
    id: StoreId,
    storage: Arena<Option<T>>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self {
            id: StoreId::new(),
            storage: Arena::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> Ref<T> {
        Ref {
            id: self.id,
            ptr: self.storage.alloc(Some(value)) as *mut Option<T>,
        }
    }

    pub fn get(&self, ptr: &Ref<T>) -> &T {
        self.try_get(ptr)
            .expect("Store::get called with invalid ref")
    }

    pub fn try_get(&self, ptr: &Ref<T>) -> Option<&T> {
        (self.id == ptr.id).then_some(())?;
        unsafe { (&*ptr.ptr).as_ref() }
    }

    pub unsafe fn get_unchecked(&self, ptr: &Ref<T>) -> &T {
        (&*ptr.ptr).as_ref().unwrap_unchecked()
    }

    pub fn get_mut(&mut self, ptr: &Ref<T>) -> &mut T {
        self.try_get_mut(ptr)
            .expect("Store::get_mut called with invalid ref")
    }

    pub fn try_get_mut(&mut self, ptr: &Ref<T>) -> Option<&mut T> {
        (self.id == ptr.id).then_some(())?;
        unsafe { (&mut *ptr.ptr).as_mut() }
    }

    pub unsafe fn get_mut_unchecked(&mut self, ptr: &Ref<T>) -> &mut T {
        (&mut *ptr.ptr).as_mut().unwrap_unchecked()
    }

    pub fn remove(&mut self, ptr: &Ref<T>) -> Option<T> {
        (self.id == ptr.id).then_some(())?;
        unsafe { (&mut *ptr.ptr).take() }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.storage.iter_mut().filter_map(|v| v.as_mut())
    }
}

impl<T> Index<&Ref<T>> for Store<T> {
    type Output = T;

    fn index(&self, index: &Ref<T>) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<&Ref<T>> for Store<T> {
    fn index_mut(&mut self, index: &Ref<T>) -> &mut Self::Output {
        self.get_mut(index)
    }
}
