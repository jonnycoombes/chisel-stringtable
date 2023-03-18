//! Simple hash-based string table for use when interning and such. Protected by a r/w lock
//! so should be reasonably safe to use between threads. By default, fxhash is used for speed
//! and simplicity, although don't rely on this implementation if you're looking for cryptographic
//! soundness.
pub mod btree_string_table;
mod common;
#[cfg(test)]
mod test;
