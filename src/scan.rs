use std::io::{Read, Chars, CharsError, BufReader};
use std::io;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::fmt::{Debug, Display};
use std::fmt;
use std::any::Any;
use std::error::Error;

/// Enum representing an error which can arise when extracting an F from a
/// readable stream.
///
/// # Examples
///
/// ```
/// # use scan::{Scan, from_stdin, ScanError};
/// let mut scanner = from_stdin();
/// let result = scanner.next::<i32>();
/// match result {
///     Err(ScanError::Parse(e)) => println!("Could not parse! Error: {}", e),
///     Err(ScanError::Io(e)) => println!("I/O related error: {}", e),
///     Err(ScanError::EndOfFile) => println!("No more input to read!"),
///     Ok(v) => println!("Integer scanned: {}", v),
/// }
/// ```
#[derive(Debug)]
pub enum ScanError<F: FromStr> {
    /// A string was tokenized but could not be parsed
    Parse(F::Err),
    /// An I/O error occured, including a potential UTF-8 error
    Io(CharsError),
    /// No input left to tokenize
    EndOfFile
}

impl<F> fmt::Display for ScanError<F> where
    F: FromStr + Debug,
    <F as FromStr>::Err: Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScanError::Parse(ref err) => write!(f, "Parse error: {}", err),
            ScanError::Io(ref err) => write!(f, "IO error: {}", err),
            ScanError::EndOfFile => write!(f, "End of File Error"),
        }
    }
}


impl<F: FromStr> From<io::CharsError> for ScanError<F> {
    fn from(e: io::CharsError) -> ScanError<F> { ScanError::Io(e) }
}

impl<F: FromStr> From<io::Error> for ScanError<F> {
    fn from(e: io::Error) -> ScanError<F> { ScanError::Io(CharsError::Other(e)) }
}

impl<F> Error for ScanError<F> where
    F: FromStr + Debug,
    <F as FromStr>::Err: Error + Any {

    fn description(&self) -> &str {
        match *self {
            ScanError::Parse(ref err) => err.description(),
            ScanError::Io(ref err) => Error::description(err),
            ScanError::EndOfFile => &"End of File reached before next token"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ScanError::Parse(ref err) => Some(err),
            ScanError::Io(ref err) => Some(err),
            ScanError::EndOfFile => None,
        }
    }
}

/// A Trait for enabling values to be tokenized and then parsed into types
/// implementing FromStr
///
/// Requires a function
/// `next_str(&mut self) -> Option<Result<String, CharsError>>`
/// to be implemented. This is then used to provide a method
/// `next<F: FromStr>(&mut self) -> F`
/// which takes the next string and tries to parse it as an F.
///
/// # Examples
///
/// You can scan for most built-in types
///
/// ```
/// use scan::{Scan, from_stdin};
/// let mut scanner = from_stdin(); // Yields a Scanner which implements Scan
/// let int = scanner.next::<u32>();
/// match int {
///     Ok(i) => println!("The unsigned integer is {}", i),
///     Err(e) => println!("Error: {}", e),
/// };
/// let float = scanner.next::<f32>();
/// ```
///
/// You can also scan for any type that implements std::str::FromStr
///
/// ```
/// # use scan::{Scan, from_stdin};
/// # use std::str::FromStr;
/// struct MyType { value: i32 };
/// impl FromStr for MyType {
///     type Err = ();
///     fn from_str(s: &str) -> Result<MyType, ()> { Ok( MyType{ value: 3 } ) }
/// }
///
/// let mut scanner = from_stdin();
/// let mine = scanner.next::<MyType>();
/// ```
pub trait Scan {

    /// Extract the next string. There are three possible results:
    ///    * A valid `String`, as `Some(Ok(String))`
    ///    * An indication that there are no more strings, as `None`
    ///    * An indication that some error occured and new characters could
    ///      not be produced, as `Some(Err(Error))`
    /// The choice to return an `Option<Result>` instead of the other way
    /// around was made to mirror the convention of the `io::Chars` struct.
    fn next_str(&mut self) -> Option<Result<String, CharsError>>;

    /// Parse the next string in the scanner as an `F: FromStr`.
    fn next<F: FromStr>(&mut self) -> Result<F,ScanError<F>> {
        self.next_str().map(|res| res
            .map_err(|io_err| ScanError::Io(io_err))
            .and_then(|s| s.parse::<F>()
                .map_err(|fmt_err| ScanError::Parse(fmt_err))
            )
        ).unwrap_or(Err(ScanError::EndOfFile))
    }
}

/// Scanner is the built-in implementation of the Scan trait. It can be
/// constructed in a few ways.
///
/// # Examples
/// Get a scanner of standard input
///
/// ```
/// let scanner = scan::from_stdin();
/// ```
///
/// Get a scanner of a file
///
/// ```
/// let result = scan::from_path("path/to/file.ext");
/// match result {
///     Ok(scanner) => println!("Got a scanner!"),
///     Err(e) => println!("Could not find/open file"),
/// }
/// ```
///
/// Get a scanner of something implementing Read
///
/// ```no_run
/// let reader = std::fs::File::open(&"path/to/file.ext").unwrap();
/// let scanner = scan::Scanner::new(reader);
/// ```
///
/// Get a scanner of something implementing Read, with a custom delimiter
///
/// ```no_run
/// let reader = std::fs::File::open(&"path/to/file.ext").unwrap();
/// fn is_comma(c: &char) -> bool { *c == ',' }
/// let comma_scanner = scan::Scanner::custom(reader, is_comma);
/// ```
#[derive(Debug)]
pub struct Scanner<I, F> {
    chars: I,
    ignore: F,
}

fn is_white(c: &char) -> bool {
    *c == ' '  || *c == '\t' || *c == '\n' || *c == '\r'
}

impl<R: Read, F: Fn(&char) -> bool> Scanner<Chars<R>, F> {

    /// Creates a custom Scanner, given something that can be read and a
    /// function which indicates whether a character is a delimiter.
    ///
    /// The function should return true if that character is a delimiter.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let reader = std::fs::File::open(&"path/to/file.ext").unwrap();
    /// fn is_comma(c: &char) -> bool { *c == ',' }
    /// let comma_scanner = scan::Scanner::custom(reader, is_comma);
    /// ```
    pub fn custom(reader: R, f: F) -> Scanner<Chars<R>, F> {
        Scanner{ chars: reader.chars(), ignore: f }
    }
}

impl<R: Read> Scanner<Chars<R>, fn(&char) -> bool> {

    /// Creates a scanner of something that can be read. Separates tokens by
    /// whitespace.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use scan::Scan;
    /// let reader = std::fs::File::open(&"path/to/file.ext").unwrap();
    /// let mut scanner = scan::Scanner::new(reader);
    /// let int = scanner.next::<i32>().unwrap();
    /// ```
    pub fn new(reader: R) -> Scanner<Chars<R>, fn(&char) -> bool> {
        Scanner::custom(reader, is_white)
    }
}

impl<I, F> Scan for Scanner<I, F> where
    I: Iterator<Item = Result<char,CharsError>>,
    F: Fn(&char) -> bool {
    fn next_str(&mut self) -> Option<Result<String, CharsError>> {
        let mut out = String::new();
        loop {
            match self.chars.next() {
                None => {
                    if out.len() > 0 {
                        return Some(Ok(out));
                    }
                    else {
                        return None;
                    }
                },
                Some(res) => {
                    match res {
                        Ok(c) => {
                            if (self.ignore)(&c) && out.len() > 0 {
                                return Some(Ok(out));
                            }
                            else if !(self.ignore)(&c) {
                                out.push(c);
                            }
                        }
                        Err(e) => {
                            return Some(Err(e));
                        }
                    }
                },
            }
        }
    }
}

/// Creates a scanner of the file located at `path`. Returns a result that is
/// either a scanner or an IOError.
///
/// # Examples
///
/// ```no_run
/// use scan::Scan;
/// let result = scan::from_path("path/to/file.ext");
/// match result {
///     Ok(mut scanner) => {
///         let int = scanner.next::<i32>().unwrap();
///         let int2: i32 = scanner.next().unwrap();
///     }
///     Err(_) => println!(":["),
/// }
/// ```
pub fn from_path<P: AsRef<Path>>(path: P) ->
    io::Result<Scanner<Chars<BufReader<File>>, fn(&char) -> bool>> {

    let file = try!(File::open(path));
    Ok(Scanner::new(BufReader::new(file)))
}

/// Creates a scanner of standard input. Returns the `Scanner` itself - not a
/// result.
///
/// # Examples
///
/// ```no_run
/// use scan::Scan;
/// let mut scanner = scan::from_stdin();
/// let my_str = scanner.next::<String>().unwrap();
/// ```
pub fn from_stdin() -> Scanner<Chars<io::Stdin>, fn(&char) -> bool> {
    Scanner::new(io::stdin())
}
