// Alex Ozdemir <aozdemir@hmc.edu>

//! This crate is designed to assist in parsing values out of input streams
//! (or any other constructs which implement Read). It does so by providing a
//! trait, [`Scan`](trait.Scan.html) which can be used to tokenize and parse
//! input.
//!
//! This crate also provides an implementation of `Scan`, [`Scanner`]
//! (struct.Scanner.html).
//!
//! # Quick Start
//! Simply include `extern crate scan` at the top of your project, add
//! `scan = "0.1.0"` to your project's dependencies, and you are ready to roll!
//!
//! ```no_run
//! use scan::Scan;
//! let mut scanner = scan::from_stdin();
//! let my_int = scanner.next::<i32>();
//! match my_int {
//!     Ok(int) => println!("Integer: {}", int),
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```
//!
//! # Features
//!
//! This crate provides a built-in implementation of Scan: Scanner. A scanner
//! can be constructed from any implementation of read, and helper functions
//! are provided for concisely creating a scanner of a file or of standard
//! input.
//!
//! By default a Scanner treats whitespace (space, tab, line feed, and carriage
//! return) as the delimiter for what to parse; however, you can also provide
//! a different set of delimiters.
//!
//! Scan allows for any structure implementing `std::str::FromStr` to be
//! parsed. This includes most types built-in to the Rust language, but you
//! may also implement FromStr for any type you define.

#![feature(io)]
#![crate_type = "lib"]
#![crate_name = "scan"]

mod scan;

pub use scan::{Scanner, Scan, ScanError, from_path, from_stdin};
