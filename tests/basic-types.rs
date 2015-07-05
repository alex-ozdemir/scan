extern crate scan;

use std::io;
use std::fs;
use scan::{Scan, Scanner};

#[test]
fn get_i64() {
    let filename = "tests/basic-types.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(file));
    let first = input.next::<i64>().unwrap().unwrap();
    assert_eq!(64, first);
    let second = input.next::<i64>().unwrap().unwrap();
    assert_eq!(-34, second);
    let third = input.next::<i64>().unwrap().unwrap();
    assert_eq!(5, third);
    let fourth = input.next::<i64>().unwrap();
    assert!(fourth.is_err());
    let fifth = input.next::<i64>().unwrap();
    assert!(fifth.is_err());
    let sixth = input.next::<i64>().unwrap();
    assert!(sixth.is_err());
}

#[test]
fn get_u64() {
    let filename = "tests/basic-types.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(file));
    let first = input.next::<u64>().unwrap().unwrap();
    assert_eq!(64, first);
    let second = input.next::<u64>().unwrap();
    assert!(second.is_err());
    let third = input.next::<u64>().unwrap().unwrap();
    assert_eq!(5, third);
    let fourth = input.next::<u64>().unwrap();
    assert!(fourth.is_err());
    let fifth = input.next::<u64>().unwrap();
    assert!(fifth.is_err());
    let sixth = input.next::<u64>().unwrap();
    assert!(sixth.is_err());
}

#[test]
fn get_f64() {
    let filename = "tests/basic-types.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(file));
    let first = input.next::<f64>().unwrap().unwrap();
    assert_eq!(64.0, first);
    let second = input.next::<f64>().unwrap().unwrap();
    assert_eq!(-34.0, second);
    let third = input.next::<f64>().unwrap().unwrap();
    assert_eq!(5.0, third);
    let fourth = input.next::<f64>().unwrap().unwrap();
    assert_eq!(1e21, fourth);
    let fifth = input.next::<f64>().unwrap().unwrap();
    assert_eq!(12.3, fifth);
    let sixth = input.next::<f64>().unwrap();
    assert!(sixth.is_err());
}
