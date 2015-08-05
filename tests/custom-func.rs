extern crate scan;

use std::io;
use std::fs;
use scan::{Scan, Scanner};

fn is_zero(c: &char) -> bool {
    *c == '0' || *c == '\n' // EOF newline
}

#[test]
#[allow(unused_variables)]
fn test_omit_zero() {
    let filename = "tests/custom-func.txt";
    let file = fs::File::open(&filename).unwrap();
    let mut input = Scanner::custom(io::BufReader::new(&file), is_zero);
    let first = input.next::<i32>().unwrap();
    assert_eq!(3, first);
    let second = input.next::<String>().unwrap();
    assert_eq!("aa", second);
    let third = input.next::<u32>();
    println!("{:?}", third);
    assert!(third.is_err());

    // Verify the function is not consumed
    let input2 = Scanner::custom(io::BufReader::new(&file), is_zero);
}
