use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::sync::RwLock;

/// Alias for a hash function type
pub type StringHashFunction = fn(&str) -> u32;

/// Trait defining the basic operations required by a string table
pub trait StringTable {

}

/// Default implementation just maintains a btree keyed on a 32-bit fxhash value
#[derive(Debug, Clone)]
pub struct BTreeStringTable<'a> {

    /// The hash function pointer
    hash : StringHashFunction,

    /// The internal map carrying the actual strings
    index: BTreeMap<u32, String>
}

impl <'a> BTreeStringTable<'a> {

    /// Create a new string table with default initial capacity using the default hash function
    pub fn new() -> Self {
        StringTable{
            hash : fxhash::hash32,
            index: BTreeMap::default()
        }
    }

    /// Create a new string table with a user-defined string hashing function
    pub fn with_hash(f : StringHashFunction) -> Self{
        StringTable {
            hash : f,
            index: BTreeMap::with_capacity(DEFAULT_CAPACITY)
        }
    }
}
