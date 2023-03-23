//! Common traits, types etc...

use std::borrow::Cow;

/// Trait defining the basic operations required by a string table
pub trait StringTable<'a, Key : PartialOrd + Copy>
{
    /// Add a new value to the string table. Returns the [u32] hash value
    /// of the entry.  If the entry already exists within the table, then
    /// this operation is idempotent, but the hash value is still returned.
    fn add(&mut self, value : &str) -> Key;

    /// Attempts to remove a given value from the table.
    fn remove(&mut self, key: Key) -> ();

    /// Attempts to retrieve a given value from the table.
    fn get(&self, key : Key) -> Option<&Cow<'a, str>>;

    /// The number of elements currently within the table
    fn len(&self) -> usize;

    fn contains(&self, value : &str) -> bool;

    fn hash(&self, value : &str) -> u64;

}
