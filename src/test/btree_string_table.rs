use std::{env};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::btree_string_table::BTreeStringTable;
use crate::common::StringTable;

fn all_words_table() -> BTreeStringTable<'static> {
    let path = env::current_dir()
        .unwrap()
        .join("src/test/fixtures/words");
    let words = BufReader::new(File::open(path).unwrap()).lines();
    let mut table = BTreeStringTable::new();
    for word in words.flatten(){
        table.add(word.trim());
    }
    table
}

#[test]
fn should_perform_lookups_correctly() {
   let words  = all_words_table();

}
