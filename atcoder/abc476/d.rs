use std::io::{Read, stdin};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// total money: y * k + x
fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, m, k) = (parse!(iter), parse!(iter), parse!(iter));
    let (x, y) = (parse!(iter), parse!(iter));

    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let s = parse!(iter);
        a.push(s);
    }

    a.sort_unstable();
    // create cumulative
    let mut ac = Vec::with_capacity(n + 1);
    ac.push(0);
    for i in 0..n {
        ac.push(ac[i] + a[i]);
    }

    let mut b = Vec::with_capacity(m);
    for _ in 0..m {
        let s = parse!(iter);
        b.push(s);
    }

    b.sort_unstable();

    #[derive(Debug, Default)]
    struct CumulativeDrink {
        k: usize,
        price: usize,
    }

    // number of K-currency required for an item
    // cumulative for K
    let mut bc = Vec::with_capacity(m + 1);
    bc.push(CumulativeDrink::default());
    for i in 0..m {
        let drink = CumulativeDrink {
            k: bc[i].k + b[i].div_ceil(k),
            price: bc[i].price + b[i],
        };
        bc.push(drink);
    }

    // strategy: exhaustively buy drinks first and then buy desserts with the remaining amount
    // println!("ac: {ac:?}");
    let mut ans = 0;
    for (ck, dc) in bc.into_iter().enumerate() {
        // println!("buying {ck} drinks, cum: {dc:?}, k: {k}");
        // if cumulative K-currency exceeds available currency, that means we cannot buy more drinks.
        if dc.k > y { break; }
        // figure out remaining amount.
        let r = x + y * k - dc.price;
        // buy desserts
        let c1 = ac.partition_point(|v| *v <= r) - 1;
        // println!("money: {r}, num desserts: {c1}, items: {}", ck + c1);
        ans = ans.max(ck + c1);
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "2 3 10
50 6
22 30
20 12 24
";

        std::assert_matches!(solve(input), Ok(4));
    }

    #[test]
    fn test_2() {
        let input = "1 7 67
677677677766666 0
777666777
20 12 24 67 67 67 67
";

        std::assert_matches!(solve(input), Ok(1));
    }

    #[test]
    fn test_3() {
        let input = "20 20 30
605776135 133105105
97363214 218434035 697895427 109255624 299037330 227873982 195540071 411713803 828357845 244535208 138059186 639510883 39844882 707397687 371274487 696536603 351588202 319490007 47121612 87169661
32256972 567982330 554885983 299718223 443859449 687952877 264684780 666659381 576335424 941894234 406248934 321334900 423472560 863738035 213143887 384834384 468161291 673106162 164648316 15903323
";

        std::assert_matches!(solve(input), Ok(22));
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