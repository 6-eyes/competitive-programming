use std::{fmt::Display, io::stdin, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_line(&mut s).map_err(Error::Input)?;

    let n = s.trim().parse::<u32>().map_err(Error::Parse)?;
    // 1...2...3...4...5...6...7...8
    //  0.1 0.9 0.5 1.8 0.2 0.1 1.1
    // 1 -> 1 2 3
    // 1 2 3 4
    let mut l = 1;
    let mut r = 2;
    let mut ans = 0;

    while r <= n {
        println!("? {} {}", l, r);

        // read the input
        let mut s = String::new();
        stdin().read_line(&mut s).map_err(Error::Input)?;

        match s.as_str().trim() {
            // distance between l and r is atmost 1
            // continue exploring
            "Yes" => {
                r += 1;
            },
            // distance between l and r is more than 1
            "No" => {
                ans += r - l - 1;
                // increment l
                l += 1;
                // increment r if it is equal to l
                if l == r { r += 1; }
            },
            _ => return Err(Error::JudgeInput(s)),
        }
    }

    // r is now greater than n
    assert_eq!(r, n + 1);
    // this means that the elements between l and n can be added to ans
    // 1 + 2 + 3 + .. + n = n * (n + 1) / 2
    let e = n - l;
    ans += (e * (e + 1)) / 2;

    println!("! {ans}");

    Ok(())
}

#[derive(Debug)]
enum Error {
    Input(std::io::Error),
    Parse(ParseIntError),
    JudgeInput(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Input(e) => write!(f, "unable to fetch input: {e}"),
            Error::Parse(e) => write!(f, "error parsing element: {e}"),
            Error::JudgeInput(s) => write!(f, "invalid input from judge {s}"),
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
            Error::JudgeInput(_) => ExitCode::from(2),
            Error::Parse(_) => ExitCode::from(3),
        }
    }
}

impl std::error::Error for Error {}
