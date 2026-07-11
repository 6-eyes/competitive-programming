use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    // print!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> usize {
    let mut dp = vec!{ vec!{ vec!{ vec!{ 0usize ; 2 } ; 3 } ; 1<<10 } ; 501 };
    dp[0][0][0][0] = 1;

    for (i, s) in input.chars().map(|c| c as u8 - '0' as u8).enumerate() {
        // populate i + 1
    }
    todo!()
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
        let input = "45";

        assert_eq!(solve(input), 19);
    }

    #[test]
    fn test_2() {
        let input = "1013";

        assert_eq!(solve(input), 424);
    }

    #[test]
    fn test_3() {
        let input = "2";

        assert_eq!(solve(input), 0);
    }

    #[test]
    fn test_4() {
        let input = "314159265358979323846264338327950";

        assert_eq!(solve(input), 658111391);
    }
}
