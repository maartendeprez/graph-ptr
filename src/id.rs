use std::{
    process::abort,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) struct StoreId(usize);

impl StoreId {
    pub(crate) const fn invalid() -> Self {
        Self(0)
    }

    pub(crate) fn new() -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        if id > isize::MAX as usize {
            abort()
        }
        Self(id)
    }
}
