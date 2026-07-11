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

fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, m) = (parse!(iter, usize), parse!(iter, usize));

    let mut r_blacklist = vec!{ false; n };
    let mut c_blacklist = vec!{ false; n };
    let mut steps = Vec::with_capacity(m);

    for _ in 0..m {
        let step = (parse!(iter, usize), parse!(iter, usize));
        steps.push(step);
    }

    // implement in reverse
    let mut ans = 0;

    for (r, c) in steps.into_iter().rev() {
        if ! r_blacklist[r - 1] && ! c_blacklist[c - 1] {
            ans += 1;
        }

        r_blacklist[r - 1] = true;
        c_blacklist[c - 1] = true;
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
        let input = "3 6
        1 1
        1 2
        3 3
        3 2
        1 3
        1 3
";

        std::assert_matches!(solve(input), Ok(2));
    }

    #[test]
    fn test_2() {
        let input = "2 3
        1 2
        2 1
        1 1
";

        std::assert_matches!(solve(input), Ok(1));
    }
}
