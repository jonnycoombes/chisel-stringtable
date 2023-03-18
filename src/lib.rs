//! Simple hash-based string table for use when interning and such.  fxhash is used for speed
//! and simplicity, although don't rely on this implementation if you're looking for cryptographic
//! soundness.
pub mod btree_string_table;
pub mod common;
