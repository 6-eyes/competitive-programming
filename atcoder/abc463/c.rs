use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    println!("{}", solve(&s)?);

    Ok(())
}

/// Find the time using binary search?
/// Then find the max taka after that position
/// 0..4 --> 31
/// 4..5 --> 26
/// 5..9 --> 15
/// 9..MAX --> unreachable (no query geq 9)
/// Array with values (end time, max height)
fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();
    let n = iter.next().ok_or(Error::Iter)?.parse::<usize>()?;

    let mut data = Vec::new();
    for _ in 0..n {
        let (h, l) = (iter.next().ok_or(Error::Iter)?.parse::<u32>()?, iter.next().ok_or(Error::Iter)?.parse::<u32>()?);
        // pop while the takahashis are smaller than the given height
        while let Some((ph, _)) = data.last() && *ph <= h { data.pop(); }
        data.push((h, l));
    }

    let q = iter.next().ok_or(Error::Iter)?.parse::<usize>()?;
    let mut ans = String::new();

    for _ in 0..q {
        let t = iter.next().ok_or(Error::Iter)?.parse::<u32>()?;

        use std::cmp::Ordering::{Less, Greater};
        let i = data.binary_search_by(|(_, l)| match t.cmp(l) {
            Less => Greater,
            _ => Less,
        }).unwrap_err();

        use std::fmt::Write;
        writeln!(ans, "{}", data[i].0)?;
    }

    Ok(ans)
}

#[derive(Debug)]
enum Error {
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
            Error::Write(e) => write!(f, "unable to write to string: {e}"),
        }
    }
}

impl From<std::fmt::Error> for Error {
    fn from(value: std::fmt::Error) -> Self {
        Self::Write(value)
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
            Error::Write(_) => ExitCode::from(4),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = r#"4
31 4
26 5
3 5
15 9
4
3 4 5 6
"#;

        let output = r#"31
26
15
15
"#;
        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_2() {
        let input = r#"10
587 138
772 155
755 404
519 408
529 432
169 586
114 632
249 656
329 972
299 984
14
443 801 824 276 399 314 300 510 311 580 498 930 359 5
"#;

        let output = r#"329
329
329
755
755
755
755
329
755
329
329
329
755
772
"#;
        std::assert_matches!(solve(input), Ok(o) if o == output);
    }
}
