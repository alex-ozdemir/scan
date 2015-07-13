extern crate scan;

use std::io;
use std::fs;
use std::str::FromStr;
use scan::{Scan, Scanner, ScanError};

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
    let filename = "tests/custom.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::new(io::BufReader::new(&file));
    let first = input.next::<Power10>().unwrap().unwrap();
    assert_eq!(Power10{ power:3 }, first);
    let second = input.next::<Power10>().unwrap().unwrap();
    assert_eq!(Power10{ power:5 }, second);
    let third = input.next::<Power10>().unwrap();
    match third {
        Err(ScanError::Parse( () )) => assert!(true),
        Err(ScanError::Io(_)) => assert!(false),
        Ok(_) => assert!(false),
    }
    let fourth = input.next::<Power10>().unwrap();
    match fourth {
        Err(ScanError::Parse( () )) => assert!(true),
        Err(ScanError::Io(_)) => assert!(false),
        Ok(_) => assert!(false),
    }
    let fifth = input.next::<Power10>();
    assert!(fifth.is_none());
}
