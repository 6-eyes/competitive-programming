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

/// 2k + 1 splits
fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();

    let (n, k) = (parse!(iter, usize), parse!(iter, usize));

    let mut cards = Vec::with_capacity(n);
    for _ in 0..n {
        cards.push((parse!(iter, usize), parse!(iter, usize)));
    }

    let mut dp = vec!{ vec!{ 0 ; 2 * k + 1 } ; n + 1 };
    // n = 7, k = 2
    // 0       1       2.      3       4.      5       6
    // (2, 1), (6, 9), (3, 5), (9, 2), (4, 8), (7, 4), (5, 6)
    // 0, 1, 2, 3, 4, 5, 6, 7
    // maximum 5 chunks
    // chunk0 | chunk1 | chunk2 | chunk3 | chunk4
    // same | flipped | same | flipped | same
    // dp[7][5]
    //
    // 2 1 2 1 2 1 2
    // 8 11 8 11 8 11 8
    // ..so on
    for i in 0..n {
        for j in 0..2 * k + 1 {
            let cur = dp[i][j];

            // same chunk
            if let Some(v) = dp.get_mut(i + 1).and_then(|r| r.get_mut(j)) {
                let card = if j % 2 == 0 { cards[i].0 } else { cards[i].1 };
                *v = (cur + card).max(*v);
            }

            // next chunk
            if let Some(v) = dp.get_mut(i + 1).and_then(|r| r.get_mut(j + 1)) {
                let card = if j % 2 == 0 { cards[i].1 } else { cards[i].0 };
                *v = (cur + card).max(*v);
            }
        }
    }

    Ok(*dp[n].iter().max().expect("iterator empty"))
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
        let input = "7 2
        2 1
        6 9
        3 5
        9 2
        4 8
        7 4
        5 6
";

        std::assert_matches!(solve(input), Ok(45));
    }

    #[test]
    fn test_2() {
        let input = "5 6
        9 6
        3 2
        8 1
        7 5
        8 4
";

        std::assert_matches!(solve(input), Ok(35));
    }

    #[test]
    fn test_3() {
        let input = "9 1
        2 7
        9 4
        1 1
        6 1
        3 4
        8 9
        1 2
        7 5
        3 9
";

        std::assert_matches!(solve(input), Ok(47));
    }
}