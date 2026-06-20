use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    println!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<&'static str, Error> {
    let (x, y) = {
        let (x, y) = input.split_once(' ').ok_or(Error::Split)?;
        (x.trim().parse::<u32>()?, y.trim().parse::<u32>()?)
    };

    // -_-
    Ok(if 9 * x == 16 * y { "Yes" } else { "No" })
}

#[derive(Debug)]
enum Error {
    Input(std::io::Error),
    Split,
    Parse(ParseIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Input(e) => write!(f, "unable to fetch input: {e}"),
            Error::Split => write!(f, "error splitting input"),
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
            Error::Split => ExitCode::from(2),
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
        let input = r#"800 450"#;
        std::assert_matches!(solve(input), Ok("Yes"));
    }

    #[test]
    fn test_2() {
        let input = r#"234 108"#;
        std::assert_matches!(solve(input), Ok("No"));
    }
}
