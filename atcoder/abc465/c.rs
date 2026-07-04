use std::{collections::VecDeque, fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    println!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.lines();
    let n = iter.next().ok_or(Error::Iter)?.parse::<usize>()?;
    let mut seq = VecDeque::with_capacity(n);

    let mut xo = iter.next().ok_or(Error::Iter)?.chars();
    let mut push_back = true;

    for i in 1..=n {
        if push_back {
            seq.push_back(i);
        }
        else {
            seq.push_front(i);
        }

        if let Some('o') = xo.next() {
            push_back = !push_back;
        }
    }

    let items = if push_back {
        seq.into_iter().map(|v| v.to_string()).collect::<Vec<String>>()
    }
    else {
        seq.into_iter().rev().map(|v| v.to_string()).collect::<Vec<String>>()
    };

    Ok(items.join(" "))
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
        let input = "5
ooxoo
";
        let output = "5 2 1 3 4";

        std::assert_matches!(solve(input), Ok(ans) if ans == output);
    }

    #[test]
    fn test_2() {
        let input = "7
ooooooo
";
        let output = "7 5 3 1 2 4 6";

        std::assert_matches!(solve(input), Ok(ans) if ans == output);
    }

    #[test]
    fn test_3() {
        let input = "15
xooxoxoxoxoxxoo
";
        let output = "15 11 10 7 6 3 1 2 4 5 8 9 12 13 14";

        std::assert_matches!(solve(input), Ok(ans) if ans == output);
    }
}
