use std::{collections::HashMap, fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
	let mut s = String::new();
	stdin().read_to_string(&mut s).map_err(Error::Input)?;

	println!("{}", solve(&s)?);

	Ok(())
}

macro_rules! parse {
	($iter:expr) => ($iter.next().ok_or(Error::Iter)?.parse::<usize>()?);
}

fn solve(input: &str) -> Result<usize, Error> {
	let mut iter = input.split_ascii_whitespace();
	let (n, k, m) = (parse!(iter), parse!(iter), parse!(iter));

	let mut map = HashMap::<usize, Vec<usize>>::new();

	// parse elements to the map
	for _ in 0..n {
		let (c, v) = (parse!(iter), parse!(iter));
		map.entry(c).and_modify(|a| a.push(v)).or_insert(vec!{ v });
	}

	// fetch the first element from the sorted vec into unique
	// fetch the remaining elements into the elements
	let mut unique = Vec::new();
	let mut elements = Vec::new();
	map.values_mut().for_each(|a| {
		a.sort_unstable();
		unique.push(a[a.len() - 1]);
		elements.extend_from_slice(&a[..a.len() - 1]);
	});

	// sort unique values and take m values
	unique.sort_unstable();
	let mut sum = (0..m).map(|_| unique.pop().expect("less unique elements present than requested")).sum();

	// append remaining values to elements and sort
	elements.append(&mut unique);
	elements.sort_unstable();

	// take remaining k - m values
	sum += (0..k - m).map(|_| elements.pop().expect("less number for elements found than requested")).sum::<usize>();

	Ok(sum)
}

#[derive(Debug)]
enum Error {
	Input(std::io::Error),
	Iter,
	Parse(ParseIntError),
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
    	Self::Parse(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	match self {
	        Error::Input(e) => write!(f, "unable to fetch input: {e}"),
	        Error::Iter => write!(f, "unable to fetch from iterator"),
	        Error::Parse(e) => write!(f, "error parsing element: {e}"),
	    }
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
		let input = r#"5 3 2
1 30
1 40
1 50
2 10
3 20
"#;

		std::assert_matches!(solve(input), Ok(110));
	}

	#[test]
	fn test_2() {
		let input = r#"5 3 3
1 30
1 40
1 50
2 10
3 20
"#;

		std::assert_matches!(solve(input), Ok(80));
	}

	#[test]
	fn test_3() {
		let input = r#"5 5 1
4 1000000000
5 1000000000
4 1000000000
5 1000000000
4 1000000000
"#;

		std::assert_matches!(solve(input), Ok(5000000000));
	}
}
