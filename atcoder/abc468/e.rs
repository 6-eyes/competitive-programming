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
use ac::{MOD, mod_pow, Error};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// a => a
/// ab => a + b + (a + b) / 2
/// abc => a + b + c + (a + b) / 2 + (b + c) / 2 + (a + b + c) / 3
/// abcd => a + (a + b) / 2 + (a + b + c) / 3 + (a + b + c + d) / 4 + b + (b + c) / 2 + (b + c + d) / 3 + c + (c + d) / 2 + d
///
/// ```tex
/// f(l, r) = \sum_{1 \leq l \leq r \leq n} (c(r) - c(l - 1)) / (r - l + 1)
///         = \sum_{l = 1}^{n} \sum_{r = l}^{n} (c(r) - c(l - 1)) / (r - l + 1)
///         = \sum_{l = 1}^{n} \sum_{r = l}^{n} c(r) / (r - l + 1) - \sum_{l = 1}^{n} \sum_{r = l}^{n} c(l - 1) / (r - l + 1)
///         = \sum_{r = 1}^{n} \sum_{l = r}^{n} c(r) / (r - l + 1) - \sum_{l = 1}^{n} \sum_{r = l}^{n} c(l - 1) / (r - l + 1)
///         = \sum_{r = 0}^{n - 1} b(0, r) * c(r) - \sum_{l = 1}^{n} b(n - l + 1) * c(l - 1)
///         = \sum_{i = 0}^{n} (b(i) - b(n - i)) * c(i)
/// ```
/// 2 3
/// c => 2 5
/// b(0) = 0
/// b(1) = 1
/// b(2) = 1 + 1/2
/// 
/// (b(0) - b(2)) * 0 + (b(1) - b(1)) * 2 + (b(2) - b(0)) * 5
/// 0 + 0 + 3 * 5 / 2
/// 15 / 2
fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let n = parse!(iter);

    let mut c = Vec::with_capacity(n + 1);
    c.push(0);

    for i in 0..n {
        let sum = (c[i] + parse!(iter)) % MOD;
        c.push(sum);
    }

    let mut b = Vec::with_capacity(n + 1);
    b.push(0);

    for i in 1..=n {
        let next = (mod_pow(i, MOD - 2, MOD) + b[i - 1]) % MOD;
        b.push(next);
    }

    let mut ans = 0;
    for i in 0..=n {
        // add MOD to b[i] because the diff can be negative
        let diff = (b[i] + MOD - b[n - i]) % MOD;
        ans += c[i] * diff;
        ans %= MOD;
    }

    Ok(ans)
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
