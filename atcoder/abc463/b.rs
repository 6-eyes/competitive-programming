use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    println!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<&'static str, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, x) = {
        let (n, x) = (iter.next().ok_or(Error::Iter)?, iter.next().ok_or(Error::Iter)?);
        let n = n.parse::<u8>()? - 1; // 0 indexed
        let x = x.chars().next().ok_or(Error::Iter)? as usize - 'A' as usize; // 0 indexed
        (n, x)
    };

    for _ in 0..=n {
        let line = iter.next().ok_or(Error::Iter)?;
        match line.chars().nth(x).ok_or(Error::Iter)? {
            'o' => return Ok("Yes"),
            'x' => continue,
            c => unreachable!("invalid value {c}"),
        }
    }

    Ok("No")
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
            Error::Iter => write!(f, "error getting the next input in iterator"),
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
        let input = r#"3 A
xoxox
xxooo
oxxxx
"#;
        std::assert_matches!(solve(input), Ok("Yes"));
    }

    #[test]
    fn test_2() {
        let input = r#"5 C
xoxoo
oxxoo
oxxxo
xoxxx
oxxoo
"#;
        std::assert_matches!(solve(input), Ok("No"));
    }
}
