use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

macro_rules! parse {
    ($iter: expr, $t: ty) => {
        $iter.next().ok_or(Error::Iter)?.parse::<$t>()?
    };
}

fn solve(input: &str) -> Result<u8, Error> {
    let mut iter = input.split_ascii_whitespace();
    let n = parse!(iter, usize);
    assert!(n > 2, "invalid input");

    let mut seq = Vec::with_capacity(n);
    for _ in 0..n {
        seq.push(parse!(iter, u8))
    }

    let mut i = 1;
    let mut ans = 0;
    while i <= n - 2 {
        if seq[i] > seq[i - 1] && seq[i] > seq[i + 1] {
            ans += 1;
            // skip the next one
            i += 1;
        }
        i += 1;
    }

    Ok(ans)
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
        let input = "6
        3 1 4 1 5 2
";

        std::assert_matches!(solve(input), Ok(2));
    }

    #[test]
    fn test_2() {
        let input = "5
        1 1 1 2 1
";

        std::assert_matches!(solve(input), Ok(1));
    }

    #[test]
    fn test_3() {
        let input = "10
        7 3 9 8 10 3 1 5 5 4
";

        std::assert_matches!(solve(input), Ok(2));
    }
}
