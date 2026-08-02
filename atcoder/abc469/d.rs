use std::{collections::HashSet, io::{Read, stdin}};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, m) = (parse!(iter), parse!(iter));

    let mut tournaments = HashSet::new();
    for _ in 0..m {
        let (x, y) = (parse!(iter), parse!(iter));
        tournaments.insert((x, y));
    }

    let Some(selected) = tournaments.iter().next().copied() else {
        return Ok(0);
    };

    // remove the selected value
    // assert!(tournaments.remove(&selected));

    let mut ans = HashSet::new();
    // if x == a, find the intersection in tournaments which doesn't have `a`
    for a in [selected.0, selected.1] {
        let mut maybe_intersection: Option<HashSet<usize>> = None;
        for (x, y) in &tournaments {
            if *x == a || *y == a {
                continue;
            }

            let Some(intersection) = maybe_intersection.as_mut() else {
                let mut set = HashSet::new();
                set.insert(*x);
                set.insert(*y);
                maybe_intersection = Some(set);
                continue;
            };

            // short circuit when the intersection becomes empty
            if intersection.is_empty() {
                break;
            }

            intersection.retain(|v| v == x || v == y);
        }

        if let Some(intersections) = maybe_intersection {
            for b in intersections {
                let (p, q) = if a < b { (a, b) } else { (b, a) };
                ans.insert((p, q));
            }
        }
        else {
            // all cards in tournament are valid
            for b in 1..=n {
                if b == a { continue; }
                let (p, q) = if a <= b { (a, b) } else { (b, a) };
                ans.insert((p, q));
            }
        }
    }
    
    Ok(ans.len())
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "5 5
1 2
3 4
1 3
2 3
2 5
";

        std::assert_matches!(solve(input), Ok(1));
    }

    #[test]
    fn test_2() {
        let input = "7 8
2 4
1 3
1 7
1 3
1 2
1 6
1 5
1 3
";

        std::assert_matches!(solve(input), Ok(2));
    }

    #[test]
    fn test_3() {
        let input = "5 8
1 2
2 4
1 3
1 3
1 2
1 2
1 5
1 2
";

        std::assert_matches!(solve(input), Ok(2));
    }

    #[test]
    fn test_4() {
        let input = "3 1
1 3
";

        std::assert_matches!(solve(input), Ok(3));
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