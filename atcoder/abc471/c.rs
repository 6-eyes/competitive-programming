use std::io::{Read, stdin};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// -1 -4 2 -11
/// -11 -4 -1, 2
/// 
fn solve(input: &str) -> Result<u32, Error> {
    let mut iter = input.split_ascii_whitespace();
    let n = parse!(iter);

    let mut po = Vec::new();
    let mut ne = Vec::new();

    for _ in 0..n {
        let a = parse!(iter, i32);
        if a < 0 { ne.push(a); }
        else { po.push(a); }
    }

    // sort
    ne.sort_unstable();
    po.sort_unstable();
    po.reverse();

    let mut c = 0;
    let mut ans = 0;

    while ! ne.is_empty() || ! po.is_empty() {
        if let Some(nt) = ne.pop_if(|nt| po.last().is_none_or(|&pt| (c - *nt).abs() <= (c - pt).abs())) {
            ans += (c - nt).abs() as u32;
            c = nt;
        }

        if let Some(pt) = po.pop_if(|pt| ne.last().is_none_or(|&nt| (c - *pt).abs() < (c - nt).abs())) {
            ans += (c - pt).abs() as u32;
            c = pt;
        }
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "4
-1 -4 2 -11
";

        std::assert_matches!(solve(input), Ok(23));
    }

    #[test]
    fn test_2() {
        let input = "10
1 2 3 4 5 -1 -2 -3 -4 -6
";

        std::assert_matches!(solve(input), Ok(17));
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