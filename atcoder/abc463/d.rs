use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    if let Some(v) = solve(&s)? { println!("{v}") } else { println!("-1") }

    Ok(())
}

fn solve(input: &str) -> Result<Option<usize>, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, k) = (iter.next().ok_or(Error::Iter)?.parse::<usize>()?, iter.next().ok_or(Error::Iter)?.parse::<usize>()?);
    let mut clothes = Vec::with_capacity(n);
    for _ in 0..n {
        let (l, r) = (iter.next().ok_or(Error::Iter)?.parse::<usize>()?, iter.next().ok_or(Error::Iter)?.parse::<usize>()?);
        clothes.push((l, r));
    }

    clothes.sort_unstable_by_key(|v| v.1);

    /// find the largest number in the range such that predicate is true
    fn bin(mut l: usize, mut r: usize, pred: impl Fn(usize) -> bool) -> Option<usize> {
        if ! pred(l) { return None; }

        while l < r {
            let mid = (l + r).div_ceil(2);
            if pred(mid) {
                // right range
                l = mid;
            }
            else {
                // left range
                r = mid - 1;
            }
        }

        Some(l)
    }

    // we need to find the maximum score.
    // if a score of `x` exists, then score of `x - 1` should also exist.
    // so, we can do a binary search on valid score range until we find a score using which we can make `k` sequences.
    // the max value of score can be 1e9
    const RANGE: usize = 10usize.pow(9);
    let i = bin(0, RANGE, |x| {
        // `last` represents the index from which the next cloth needs to be seen
        // last = right + x + 1

        // iterate over all clothes and select non overlaping ones
        let mut last = 0;
        clothes.iter().filter(|(l, r)| {
            if last <= *l {
                // update last
                last = r + x + 1;
                return true;
            }

            false
        }).count() >= k
    });

    // the result is zero indexed
    Ok(i.map(|i| i + 1))
}

#[derive(Debug)]
enum Error {
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

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = r#"6 3
1 12
2 7
5 9
9 13
10 18
15 20
"#;

        std::assert_matches!(solve(input), Ok(Some(2)));
    }

    #[test]
    fn test_2() {
        let input = r#"2 2
1 5
5 9
"#;

        std::assert_matches!(solve(input), Ok(None));
    }

    #[test]
    fn test_3() {
        let input = r#"20 5
169 748
329 586
529 972
432 520
408 587
138 250
114 656
299 632
755 984
404 772
155 506
832 854
353 465
374 387
384 567
555 631
428 951
104 705
405 530
102 258
"#;

        std::assert_matches!(solve(input), Ok(Some(35)));
    }
}
