use std::io::{Read, stdin};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, m, k) = (parse!(iter), parse!(iter), parse!(iter));
    let mut calories = Vec::with_capacity(n);

    for _ in 0..n {
        let a = parse!(iter);
        calories.push(a);
    }

    let mut ans = String::new();
    let mut health = 0;

    for i in 0..n {
        if let Some(j) = i.checked_sub(m) {
            health -= calories[j];
        }

        use std::fmt::Write;
        if health + calories[i] <= k {
            writeln!(ans, "Yes")?;
            health += calories[i];
        }
        else {
            writeln!(ans, "No")?;
            calories[i] = 0;
        }
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "5 3 83
48 73 59 90 21
";

        let output = "Yes
No
No
No
Yes
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_2() {
        let input = "7 4 728
187 816 349 609 255 308 175
";

        let output = "Yes
No
Yes
No
Yes
No
Yes
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_3() {
        let input = "10 3 1368290936
216519459 804733999 297250023 775422599 287963235 999315644 354987425 974810607 653940822 117157941
";

        let output = "Yes
Yes
Yes
No
Yes
Yes
No
No
Yes
Yes
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