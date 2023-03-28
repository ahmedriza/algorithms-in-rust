//! Items that can be stored in symbol tables
use std::fmt::Debug;

use rand::{thread_rng, Rng};

/// Interface of items that can be stored in a symbol table
pub trait Item: Debug {
    type Key: Ord; // key has to be comparable

    fn key(&self) -> Self::Key;
    fn null(&self) -> bool;
    fn rand(&mut self);
}

// This allows us to compare vectors of type Vec<&dyn Item<Key>> for an Key that has an `Ord`
// implementation.
//
// Items are equal if their keys are equal. Due to the requirement on Item::Key to be `Ord`, we
// also need the type `T` to be `Ord`.
impl<T: Ord> PartialEq for dyn Item<Key = T> + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

// -------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub struct GenericItem<K, V> {
    key: K,
    null_key: K,
    value: V,
}

impl <K: Debug, V: Debug> Debug for GenericItem<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericItem").field("key", &self.key).field("value", &self.value).finish()
    }
}

impl<K: Default, V: Default> Default for GenericItem<K, V> {
    fn default() -> Self {
        Self {
            key: Default::default(),
            null_key: Default::default(),
            value: Default::default(),
        }
    }
}

impl<K: Ord, V> PartialEq for GenericItem<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Clone + Debug + Ord, V: Debug> Item for GenericItem<K, V> {
    type Key = K;

    fn key(&self) -> Self::Key {
        self.key.clone()
    }

    fn null(&self) -> bool {
        self.key == self.null_key
    }

    fn rand(&mut self) {
        unimplemented!()
    }
}

impl<K: Default, V: Default> GenericItem<K, V> {
    pub fn new(key: K) -> Self {
        Self {
            key,
            null_key: K::default(),
            value: V::default(),
        }
    }
}

// -------------------------------------------------------------------------------------------------

const MAX_KEY: usize = 1000;
type DoubleItemKey = usize;

#[derive(Clone, Copy, Debug)]
pub struct DoubleItem {
    key_val: DoubleItemKey,
    info: f64,
}

impl Default for DoubleItem {
    fn default() -> Self {
        Self {
            key_val: MAX_KEY,
            info: Default::default(),
        }
    }
}

impl PartialEq for DoubleItem {
    fn eq(&self, other: &Self) -> bool {
        self.key_val == other.key_val
    }
}

impl DoubleItem {
    pub fn new() -> Self {
        Self {
            key_val: MAX_KEY,
            info: 0.0,
        }
    }

    pub fn with_key(key_val: DoubleItemKey) -> Self {
        Self { key_val, info: 0.0 }
    }
}

impl Item for DoubleItem {
    type Key = DoubleItemKey;

    fn key(&self) -> DoubleItemKey {
        self.key_val
    }

    fn null(&self) -> bool {
        self.key_val == MAX_KEY
    }

    fn rand(&mut self) {
        let mut _r = thread_rng();
        self.key_val = _r.gen::<Self::Key>();
        self.info = _r.gen::<f64>();
    }
}
