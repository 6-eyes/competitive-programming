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

use std::io::{Read, stdin};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// (x, y, c)
/// (p1, p2, ..., pn)
///
/// (0, 0, 0)
/// (4, 0, 1)
/// remaining elements: 4
/// (4, 0, 1)
/// x can accomodate 1 element, y can accomodate 2 elements
/// 
/// (4, 3, 1)
/// x can accomodate 1 element, y can accomodate (2 - 1) element
///
/// (6, 5, 4, 3, 2, 1)
/// (6, 0, 1)
/// (6, 1, 2)
///
/// (3, 6, 5, 2, 7, 8, 9, 1, 4)
/// (3, 0, 1)
/// (3, 6, 0) => more than 3 + more than 6 = 5 + 3 = 8
/// (6, 0, 1) => more than 6 + more than 0 = 3 + 7 = 10
/// (6, 0, 1)
/// (6, 0, 1) => 3 + 6 = 9
/// (6, 5, 1) => 3 + 3 = 6
/// (6, 0, 1)
/// ()
fn solve(input: &str) -> Result<usize, Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "2
2 3
";

        std::assert_matches!(solve(input), Ok(499122184));
    }

    #[test]
    fn test_2() {
        let input = "6
1 2 3 4 5 6
";

        std::assert_matches!(solve(input), Ok(499122250));
    }

    #[test]
    fn test_3() {
        let input = "9
3 1 4 1 5 9 2 6 5
";

        std::assert_matches!(solve(input), Ok(855638200));
    }
}
