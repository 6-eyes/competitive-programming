use std::io::{Read, stdin};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// 1 2 3  4  5  6
///  5 2 4  1  6
///  5 7 11 12 18
/// -7 -2 0 4 5 11
///
/// 1 2 3 4 5 6 7 8
fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, s, l) = (parse!(iter), parse!(iter), parse!(iter));

    assert!(n > 0 && s > 0);
    let mut a = Vec::with_capacity(n - 1);
    let mut sum = 0;
    for _ in 0..n - 1 {
        let an = parse!(iter);
        sum += an;
        a.push(sum);
    }

    let mut ans = 1;

    for right in s..=n {
        for left in 1..=s {
            // the minimum distance travelled between l and r is given by
            // min(2l - r, 2r - l)
            let g = |i: usize| i.checked_sub(2).map(|v| a[v]).unwrap_or_default();
            let w = g(s);
            let x = w - g(left);
            let y = g(right) - w;
            // for maximum possible towns the distance should be less than equal to 'l'
            if (2 * x + y).min(2 * y + x) <= l {
                ans = ans.max(right - left + 1);
            }
        }
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "6 3 10
5 2 4 1 6
";

        std::assert_matches!(solve(input), Ok(4));
    }

    #[test]
    fn test_2() {
        let input = "8 8 17
2 3 4 4 3 5 1
";

        std::assert_matches!(solve(input), Ok(6));
    }

    #[test]
    fn test_3() {
        let input = "2 1 1000000000000000000
10000
";

        std::assert_matches!(solve(input), Ok(2));
    }

    #[test]
    fn test_4() {
        let input = "9 6 28
5 4 9 2 3 6 1 4
";

        std::assert_matches!(solve(input), Ok(6));
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