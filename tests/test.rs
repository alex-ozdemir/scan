extern crate scan;

use std::io;
use std::fs;
use scan::Scan;
use scan::Scanner;

#[test]
fn it_works() {
    let filename = "tests/test.txt";
    let file = match fs::File::open(&filename) {
        Ok(file) => file,
        Err(err) => panic!("Error with file: {}, {}", filename, err),
    };
    let mut input = Scanner::new(io::BufReader::new(&file));
    let first = input.next_str();
}
