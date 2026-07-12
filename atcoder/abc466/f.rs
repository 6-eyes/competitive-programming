use std::{collections::BinaryHeap, fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

macro_rules! parse {
    ($iter: expr, $t: ty) => {
        $iter.next().ok_or(Error::Iter)?.parse::<$t>()?
    };
}

/// 0 1 2 3 4 5 6
/// 0 1 2 3 4 0 1 <- mod 5
/// 0 1 0 1 0 0 1 <- mod 2
/// 0 1 0 1 0 0 1 <- mod 3
///
/// [0..a) * (n / a) + [0..(n % a))
/// (5, 1), (2, 1) <- mod 5
/// (2, 2), (1, 1)
///
/// case:
/// repeating, remaining
/// heap: (8, 5)   , (5, 1)
/// 
/// tx = (8, 5)
/// a = 3
/// => (3, floor(8 / 3) * 5), (2, 5)
/// => (3, 30), (2, 5)
///
/// heap: (5, 1), (3, 30), (2, 5)
///
/// case:
/// (8, 5), (8, 2)
fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();

    let t = parse!(iter, usize);

    let mut ans = Vec::<usize>::new();
    for _ in 0..t {
        let (n, x) = (parse!(iter, usize), parse!(iter, usize));
        let mut heap = BinaryHeap::new();
        heap.push((x + 1, 1));

        for _ in 0..n {
            let a = parse!(iter, usize);

            // if a is greater than the range (top_n), mod will not affect the range. therefore, we skip the remainng elements where tn <= a.
            while let Some(top_n) = heap.peek().map(|v| v.0) && top_n > a {
                // SAFETY: element guaranteed by the peek defined above
                let (top_n, mut freq) = heap.pop().unwrap();

                // pop similar elements
                while let Some((identical_n, _)) = heap.peek() && *identical_n == top_n {
                    // SAFETY: element guaranteed by the peek defined above
                    freq += heap.pop().unwrap().1;
                }

                heap.push((a, top_n / a * freq));
                // push elements
                if top_n % a != 0 {
                    // push remaining elements
                    heap.push((top_n % a, freq));
                }
            }
        }

        let mut a = 0;
        while let Some(v) = heap.pop().map(|v| v.1) { a += v; }
        ans.push(a - 1);
    }

    Ok(ans.into_iter().fold(String::new(), |mut acc, v| {
        use std::fmt::Write;
        writeln!(acc, "{v}").expect("error writing to string");
        acc
    }))
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
        let input = "4
3 7
5 2 3
9 31415
9 9 8 2 4 4 3 5 3
1 1000000000000000000
1
9 20260405
3141 5926 5358 9793 2384 6264 3383 2795 288
";

    let output = "4
17452
1000000000000000000
77403
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }
}