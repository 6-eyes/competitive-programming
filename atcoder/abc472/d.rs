use std::{collections::{HashSet, VecDeque}, io::{Read, stdin}};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (h, w, k) = (parse!(iter), parse!(iter), parse!(iter));
    let mut bombs = HashSet::new();
    let mut bomb_rows = HashSet::new();
    let mut bomb_cols = HashSet::new();
    for i in 0..h {
        let l = iter.next().ok_or(Error::Iter)?;
        l.char_indices().for_each(|(j, c)| if c == '#' {
            bombs.insert((i, j));
            bomb_rows.insert(i);
            bomb_cols.insert(j);
        });
    }

    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    // insert all safe cells
    for i in (0..h).filter(|r| ! bomb_rows.contains(r)) {
        for j in (0..w).filter(|c| ! bomb_cols.contains(c)) {
            q.push_back((i, j, 0));
            seen.insert((i, j));
        }
    }

    let mut ans = 0;
    while let Some((x, y, d)) = q.pop_front() {
        ans += 1;
        if d >= k { continue; }

        const D: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for (dx, dy) in D {
            let Some(i) = x.checked_add_signed(dx).and_then(|v| (v < h).then_some(v)) else {
                continue;
            };

            let Some(j) = y.checked_add_signed(dy).and_then(|v| (v < w).then_some(v)) else {
                continue;
            };

            if bombs.contains(&(i, j)) || ! seen.insert((i, j)) {
                continue;
            }

            q.push_back((i, j, d + 1));
        }
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "3 3 1
#..
...
..#
";

        std::assert_matches!(solve(input), Ok(5));
    }

    #[test]
    fn test_2() {
        let input = "2 3 0
...
...
";

        std::assert_matches!(solve(input), Ok(6));
    }

    #[test]
    fn test_3() {
        let input = "5 7 2
..#....
..#....
.......
...#...
...#...
";

        std::assert_matches!(solve(input), Ok(29));
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