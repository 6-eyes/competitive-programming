use std::io::{Read, stdin};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let n = parse!(iter);
    assert!(n > 1);

    let mut notches = Vec::with_capacity(n);

    let mut sum = 0;
    for _ in 0..n {
        let l = parse!(iter);
        sum += l;
        notches.push(sum);
    }

    // (n[l - 1] - n[i]).abs_diff(n[i])
    let half = notches[n - 1] / 2;
    
    // find the middle length
    let i = notches.binary_search(&half).unwrap_or_else(|i| i);

    // diff between the lengths of the snapped sticks
    let diff = |i: usize| (notches[n - 1] - notches[i]).abs_diff(notches[i]);

    // check i and i - 1
    let mut ans = diff(i);
    if i > 0 {
        ans = ans.min(diff(i - 1));
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "4
5 2 3 8
";

        std::assert_matches!(solve(input), Ok(2));
    }

    #[test]
    fn test_2() {
        let input = "7
31 41 59 26 53 58 97
";

        std::assert_matches!(solve(input), Ok(51));
    }

    #[test]
    fn test_3() {
        let input = "10
67011 35764 33042 24098 63738 98760 17199 68579 21812 45408
";

        std::assert_matches!(solve(input), Ok(28105));
    }
}

mod ac {
    #![allow(unused)]

    use std::{fmt::Display, num::ParseIntError, process::{ExitCode, Termination}};

    /// atcoder prime const
    pub const MOD: usize = 998244353;

    #[derive(Debug)]
    pub enum Error {
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
    			Error::Write(e) => write!(f, "error writing to the string: {e}"),
            }
        }
    }

    impl From<ParseIntError> for Error {
        fn from(value: ParseIntError) -> Self {
            Self::Parse(value)
        }
    }

    impl From<std::fmt::Error> for Error {
    	fn from(value: std::fmt::Error) -> Self {
    		Self::Write(value)
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


    #[macro_export]
    macro_rules! parse {
        ($iter: expr) => {
            $iter.next().ok_or(Error::Iter)?.parse::<usize>()?
        };
        ($iter: expr, $t: ty) => {
            $iter.next().ok_or(Error::Iter)?.parse::<$t>()?
        };
    }

    /// Calculates the gcd/hcf of two numbers a and b
    pub fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    pub fn mod_pow(mut base: usize, mut exp: usize, m: usize) -> usize {
        let mut res = 1;
        base %= m;
        while exp > 0 {
            if exp & 1 == 1 {
                res = (res * base) % m;
            }

            base = (base * base) % m;
            exp >>= 1;
        }

        res
    }
}