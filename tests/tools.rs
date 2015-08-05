extern crate scan;

use scan::{Scan, from_path};

#[test]
fn build_from_path() {
    let filename = "tests/spaces.txt";
    let mut input = from_path(&filename).unwrap();
    let first = input.next_str().unwrap().unwrap();
    assert!("one" == first);
    let second = input.next::<String>().unwrap();
    assert_eq!("more", second);
}
