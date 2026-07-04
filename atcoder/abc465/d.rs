use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

macro_rules! parse {
    ($iter: expr) => {
        $iter.next().ok_or(Error::Iter)?.parse::<usize>()?
    };
}

/// x / K = y => [x / K..x / K + 1)
/// y / K = x => [x * K..(x + 1) * K)
///
/// X = 842, Y = 180, K = 7
/// x = 842 => y = 180
/// x = 180, y = 120
/// x = 120, y = 25
/// x = 25, y = 17
/// x = 17, y = 3
/// x = 3, y = 2
/// x = 0, y = 0
fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();
    let n = parse!(iter);

    let mut ans = String::new();
    for _ in 0..n {
        let (mut x, mut y, k) = (parse!(iter), parse!(iter), parse!(iter));
        let mut d = 0;
        while x != y {
            if x < y {
                std::mem::swap(&mut x, &mut y);
            }
            x /= k;

            d += 1;
        }

        use std::fmt::Write;
        writeln!(ans, "{d}")?;
    }

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
        let input = "4
11 9 3
0 0 2
842 180 7
1948706013487601 48019760148910476 89014537
";
        let output = "2
0
7
5
";

        std::assert_matches!(solve(input), Ok(ans) if ans == output);
    }
}
