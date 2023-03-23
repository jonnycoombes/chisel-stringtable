//! Default implementation of a [StringTable] based on an internal B-Tree index, which maps `u64`
//! hash values to owned [String] instances.  When you access a given value, you will be given a
//! [Cow] preventing unnecessary allocations within immutable use cases.
//!
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use crate::common::StringTable;

/// Default implementation just maintains a btree keyed on a 64-bit fxhash value
#[derive()]
pub struct BTreeStringTable<'a>
{
    /// The internal map carrying the actual strings
    index: BTreeMap<u64, Cow<'a, str>>,
}

impl<'a> BTreeStringTable<'a>
{
    /// Create a new string table with default initial capacity using the default hasher
    pub fn new() -> Self {
        BTreeStringTable {
            index: BTreeMap::default(),
        }
    }

    /// Hash a given string slice using the hasher
    fn hash(&self, value: &str) -> u64 {
        let mut hasher = DefaultHasher::default();
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

impl<'a> Clone for BTreeStringTable<'a> {
    /// Clone the contents of a given [BTreeStringTable] instance
    fn clone(&self) -> Self {
        BTreeStringTable {
            index: self.index.clone(),
        }
    }
}

impl<'a> StringTable<'a, u64> for BTreeStringTable<'a> {
    fn add(&mut self, value: &str) -> u64 {
        let hash = self.hash(value);
        if !self.index.contains_key(&hash) {
            self.index.insert(hash, Cow::Owned(String::from(value)));
        }
        hash
    }

    fn remove(&mut self, key: u64) -> () {
        if self.index.contains_key(&key) {
            self.index.remove(&key);
        }
    }

    fn get(&self, key: u64) -> Option<&Cow<'a, str>> {
        self.index.get(&key)
    }

    fn len(&self) -> usize {
        self.index.len()
    }

    fn contains(&self, value: &str) -> Option<u64> {
        let hash = self.hash(value);
        if self.index.contains_key(&hash) {
            return Some(hash);
        }
        None
    }

    fn hash(&self, value: &str) -> u64 {
        self.hash(value)
    }
}
