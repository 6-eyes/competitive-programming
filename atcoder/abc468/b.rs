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
    let (m, d) = (parse!(iter, usize), parse!(iter, usize));

    let seq = iter.next().ok_or(Error::Iter)?.char_indices().filter_map(|(i, c)| match c {
        'G' => Some(i),
        '.' => None,
        _ => panic!("invalid input"),
    }).collect::<Vec<usize>>();

    if seq.is_empty() {
        return Ok(m);
    }

    let mut ans = 0;
    ans += seq[0].checked_sub(d).unwrap_or_default();

    ans += seq.windows(2).filter_map(|w| {
        (w[1] - w[0] - 1).checked_sub(2 * d)
    }).sum::<usize>();

    // G...
    ans += (m - seq[seq.len() - 1] - 1).checked_sub(d).unwrap_or_default();

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
        let input = "7 1
.G...GG
";

        std::assert_matches!(solve(input), Ok(1));
    }

    #[test]
    fn test_2() {
        let input = "6 5
......
";

        std::assert_matches!(solve(input), Ok(6));
    }

    #[test]
    fn test_3() {
        let input = "21 2
....G...GG.....G.....
";

        std::assert_matches!(solve(input), Ok(6));
    }
}
