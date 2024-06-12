use std::collections::BTreeMap;

use crate::{refs::Ref, store::Store};

pub struct BTreeStore<K, V> {
    index: BTreeMap<K, Ref<V>>,
    store: Store<V>,
}
