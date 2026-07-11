use std::{fmt::Display, io::stdin, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_line(&mut s).map_err(Error::Input)?;

    let n = s.trim().parse::<u16>().map_err(Error::Parse)?;
    let mut ans = 0;
    for (x, y) in (1..=n).flat_map(|x| (x + 1..=n).map(move |y| (x, y))) {
        println!("? {} {}", x, y);

        // read the input
        let mut s = String::new();
        stdin().read_line(&mut s).map_err(Error::Input)?;

        match s.as_str().trim() {
            "Yes" => ans += 1,
            "No" => {},
            _ => return Err(Error::JudgeInput(s)),
        }
    }

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
