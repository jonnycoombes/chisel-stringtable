use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chisel_stringtable::btree_string_table::BTreeStringTable;
use chisel_stringtable::common::StringTable;

// the total number of words in the fixture file
const WORD_COUNT : usize = 479826;

fn load_words() -> impl StringTable<'static, u32> {
    let mut table = BTreeStringTable::new();
    let path = env::current_dir()
        .unwrap()
        .join("tests/fixtures/words");
    let words = BufReader::new(File::open(path).unwrap()).lines();
    for word in words.flatten() {
        table.add(word.as_str());
    }
    table
}

#[test]
fn should_not_contain_collisions() {
    let words = load_words();
    assert_eq!(WORD_COUNT, words.len())
}

#[test]
fn should_perform_positive_lookups_correctly() {
    let words = load_words();
    assert!(words.contains("abada"));
    assert!(words.contains("alphabet"));
    assert!(words.contains("abwab"));
    assert!(words.contains("buzzed"));
    assert!(words.contains("civilizee"));
    assert!(words.contains("jitter"));
    assert!(words.contains("mastras"));
    assert!(words.contains("neotype"));
    assert!(words.contains("prayer-granting"));
    assert!(words.contains("sloted"));
    assert!(words.contains("truthy"));
    assert!(words.contains("xyphoid"));
    assert!(words.contains("zymurgy"));
}

#[test]
fn should_perform_negative_lookups_correctly() {
    let words = load_words();
    assert!(!words.contains("zjfgkdf"));
    assert!(!words.contains("l.s.f.s.w.r.t"));
    assert!(!words.contains("erersdfsv"));
    assert!(!words.contains("comcomcomcocmcoasdfasd"));
    assert!(!words.contains("asdfaworofvf"));
    assert!(!words.contains("gibberbiffereidsfsdfs"));
}

#[test]
fn should_return_a_cow() {
    let words = load_words();
    let hash = fxhash::hash32("alphabet");
    let cow = words.get(hash);
    assert!(&cow.is_some());
    let value = String::from(format!("{}", cow.unwrap()));
    assert_eq!(value, "alphabet")
}
