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

    /// Check whether a given value already exists within the table.  If it does, then
    /// return the hash value associated with it.  If not, just return None.
    fn contains(&self, value : &str) -> Option<u64>;

    /// Get the hash value associated with a given value.  This should be consistent and
    /// repeatable between calls for the same value.
    fn hash(&self, value : &str) -> u64;

}
