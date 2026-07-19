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

/// n = 10, m = 2
/// 0 0 0 1 1 0 1 0 1 0
///  0 1 0 1 0 1 0 1 0
///
/// **credits:** ぱるま(yt)
fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, _m) = (parse!(iter, usize), parse!(iter, u8));
    assert!(n > 0, "cannot operate on empty entry");

    // capture the array with sum of consecutive
    let mut a = Vec::with_capacity(n);
    for _ in 0..a.capacity() {
        a.push(parse!(iter, u8));
    }

    let mut b = Vec::with_capacity(n - 1);
    for _ in 0..b.capacity() {
        b.push(parse!(iter, u8));
    }

    // the final array can have two forms:
    // b => 1 0 0 1 1 0
    // possibility 1 => 1 0 0 0 1 0 0
    // possibility 2 => 0 1 1 1 0 1 1
    let p1 = {
        let mut p = Vec::with_capacity(n);
        p.push(1u8);
        for i in 0..n - 1 {
            p.push((p[i] + b[i]) % 2);
        }
        p
    };

    let p2 = {
        // complement of p1
        let mut p = p1.clone();
        p.iter_mut().for_each(|v| if *v == 1 { *v = 0 } else { *v = 1 });
        p
    };

    let diff1 = p1.into_iter().zip(&a).filter(|(p, a)| p != *a).count();
    let diff2 = p2.into_iter().zip(a).filter(|(p, a)| p != a).count();

    Ok(diff1.min(diff2))
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
        let input = "3 2
1 1 1
1 1
";

        std::assert_matches!(solve(input), Ok(1));
    }

    #[test]
    fn test_2() {
        let input = "2 2
1 1
0
";

        std::assert_matches!(solve(input), Ok(0));
    }

    #[test]
    fn test_3() {
        let input = "10 2
0 0 0 1 1 0 1 0 1 0
0 1 0 1 0 1 0 1 0
";

        std::assert_matches!(solve(input), Ok(4));
    }
}
