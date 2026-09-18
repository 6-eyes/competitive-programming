use std::io::{Read, stdin};
use ac::{Error, eratosthenes_sieve};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<isize, Error> {
    let input = input.trim();
    let n = input.len();
    assert!(n > 0);
    let s = 10usize.pow((n - 1) as u32);
    let e = 10usize.pow(n as u32);

    let sieve = eratosthenes_sieve(e);

    fn pattern(s: impl ToString) -> Vec<usize> {
        let s = s.to_string();
        s.chars().map(|c| s.chars().position(|d| d == c).unwrap()).collect()
    }

    // unwrap safety: atleast self would match
    let given = pattern(input);
    // range: 10^{n - 1} to 10^{n}
    for i in sieve.into_iter().enumerate().skip(s).take(e - s).filter_map(|(i, v)| v.then_some(i)) {
        if given == pattern(i) {
            return Ok(i as isize);
        }
    }

    Ok(-1)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "motor
";

        std::assert_matches!(solve(input), Ok(10607));
    }

    #[test]
    fn test_2() {
        let input = "byebye
";

        std::assert_matches!(solve(input), Ok(-1));
    }

    #[test]
    fn test_3() {
        let input = "coconut
";

        std::assert_matches!(solve(input), Ok(1010237));
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
    pub fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
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

    /// # Eratosthenes Sieve
    /// creates a vector of boolean representing primes if the value is true.
    pub fn eratosthenes_sieve(n: usize) -> Vec<bool> {
        let mut s = vec!{ true; n + 1 };
        s[0] = false;
        if n > 1 {
            s[1] = false;
        }

        let mut i = 2;
        while i * i < n {
            if s[i] {
                // start at i * i because smaller multiples are crossed by smaller primes like 2i, 3i, ..
                let mut j = i * i;
                while j <= n {
                    s[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
        s
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_euclid() {
            assert_eq!(gcd(48, 36), 12);
            assert_eq!(gcd(1071, 462), 21);
            assert_eq!(gcd(5, 0), 5);
            assert_eq!(gcd(0, 5), 5);
            assert_eq!(gcd(usize::MAX, usize::MAX), usize::MAX);
        }
    }
}