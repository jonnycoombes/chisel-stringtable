use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufRead, BufReader};
use chisel_stringtable::btree_string_table::BTreeStringTable;
use chisel_stringtable::common::StringTable;

extern crate rand;
use rand::seq::SliceRandom;

// the total number of words in the fixture file
const WORD_COUNT : usize = 479826;

fn load_words() -> (Vec<u64>, impl StringTable<'static, u64>) {
    let mut table = BTreeStringTable::new();
    let mut keys : Vec<u64> = vec!();
    let path = env::current_dir()
        .unwrap()
        .join("tests/fixtures/words");
    let words = BufReader::new(File::open(path).unwrap()).lines();
    for word in words.flatten() {
        keys.push(table.add(word.as_str()));
    }
    (keys, table)
}

#[test]
fn should_not_contain_collisions() {
    let (_, words) = load_words();
    assert_eq!(WORD_COUNT, words.len())
}

#[test]
fn should_identity_existing_entries(){
    let (_, words) = load_words();
    let hash = words.hash("day");
    assert_eq!(words.get(hash).unwrap(), "day");
    assert!(words.contains("day").is_some());
}

#[test]
fn should_add_all_keys(){
    let (keys, words) = load_words();
    let samples : Vec<&u64> = keys.choose_multiple(&mut rand::thread_rng(), 5000).collect();
    for key in samples {
        assert!(words.get(*key).is_some())
    }
}

#[test]
fn should_remove_randomly() {
    let (keys, mut words) = load_words();
    let samples : Vec<&u64> = keys.choose_multiple(&mut rand::thread_rng(), 5000).collect();
    for key in samples.iter(){
        words.remove(**key)
    }
    for key in samples.iter(){
        assert!(words.get(**key).is_none())
    }
}

#[test]
fn should_return_a_cow() {
    let (_, words) = load_words();
    let mut hasher = DefaultHasher::default();
    hasher.write("1080".as_bytes());
    let hash = hasher.finish();
    let cow = words.get(hash);
    assert!(&cow.is_some());
    let value = String::from(format!("{}", cow.unwrap()));
    assert_eq!(value, "1080")
}
