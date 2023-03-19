//!
//!
use std::borrow::Cow;
use std::cell::{RefCell};
use std::collections::{BTreeMap};
use std::hash::Hasher;
use fxhash::FxHasher;
use crate::common::{StringTable};

/// Default implementation just maintains a btree keyed on a 32-bit fxhash value
#[derive()]
pub struct BTreeStringTable<'a>
{
    /// The hash function pointer
    hasher: RefCell<Box<dyn Hasher>>,

    /// The internal map carrying the actual strings
    index: BTreeMap<u64, Cow<'a, str>>
}

impl <'a> BTreeStringTable<'a>
{
    /// Create a new string table with default initial capacity using the default hash function
    pub fn new() -> Self {
        BTreeStringTable{
            hasher: RefCell::new(Box::new(FxHasher::default())),
            index: BTreeMap::default()
        }
    }

    fn hash(&self, value : &str) -> u64 {
        let mut hasher = self.hasher.borrow_mut();
        hasher.write(value.as_bytes());
        hasher.finish()
    }
}

impl <'a> StringTable<'a, u64> for BTreeStringTable<'a> {
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

}
