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

fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, m) = (parse!(iter, u8), parse!(iter, usize));
    let mut ans = vec!{ -1i8; m };

    for _ in 0..n {
        let (c, s) = (parse!(iter, usize) - 1, parse!(iter, i8));
        if ans[c] < s {
            ans[c] = s;
        }
    }

    let mut pp = ans.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" ");
    pp.push('\n');
    Ok(pp)
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
        let input = "4 5
        1 3
        2 10
        1 7
        4 9
";
        let output = "7 10 -1 9 -1
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_2() {
        let input = "5 5
        2 6
        5 12
        5 2
        5 9
        2 7
";
        let output = "-1 7 -1 -1 12
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }
}
