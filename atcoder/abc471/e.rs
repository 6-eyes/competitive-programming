use std::io::{Read, stdin};
use ac::Error;

use crate::ac::{MOD, mod_pow};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// \sum{A_i}^2 = (n - 1)C(k - 1) \sum{A_i^2} + (n - 2)C(k - 2) \sum{A_iA_j}_{i != j}
/// \sum{A_i}^2 = (n - 1)C(k - 1) \sum{A_i^2} + (n - 2)C(k - 2) (\sum{A_i}^2 - \sum{A_i^2})
/// \sum{A_i}^2 = \sum{A_i^2} ((n - 1)C(k - 1) - (n - 2)C(k - 2)) + (n - 2)C(k - 2) \sum{A_i}^2
/// \sum{A_i}^2 = \sum{A_i^2} (n - 2)C(k - 2)((n - 1)/(k - 1) - 1) + (n - 2)C(k - 2) \sum{A_i}^2
/// \sum{A_i}^2 = \sum{A_i^2} (n - 2)C(k - 2)(n - k)/(k - 1) + (n - 2)C(k - 2) \sum{A_i}^2
/// \sum{A_i}^2 = (n - 2)C(k - 2)(\sum{A_i^2} (n - k)/(k - 1) + \sum{A_i}^2)
/// \sum{A_i}^2 = (n - 2)C(k - 2)(\sum{A_i^2} (n - k) + \sum{A_i}^2 (k - 1)) / (k - 1)
/// \sum{A_i}^2 = (n - 2)C(k - 2)(\sum{A_i^2} (n - k) + \sum{A_i}^2 (k - 1)) / (k - 1)
/// \sum{A_i}^2 = (n - 2)! / (k - 2)! / (n - k)! / (k - 1) (\sum{A_i^2} (n - k) + \sum{A_i}^2 (k - 1))
/// \sum{A_i}^2 = (n - 2)! / (k - 1)! / (n - k)! (\sum{A_i^2} (n - k) + \sum{A_i}^2 (k - 1))
///
/// 10, 10, 20, 20, 20
/// 3 * (1400) + 80^2
fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, k) = (parse!(iter), parse!(iter));

    if n == 1 {
        let a = parse!(iter) % MOD;
        return Ok(a * a % MOD);
    }

    // precompute factorials
    let mut fact = Vec::with_capacity(n + 1);
    fact.push(1);
    for i in 1..=n {
        fact.push((fact[i - 1] * i) % MOD);
    }

    let mut inv_fact = vec!{ 1; n + 1 };
    inv_fact[n] = mod_pow(fact[n], MOD - 2, MOD);
    for i in (0..n).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % MOD;
    }

    let mut sum_sq = 0;
    let mut sum = 0;
    for _ in 0..n {
        let a = parse!(iter) % MOD;
        sum = (sum + a) % MOD;
        sum_sq = (sum_sq + a * a % MOD) % MOD;
    }

    let coeff = fact[n - 2] * inv_fact[k - 1] % MOD * inv_fact[n - k] % MOD;
    let term1 = sum_sq * (n - k) % MOD;
    let term2 = sum * sum % MOD * (k - 1) % MOD;

    Ok(coeff * ((term1 + term2) % MOD) % MOD)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "3 2
1 10 100
";

        std::assert_matches!(solve(input), Ok(22422));
    }

    #[test]
    fn test_2() {
        let input = "5 2
10 10 20 20 20
";

        std::assert_matches!(solve(input), Ok(10600));
    }

    #[test]
    fn test_3() {
        let input = "2 1
998244353 998244353
";

        std::assert_matches!(solve(input), Ok(0));
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