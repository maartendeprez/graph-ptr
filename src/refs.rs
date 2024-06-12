use crate::id::StoreId;

pub struct Ref<T> {
    pub(crate) id: StoreId,
    pub(crate) ptr: *mut Option<T>,
}

pub struct RefBy<K, V> {
    key: K,
    ptr: Ref<V>,
}
