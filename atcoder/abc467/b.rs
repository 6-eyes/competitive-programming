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

fn solve(input: &str) -> Result<u16, Error> {
    // 2 - 1 | 1 | 2
    // 6 - 3 | 3 | 3
    // 9 - 5 | 5 | 9
    // 9 + 3 + 2 - 1 - 3 - 5
    let mut iter = input.split_ascii_whitespace();

    let  n = parse!(iter, u8);

    let mut ans = 0;
    for _ in 0..n {
        let (a, b, s) = (parse!(iter, u16), parse!(iter, u16), iter.next().ok_or(Error::Iter)?);
        if s == "keep" {
            ans += b - a;
        }
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
        let input = "3
1 2 keep
3 6 take
5 9 keep
";

        std::assert_matches!(solve(input), Ok(5));
    }

    #[test]
    fn test_2() {
        let input = "8
36 49 take
38 73 keep
27 85 take
65 71 take
52 86 keep
48 60 keep
37 98 keep
5 38 keep
";

        std::assert_matches!(solve(input), Ok(175));
    }
}
