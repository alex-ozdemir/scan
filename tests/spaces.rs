extern crate scan;

use std::io;
use std::fs;
use scan::{Scan, Scanner};

#[test]
fn read_one() {
    let filename = "tests/spaces.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(&file));
    let first = input.next_str().unwrap().unwrap();
    assert!("one" == first);
}

#[test]
fn read_all_check_none_end() {
    let filename = "tests/spaces.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(&file));
    let first = input.next_str().unwrap().unwrap();
    assert!(first == "one");
    let second = input.next_str().unwrap().unwrap();
    assert!(second == "more");
    let third = input.next_str().unwrap().unwrap();
    assert!(third == "word");
    let fourth = input.next_str();
    assert!(fourth.is_none());
    let fifth = input.next_str();
    assert!(fifth.is_none());
}
