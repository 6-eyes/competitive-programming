use std::{collections::BinaryHeap, io::{Read, stdin}};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// 1 2 3
///
/// 1/6 * 1/5 * 1/4 * 1/3 => p(6)
/// 1/6 * 1/5 * 1/4 * 1/3 => p(6)
fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (q, v) = (parse!(iter), parse!(iter, isize));

    let mut ans = String::new();
    let mut batteries = BinaryHeap::new();
    for _ in 0..q {
        let a = parse!(iter);
        if a == 1 {
            // type 1
            let (t, w) = (parse!(iter, isize), parse!(iter, isize));
            batteries.push(w - t);
        }
        else if a == 2 {
            // type 2
            let t = parse!(iter, isize);
            let charge = batteries.pop().map(|c| (c + t).clamp(0, v)).unwrap_or(-1);
            use std::fmt::Write;
            writeln!(ans, "{charge}")?;
        }
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "7 100
1 15 60
1 25 80
2 30
1 45 0
2 60
2 70
2 80
";

    let output = "85
100
25
-1
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_2() {
        let input = "20 380736236
1 21873985 256702097
2 86369729
1 114301317 288304981
1 147244640 305840435
2 150951976
1 331581391 50335458
1 352989552 47577202
1 400130024 345362760
2 458793150
2 509082216
1 591375600 197371572
1 617022014 101276068
1 679649471 310249627
1 796351653 268586022
1 825648347 129608152
2 908069704
2 921770319
1 949684819 372272469
1 971850999 335461408
2 986253026
";

    let output = "321197841
324955640
380736236
380736236
380736236
380736236
380736236
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
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