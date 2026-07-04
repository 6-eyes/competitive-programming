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

fn solve(input: &str) -> Result<&'static str, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (a, b) = (parse!(iter), parse!(iter));

    if 3 * a > 2 * b {
        Ok("Yes")
    }
    else {
        Ok("No")
    }
}

#[derive(Debug)]
enum Error {
    Input(std::io::Error),
    Iter,
    Parse(ParseIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Input(e) => write!(f, "unable to fetch input: {e}"),
            Error::Iter => write!(f, "error fetching value from iterator"),
            Error::Parse(e) => write!(f, "error parsing element: {e}"),
        }
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
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "316 465
";
        std::assert_matches!(solve(input), Ok("Yes"));
    }

    #[test]
    fn test_2() {
        let input = "101 248
";
        std::assert_matches!(solve(input), Ok("No"));
    }

    #[test]
    fn test_3() {
        let input = "666 999
";
        std::assert_matches!(solve(input), Ok("No"));
    }
}
