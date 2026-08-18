use std::io::{Read, stdin};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// 2 1 3 5 4, 2 1 3 5 4
/// 2 5 3 1 4, 4 1 3 5 2
fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();

    let (n, q) = (parse!(iter), parse!(iter));
    let mut p: [ Vec<usize> ; 2] = std::array::from_fn(|_| vec!{ 0; n });
    for i in 0..n {
        let pi = parse!(iter);
        p[0][i] = pi;
        p[1][pi - 1] = i + 1;
    }

    let mut c = 0;
    for _q in 0..q {
        let q = parse!(iter, u8);
        if q == 1 {
            let (x, y) = (parse!(iter), parse!(iter));
            let (i, j) = (p[c][x - 1], p[c][y - 1]);
            (p[c][x - 1], p[c][y - 1]) = (j, i);
            (p[1 - c][i - 1], p[1 - c][j - 1]) = (p[1 - c][j - 1], p[1 - c][i - 1]);
        }
        else if q == 2 {
            c ^= 1;
        }
    }

    let mut ans = p.into_iter().nth(c).unwrap().into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" ");
    ans.push('\n');

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "5 5
2 1 3 5 4
1 2 4
2
1 2 3
1 3 4
2
";

    let output = "4 5 2 1 3
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_2() {
        let input = "7 4
3 7 5 6 4 2 1
2
2
2
2
";

    let output = "3 7 5 6 4 2 1
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_3() {
        let input = "10 8
7 3 2 4 8 5 10 9 1 6
2
1 4 10
1 6 9
2
1 9 10
1 3 10
2
1 4 6
";

    let output = "3 10 2 8 6 7 1 5 9 4
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