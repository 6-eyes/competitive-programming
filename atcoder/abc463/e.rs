use std::{collections::BinaryHeap, fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

macro_rules! parse {
    ($iter: expr) => {
        $iter.next().ok_or(Error::Iter)?.parse::<usize>()?
    };
}

fn solve(input: &str) -> Result<String, Error> {
    /// This is also used in min heap.
    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    struct Element {
        /// the current node
        current: usize,
        /// the cost associated
        cost: usize,
    }

    impl Ord for Element {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for Element {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Element {
        #[inline]
        fn new(current: usize, cost: usize) -> Self {
            Self { current, cost }
        }
    }

    let mut iter = input.split_ascii_whitespace();
    let (n, m, y) = (parse!(iter), parse!(iter), parse!(iter));

    // adjacency list
    let mut edges = vec!{ Vec::new(); m + 2 * n + 1 };

    for _ in 0..m {
        // make it zero indexed
        let (u, v, t) = (parse!(iter) - 1, parse!(iter) - 1, parse!(iter));
        edges[u].push(Element::new(v, t));
        edges[v].push(Element::new(u, t));
    }

    for i in 0..n {
        let cost = parse!(iter);
        edges[i].push(Element::new(n, cost)); // from city i to start warp (index: n)
        edges[n + 1].push(Element::new(i, cost)); // from destination warp (index: n + 1) to city i
    }

    // between warps
    // each city is connected to its warp
    edges[n].push(Element::new(n + 1, y));

    let mut heap = BinaryHeap::<Element>::new();
    heap.push(Element::default());

    // the number of vertices are n + 2
    // n cities  + start warp + destination warp
    let mut time = vec!{ usize::MAX; n + 2 };
    while let Some(Element { current, cost }) = heap.pop() {
        if time[current] < cost { continue; }

        for &Element { current: next, cost: del } in edges[current].iter() {
            let new_cost = cost + del;
            if new_cost < time[next] {
                // update min time
                time[next] = new_cost;
                heap.push(Element::new(next, new_cost));
            }
        }
    }

    use std::fmt::Write;
    let mut ans = String::new();
    // print n - 1 elements
    for (i, t) in time.iter().take(n).skip(1).enumerate() {
        if i != 0 {
            write!(ans, " ")?;
        }
        write!(ans, "{t}")?;
    }

    writeln!(ans)?;

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
            Error::Write(e) => write!(f, "error writing to string: {e}"),
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
        let input = r#"7 7 3
1 2 1
1 3 6
2 3 4
3 5 8
3 7 4
4 5 2
4 7 9
3 1 4 1 5 9 2
"#;

        let output = r#"1 5 6 8 14 7
"#;

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_2() {
        let input = r#"2 0 1000000000
1000000000 1000000000
"#;

        let output = r#"3000000000
"#;

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_3() {
        let input = r#"12 20 873
2 7 940
6 9 444
6 11 809
7 8 786
9 10 468
7 10 234
6 10 660
4 12 939
8 10 896
1 11 953
8 10 818
4 8 967
3 9 724
6 7 929
3 4 948
1 3 999
10 11 724
7 10 338
1 8 967
1 12 733
581 978 950 629 583 729 554 712 438 930 774 279
"#;

        let output = r#"2432 999 1672 2037 1762 1753 967 1723 1677 953 733
"#;

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }
}
