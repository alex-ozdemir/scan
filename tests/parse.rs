extern crate scan;

use std::io;
use std::fs;
use std::str::FromStr;
use scan::{Scan, Scanner, ScanError};

#[test]
fn parse_i64() {
    let filename = "tests/basic-types.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(file));
    let first = input.next::<i64>().unwrap();
    assert_eq!(64, first);
    let second = input.next::<i64>().unwrap();
    assert_eq!(-34, second);
    let third = input.next::<i64>().unwrap();
    assert_eq!(5, third);
    let fourth = input.next::<i64>();
    assert!(fourth.is_err());
    let fifth = input.next::<i64>();
    assert!(fifth.is_err());
    let sixth = input.next::<i64>();
    assert!(sixth.is_err());
}

#[test]
fn parse_u64() {
    let filename = "tests/basic-types.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(file));
    let first = input.next::<u64>().unwrap();
    assert_eq!(64, first);
    let second = input.next::<u64>();
    assert!(second.is_err());
    let third = input.next::<u64>().unwrap();
    assert_eq!(5, third);
    let fourth = input.next::<u64>();
    assert!(fourth.is_err());
    let fifth = input.next::<u64>();
    assert!(fifth.is_err());
    let sixth = input.next::<u64>();
    assert!(sixth.is_err());
}

#[test]
fn parse_f64() {
    let filename = "tests/basic-types.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(file));
    let first = input.next::<f64>().unwrap();
    assert_eq!(64.0, first);
    let second = input.next::<f64>().unwrap();
    assert_eq!(-34.0, second);
    let third = input.next::<f64>().unwrap();
    assert_eq!(5.0, third);
    let fourth = input.next::<f64>().unwrap();
    assert_eq!(1e21, fourth);
    let fifth = input.next::<f64>().unwrap();
    assert_eq!(12.3, fifth);
    let sixth = input.next::<f64>();
    assert!(sixth.is_err());
}

#[derive(Debug, PartialEq)]
struct Power10 {
    power: usize,
}

impl FromStr for Power10 {
    type Err = ();
    fn from_str(s :&str) -> Result<Power10, ()> {
        let mut chars = s.chars();
        match chars.next() {
            None => Err( () ),
            Some(c) => {
                if c != '1' { Err( () ) }
                else {
                    let mut count = 0;
                    loop {
                        match chars.next() {
                        Some('0') => count += 1,
                        Some(_) => return Err( () ),
                        None => return Ok(Power10{ power: count }),
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_it() {
    let filename = "tests/custom-type.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(&file));
    let first = input.next::<Power10>().unwrap();
    assert_eq!(Power10{ power:3 }, first);
    let second = input.next::<Power10>().unwrap();
    assert_eq!(Power10{ power:5 }, second);
    let third = input.next::<Power10>();
    match third {
        Err(ScanError::Parse( () )) => assert!(true),
        _ => assert!(false),
    }
    let fourth = input.next::<Power10>();
    match fourth {
        Err(ScanError::Parse( () )) => assert!(true),
        _ => assert!(false),
    }
    let fifth = input.next::<Power10>();
    assert!(fifth.is_err());
}
