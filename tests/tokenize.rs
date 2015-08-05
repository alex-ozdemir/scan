extern crate scan;

use std::io;
use std::fs;
use scan::{Scan, Scanner};

#[test]
fn tokenize_one_from_spaces() {
    let filename = "tests/spaces.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(&file));
    let first = input.next_str().unwrap().unwrap();
    assert!("one" == first);
}

#[test]
fn tokenize_all_from_spaces_check_none_end() {
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

#[test]
fn tokenize_mixed_whitespace() {
    let filename = "tests/mixed.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(&file));
    let first = input.next_str().unwrap().unwrap();
    assert!(first == "one");
    let second = input.next_str().unwrap().unwrap();
    assert!(second == "more");
    let third = input.next_str().unwrap().unwrap();
    assert!(third == "word");
    let fourth = input.next_str().unwrap().unwrap();
    assert!(fourth == "one");
    let fifth = input.next_str().unwrap().unwrap();
    assert!(fifth == "more");
    let sixth = input.next_str();
    assert!(sixth.is_none());
}
