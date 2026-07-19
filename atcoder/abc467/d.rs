use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

macro_rules! parse {
    ($iter: expr) => {
        Point::try_from([$iter.next().ok_or(Error::Iter)?, $iter.next().ok_or(Error::Iter)?])?
    };
}

/// Following are the cases when the bisector should not coincide:
/// 1. lines joining these points are parallel
/// (q.1 - p.1) / (q.0 - p.0) == (s.1 - r.1) / (s.0 - r.0)
/// (q.1 - p.1) * (s.0 - r.0) == (s.1 - r.1) * (q.0 - p.0)
///
/// 2. bisector should not be perpendicular to a line
/// (q.1 - p.1) / (q.0 - p.0) * ((r.1 + s.1) / 2 - (p.1 + q.1) / 2) / ((r.0 + s.0) / 2 - (p.0 + q.0) / 2) != -1
/// (q.1 - p.1) * (r.1 + s.1 - p.1 - q.1) != (r.0 + s.0 - p.0 - q.0) * (p.0 - q.0)
fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();
    let n = iter.next().ok_or(Error::Iter)?.parse().map_err(Error::Parse)?;
    let mut ans = String::new();
    for _ in 0..n {
        #[derive(Debug)]
        struct Point {
            x: isize,
            y: isize,
        }

        impl TryFrom<[&str; 2]> for Point {
            type Error = Error;

            fn try_from(value: [&str; 2]) -> Result<Self, Self::Error> {
                let x = value[0].parse().map_err(Error::Parse)?;
                let y = value[1].parse().map_err(Error::Parse)?;

                Ok(Point { x, y })
            }
        }

        let (p, q, r, s) = (parse!(iter), parse!(iter), parse!(iter), parse!(iter));
        use std::fmt::Write;
        // parallel && not bisector coincide
        if (q.y - p.y) * (s.x - r.x) == (s.y - r.y) * (q.x - p.x) &&
            (q.y - p.y) * (r.y + s.y - p.y - q.y) != (r.x + s.x - p.x - q.x) * (p.x - q.x) {
            writeln!(ans, "No")?;
        }
        else {
            writeln!(ans, "Yes")?;
        }
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
            Error::Write(e) => write!(f, "unable to write to a string: {e}"),
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

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "3
2 0 1 1 -1 0 1 2
1 0 -1 0 0 1 0 -1
4 0 3 1 2 0 1 1
";

        let output = "Yes
Yes
No
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }
}
