// FIXME: Make me compile. Diff budget: 12 line additions and 2 characters.
use std::fmt;

#[derive(Debug)]
struct ErrorA;
impl std::error::Error for ErrorA {}
impl fmt::Display for ErrorA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorA")
    }
}
impl From<ErrorA> for Error {
    fn from(e: ErrorA) -> Self {
        Error::A(e)
    }
}

#[derive(Debug)]
struct ErrorB;
impl std::error::Error for ErrorB {}
impl fmt::Display for ErrorB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorB")
    }
}
impl From<ErrorB> for Error {
    fn from(e: ErrorB) -> Self {
        Error::B(e)
    }
}

#[derive(Debug)]
enum Error {
    A(ErrorA),
    B(ErrorB),
}
impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::A(e) => write!(f, "A({})", e),
            Error::B(e) => write!(f, "B({})", e)
        }
    }
}

// What traits does `Error` need to implement?

fn do_a() -> Result<u16, ErrorA> {
    Err(ErrorA)
}

fn do_b() -> Result<u32, ErrorB> {
    Err(ErrorB)
}

fn do_both() -> Result<(u16, u32), Error> {
    let a = do_a()?;
    let b = do_b()?;
    Ok((a, b))
}

fn main() {}
