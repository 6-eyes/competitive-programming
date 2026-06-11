use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, ops::RangeBounds, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
	let mut s = String::new();
	stdin().read_to_string(&mut s).map_err(Error::Input)?;

	println!("{}", solve(&s)?);

	Ok(())
}

macro_rules! parse {
	($iter: expr, $t:ty) => {
		$iter.next().ok_or(Error::Iter)?.parse::<$t>().map_err(Error::Parse)?
	};
	($iter: expr) => {
		$iter.next().ok_or(Error::Iter)?.parse::<usize>().map_err(Error::Parse)?
	};
}

fn solve(input: &str) -> Result<String, Error> {

	let mut iter = input.split_ascii_whitespace();
	
	let (n, q) = (parse!(iter), parse!(iter, u32));

	todo!()
}

/// Source:  https://github.com/rust-lang-ja/ac-library-rs/blob/master/src/fenwicktree.rs
#[derive(Debug)]
struct FenwickTree{
	data: Vec<u32>,
	init: u32,
}

impl FenwickTree {
	fn new(n: usize, init: u32) -> Self {
		Self{
			data: vec!{ init; n },
			init,
		}
	}

	fn add(&mut self, mut idx: usize, val: u32) {
		// fenwick trees are 1 indexed
		idx += 1;
		while idx <= self.data.len() {
			self.data[idx - 1] += val;
			idx += idx & idx.wrapping_neg();
		}
	}

	fn sum(&self, range: impl RangeBounds<usize>) -> u32 {
		let accum = |mut idx| {
			let mut sum = self.init;

			while idx > 0 {
				sum += self.data[idx - 1];
				idx &= idx - 1;
			}

			sum
		};

		use std::ops::Bound::*;
		let r = match range.end_bound() {
		    Included(r) => r + 1,
		    Excluded(r) => *r,
		    Unbounded => self.data.len(),
		};

		let l = match range.start_bound() {
		    Included(l) => *l,
		    Excluded(l) => l + 1,
		    Unbounded => return accum(r),
		};

		accum(r) - accum(l)
	}
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
	        Error::Iter => write!(f, "unable to fetch from iterator"),
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
	use crate::{solve, FenwickTree};
	use std::ops::Bound::*;

	#[test]
	fn test_1() {
		let input = r#"3 4
1 1
1 3
2 2
1 1
"#;

		let output = r#"3
6
4
5
"#.to_string();

		std::assert_matches!(solve(input), Ok(o) if o == output);
	}

	#[test]
	fn test_2() {
		let input = r#"300000 1
2 300000
"#;

		let output = r#"0
"#.to_string();

		std::assert_matches!(solve(input), Ok(o) if o == output);
	}

	#[test]
	fn test_fenwick() {
		let mut bit = FenwickTree::new(5, 0);

		for i in 0..5 {
			// fn add(index, value)
			bit.add(i, i as u32 + 1);
		}

		assert_eq!(bit.sum(0..5), 15);
		assert_eq!(bit.sum(0..4), 10);
		assert_eq!(bit.sum(1..3), 5);
		assert_eq!(bit.sum(..), 15);
		assert_eq!(bit.sum(..2), 3);
		assert_eq!(bit.sum(..=2), 6);
		assert_eq!(bit.sum(1..), 14);
		assert_eq!(bit.sum(1..=3), 9);
		assert_eq!(bit.sum((Excluded(0), Included(2))), 5);
	}
}
