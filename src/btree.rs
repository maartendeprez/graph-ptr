use std::{
    borrow::Borrow,
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

use crate::{
    refs::{Ref, RefBy},
    store::Store,
};

pub struct BTreeStore<K, V> {
    index: BTreeMap<K, Ref<V>>,
    store: Store<V>,
}

impl<K, V> BTreeStore<K, V> {
    pub fn new() -> Self {
        Self {
            index: BTreeMap::new(),
            store: Store::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord + Clone,
    {
        let ptr = self.store.insert(value);
        self.index
            .insert(key.clone(), ptr.clone())
            .map(|old| self.store.remove(&old).unwrap())
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.index
            .remove(key)
            .map(|ptr| self.store.remove(&ptr).unwrap())
    }

    pub fn resolve(&self, ptr: &mut RefBy<K, V>)
    where
        K: Ord,
    {
        self.try_resolve(ptr)
            .then_some(())
            .expect("BTreeStore::resolve called with invalid key")
    }

    pub fn try_resolve(&self, ptr: &mut RefBy<K, V>) -> bool
    where
        K: Ord,
    {
        self.index.get(&ptr.key).map_or(false, |r| {
            ptr.resolve(r.clone());
            true
        })
    }

    pub fn lookup<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.lookup_ref(key).map(|ptr| self.store.get(ptr))
    }

    pub fn lookup_ref<Q>(&self, key: &Q) -> Option<&Ref<V>>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        self.index.get(key)
    }
}

impl<K, V> Index<&Ref<V>> for BTreeStore<K, V> {
    type Output = V;

    fn index(&self, index: &Ref<V>) -> &Self::Output {
        self.store.get(index)
    }
}

impl<K, V> Index<&RefBy<K, V>> for BTreeStore<K, V> {
    type Output = V;

    fn index(&self, index: &RefBy<K, V>) -> &Self::Output {
        self.store.get(&index.ptr)
    }
}

impl<K, V, Q> Index<&Q> for BTreeStore<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.store.get(&self.index[index])
    }
}

impl<K, V> IndexMut<&Ref<V>> for BTreeStore<K, V> {
    fn index_mut(&mut self, index: &Ref<V>) -> &mut Self::Output {
        self.store.get_mut(index)
    }
}

impl<K, V> IndexMut<&RefBy<K, V>> for BTreeStore<K, V> {
    fn index_mut(&mut self, index: &RefBy<K, V>) -> &mut Self::Output {
        self.store.get_mut(&index.ptr)
    }
}

impl<K, V, Q> IndexMut<&Q> for BTreeStore<K, V>
where
    K: Borrow<Q> + Ord,
    Q: Ord + ?Sized,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.store.get_mut(&self.index[index])
    }
}
