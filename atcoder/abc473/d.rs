use std::io::{Read, stdin};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// a + 2b + 3c = k
fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, k) = (parse!(iter), parse!(iter));
    let mut ans = String::new();

    fn dfs(i: usize, sum: usize, seq: &mut [usize], k: usize, ans: &mut String) -> Result<(), Error> {
        // on last element
        let n = seq.len();
        if i == n - 1 {
            // print if the sum can be made equal to k
            // sum would never exceed k
            if (k - sum) % n == 0 {
                // print array
                seq[i] = (k - sum) / n;

                use std::fmt::Write;
                for e in seq {
                    write!(ans, "{e} ")?;
                }
                writeln!(ans)?;
            }

            return Ok(());
        }

        // sum would never exceed k
        for e in 0..=(k - sum) / (i + 1) {
            seq[i] = e;
            dfs(i + 1, sum + (i + 1) * e, seq, k, ans)?;
        }

        Ok(())
    }

    let mut seq = vec!{ 0; n };
    dfs(0, 0, &mut seq, k, &mut ans)?;

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "3 8
";

        let output = "0 1 2 
0 4 0 
1 2 1 
2 0 2 
2 3 0 
3 1 1 
4 2 0 
5 0 1 
6 1 0 
8 0 0 
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_2() {
        let input = "1 200000
";

        let output = "200000 
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_3() {
        let input = "8 9
";

        let output = "0 0 0 1 1 0 0 0 
0 0 1 0 0 1 0 0 
0 0 3 0 0 0 0 0 
0 1 0 0 0 0 1 0 
0 1 1 1 0 0 0 0 
0 2 0 0 1 0 0 0 
0 3 1 0 0 0 0 0 
1 0 0 0 0 0 0 1 
1 0 0 2 0 0 0 0 
1 0 1 0 1 0 0 0 
1 1 0 0 0 1 0 0 
1 1 2 0 0 0 0 0 
1 2 0 1 0 0 0 0 
1 4 0 0 0 0 0 0 
2 0 0 0 0 0 1 0 
2 0 1 1 0 0 0 0 
2 1 0 0 1 0 0 0 
2 2 1 0 0 0 0 0 
3 0 0 0 0 1 0 0 
3 0 2 0 0 0 0 0 
3 1 0 1 0 0 0 0 
3 3 0 0 0 0 0 0 
4 0 0 0 1 0 0 0 
4 1 1 0 0 0 0 0 
5 0 0 1 0 0 0 0 
5 2 0 0 0 0 0 0 
6 0 1 0 0 0 0 0 
7 1 0 0 0 0 0 0 
9 0 0 0 0 0 0 0 
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