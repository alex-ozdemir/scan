use std::io::{Read, Chars, CharsError};
use std::str::FromStr;
use std::fmt::{Debug, Display};
use std::fmt;
use std::any::Any;
use std::error::Error;

#[derive(Debug,PartialEq)]
pub enum ScanError<F> where F: FromStr {
    Parse(F::Err),
    Io(CharsError),
}

impl<F> fmt::Display for ScanError<F>
    where F: FromStr + Debug, <F as FromStr>::Err: Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScanError::Parse(ref err) => write!(f, "Parse error: {}", err),
            ScanError::Io(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl<F> Error for ScanError<F>
    where F: FromStr + Debug, <F as FromStr>::Err: Error + Any {

    fn description(&self) -> &str {
        match *self {
            ScanError::Parse(ref err) => err.description(),
            ScanError::Io(ref err) => Error::description(err),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ScanError::Parse(ref err) => Some(err),
            ScanError::Io(ref err) => Some(err),
        }
    }
}

pub trait Scan {

    fn next_str(&mut self) -> Option<Result<String, CharsError>>;

    fn next<F>(&mut self) -> Option<Result<F,ScanError<F>>>
        where F: FromStr {
        self.next_str().map(|res| res
            .map_err(|io_err| ScanError::Io(io_err))
            .and_then(|s| s.parse::<F>()
                .map_err(|fmt_err| ScanError::Parse(fmt_err))
            )
        )
    }

    fn next_panic<F>(&mut self) -> Option<F>
        where F: FromStr + Debug, <F as FromStr>::Err: Debug {
        self.next::<F>().map(|res| res.unwrap())
    }
}

pub struct Scanner<R> {
    chars: Chars<R>,
}

fn is_white(c: &char) -> bool {
    *c == ' '  || *c == '\t' || *c == '\n' || *c == '\r'
}

impl<R: Read> Scanner<R> {

    pub fn new(reader: R) -> Scanner<R> {
        Scanner{ chars: reader.chars() }
    }

}

impl<R: Read> Scan for Scanner<R> {
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
                            if is_white(&c) && out.len() > 0 {
                                return Some(Ok(out));
                            }
                            else if !is_white(&c) {
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
