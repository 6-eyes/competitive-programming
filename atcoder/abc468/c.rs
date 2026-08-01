use std::{fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

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

/// Total combinations by `right` - Total combinations by `left`
///   i
/// 54213
/// 1* + 2* + 3* + 4* + (nums < 213)
/// 4 * 
/// find i = 3 such that numbers are less than 213
fn solve(input: &str) -> Result<usize, Error> {
    let mut iter = input.split_ascii_whitespace();
    let n = parse!(iter, usize);

    let mut left = Vec::with_capacity(n);
    for _ in 0..n {
        left.push(parse!(iter, usize));
    }

    let mut right = Vec::with_capacity(n);
    for _ in 0..n {
        right.push(parse!(iter, usize));
    }

    /// 3 1 2
    /// => 1 2 3, 1 3 2
    /// => 2 1 3, 2 3 1
    /// => 3 1 2
    ///
    /// 1 3 2
    /// => 1 2 3
    fn find_lower(nums: &[usize]) -> usize {
        let mut seen = vec!{ false; nums.len() };
        let mut ans = 0;
        // 4 1 5 7 2 3 6
        // 3 * 6! + 0 + 2 * 4! + 3 * 3! + 0 + 0 + 0
        for i in 0..nums.len() {
            let free_digits = nums[i] - 1 - seen.iter().take(nums[i] - 1).filter(|b| **b).count();
            let fact = (1..nums.len() - i).reduce(|acc, n| acc * n).unwrap_or(1);

            ans += free_digits * fact;
            seen[nums[i] - 1] = true;
        }

        ans
    }

    let ans = find_lower(&right).saturating_sub(find_lower(&left) + 1);

    Ok(ans)
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
        let input = "3
1 3 2
3 1 2
";

        std::assert_matches!(solve(input), Ok(2));
    }

    #[test]
    fn test_2() {
        let input = "5
5 4 2 1 3
5 1 2 3 4
";

        std::assert_matches!(solve(input), Ok(0));
    }

    #[test]
    fn test_3() {
        let input = "7
3 6 5 2 7 1 4
4 1 5 7 2 3 6
";

        std::assert_matches!(solve(input), Ok(223));
    }
}
