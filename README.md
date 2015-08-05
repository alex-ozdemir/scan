# scan
A tokenizing/parsing utility for Rust

Allows for whitespace-delimited tokens in an input stream to be parsed as any type implementing `FromStr`.

# Examples

```rust
extern crate scan;
use scan::Scan;

fn main() {
  let mut scanner = scan::from_stdin();
  let int = scanner.next::<i32>().unwrap();
  let int2: i32 = scanner.next().unwrap();
  println!("Integer: {}", int);
  println!("Integer 2: {}" int2);
}
```
