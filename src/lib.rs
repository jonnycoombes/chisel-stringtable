//! ## Overview
//!
//! This crate contains a very simple string table which can be used to *intern* strings by
//! stashing them all within a single data-structure, and then referencing them throughout other
//! areas of a code base using a unique `u64` identifier.
//!
//! Currently there is a single implementation of an interning data structure based on an internal
//! B-Tree index. Nothing very sophisticated going on here at all.
//!
//! ### Usage
//!
//! There really isn't much to it:
//!
//! ```rust
//!     use chisel_stringtable::btree_string_table::BTreeStringTable;
//!     use chisel_stringtable::common::StringTable;
//!
//!     let mut table = BTreeStringTable::new();
//!     let key : u64 = table.add("some value to intern");
//!     assert_eq!(table.get(key).unwrap(), "some value to intern");
//!     assert_eq!(format!("Here's me embedded string: {}", table.get(key).unwrap()),
//!                         "Here's me embedded string: some value to intern")
//! ```
pub mod btree_string_table;
pub mod common;
