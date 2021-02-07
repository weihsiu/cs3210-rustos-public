// FIXME: Make me compile. Diff budget: 12 line additions and 2 characters.

// I AM NOT DONE
use std::fmt;

#[derive(Debug)]
struct ErrorA;
impl std::error::Error for ErrorA {}
impl fmt::Display for ErrorA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ErrorA")
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

#[derive(Debug)]
enum Error {
    A(ErrorA),
    B(ErrorB),
}
impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            A(e) => format!("A({})", )
        }
        write!(f, format!(""))
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
    Ok((do_a(), do_b()))
}

fn main() {}
