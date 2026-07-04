use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

macro_rules! parse {
    ($iter: expr) => {
        $iter.next().ok_or(Error::Iter)?.parse::<u32>()?
    };
}

fn solve(input: &str) -> Result<u32, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (x, y, l, r, a, b) = (parse!(iter), parse!(iter), parse!(iter), parse!(iter), parse!(iter), parse!(iter));
    let shift_range = l..r;
    let ans = if shift_range.contains(&a) && shift_range.contains(&b) {
        (b - a) * x
    }
    else if shift_range.contains(&b) {
        (l - a) * y + (b - l) * x
    }
    else if shift_range.contains(&a) {
        (r - a) * x + (b - r) * y
    }
    else if a < l && b >= r {
        (l + b - a - r) * y + (r - l) * x
    }
    else {
        (b - a) * y
    };

    Ok(ans)
}

#[derive(Debug)]
enum Error {
    Input(std::io::Error),
    Iter,
    Parse(ParseIntError),
    Write(std::fmt::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Input(e) => write!(f, "unable to fetch input: {e}"),
            Error::Iter => write!(f, "error fetching value from iterator"),
            Error::Parse(e) => write!(f, "error parsing element: {e}"),
            Error::Write(e) => write!(f, "error writing to string: {e}"),
        }
    }
}

impl From<std::fmt::Error> for Error {
    fn from(value: std::fmt::Error) -> Self {
        Self::Write(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::Parse(value)
    }
}

impl Termination for Error {
    fn report(self) -> std::process::ExitCode {
        match self {
            Error::Input(_) => ExitCode::from(1),
            Error::Iter => ExitCode::from(2),
            Error::Parse(_) => ExitCode::from(3),
            Error::Write(_) => ExitCode::from(4),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "700 300 9 17 7 21
";
        std::assert_matches!(solve(input), Ok(7400));
    }

    #[test]
    fn test_2() {
        let input = "600 500 9 17 17 20
";
        std::assert_matches!(solve(input), Ok(1500));
    }

    #[test]
    fn test_3() {
        let input = "900 200 12 14 11 13
";
        std::assert_matches!(solve(input), Ok(1100));
    }
}
