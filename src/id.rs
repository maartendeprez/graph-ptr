use std::{
    process::abort,
    sync::atomic::{AtomicUsize, Ordering},
};

static next_id: AtomicUsize = AtomicUsize::new(0);

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) struct StoreId(usize);

impl StoreId {
    pub(crate) fn new() -> Self {
        let id = next_id.fetch_add(1, Ordering::Relaxed);
        if id > isize::MAX as usize {
            abort()
        }
        Self(id)
    }
}
