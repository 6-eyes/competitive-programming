use std::{collections::HashMap, io::{Read, stdin}};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, k) = (parse!(iter), parse!(iter));
    let mut ps = Vec::<usize>::with_capacity(n + 1);
    ps.push(0);
    for i in 0..n {
        let a = parse!(iter);
        ps.push((a + ps[i]) % k);
    }

    let mut ans = 0;

    let mut seen = HashMap::new();
    let mut r = 0;
    for (i, e) in ps.into_iter().enumerate() {
        // check if the entry is already seen
        seen.entry(e).and_modify(|p| {
            // check if the seen is after the last added entry index
            if *p >= r {
                ans += 1;
                // update the last added entry index
                r = i;
            }

            // update the value
            *p = i;
        }).or_insert(i);
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "6 10
6 8 2 2 6 4
";

        std::assert_matches!(solve(input), Ok(2));
    }

    #[test]
    fn test_2() {
        let input = "8 1
0 0 0 0 0 0 0 0
";

        std::assert_matches!(solve(input), Ok(8));
    }

    #[test]
    fn test_3() {
        let input = "30 8
5 0 4 2 7 3 2 3 2 4 0 1 4 0 4 1 7 5 2 5 0 3 6 6 2 3 2 2 4 2
";

        std::assert_matches!(solve(input), Ok(8));
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