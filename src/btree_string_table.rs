use std::borrow::Cow;
use std::collections::{BTreeMap};
use crate::common::{StringHashFunction, StringTable};

/// Default implementation just maintains a btree keyed on a 32-bit fxhash value
#[derive(Clone)]
pub struct BTreeStringTable<'a>
{
    /// The hash function pointer
    hash_func: StringHashFunction<u32>,

    /// The internal map carrying the actual strings
    index: BTreeMap<u32, Cow<'a, str>>
}

impl <'a> BTreeStringTable<'a>
{
    /// Create a new string table with default initial capacity using the default hash function
    pub fn new() -> Self {
        BTreeStringTable{
            hash_func: fxhash::hash32,
            index: BTreeMap::default()
        }
    }
    /// Create a new string table with a user-defined string hashing function
    pub fn with_hash(f : StringHashFunction<u32>) -> Self{
        BTreeStringTable {
            hash_func: f,
            index: BTreeMap::new()
        }
    }
}

impl <'a> StringTable<'a, u32> for BTreeStringTable<'a> {
    fn add(&mut self, value: &str) -> u32 {
        let hash = (self.hash_func)(value);
        if !self.index.contains_key(&hash) {
            self.index.insert(hash, Cow::Owned(String::from(value)));
        }
        hash
    }

    fn contains(&self, value: &str) -> bool {
        let hash = (self.hash_func)(value);
        self.index.contains_key(&hash)
    }

    fn remove(&mut self, value: &str) -> () {
        let hash = (self.hash_func)(value);
        if self.index.contains_key(&hash) {
            self.index.remove(&hash);
        }
    }

    fn get(&self, key: u32) -> Option<&Cow<'a, str>> {
       self.index.get(&key)
    }
}
