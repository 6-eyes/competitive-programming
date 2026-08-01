use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// `ababa`
/// ### Odd
/// a
/// b
/// aba
/// a
/// bab
/// ababa
/// b
/// aba
/// a
fn solve(input: &str) -> Result<usize, Error> {
    let input = input.trim().chars().collect::<Vec<char>>();

    let mut ans = 0;
    for n in 0..2 {
        for i in 0..input.len() {
            let (mut l, mut r) = (i, i + n);
            let mut changed = false;

            while let Some(rv) = input.get(r) {
                if &input[l] != rv {
                    if changed {
                        break;
                    }
                    changed = true;
                }

                ans += 1;

                // check bounds for left
                if l == 0 {
                    break;
                }

                // update counter
                l -= 1;
                r += 1;
            }
        }
    }

    Ok(ans)
}

#[derive(Debug)]
enum Error {
    Input(std::io::Error),
    Parse(ParseIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Input(e) => write!(f, "unable to fetch input: {e}"),
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
        let input = "ababa
";

        std::assert_matches!(solve(input), Ok(13));
    }

    #[test]
    fn test_2() {
        let input = "atcoder
";

        std::assert_matches!(solve(input), Ok(18));
    }

    #[test]
    fn test_3() {
        let input = "abccbacbacb
";

        std::assert_matches!(solve(input), Ok(40));
    }
}
