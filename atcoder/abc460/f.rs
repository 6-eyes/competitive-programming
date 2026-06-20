use std::{collections::HashMap, fmt::Display, io::{Read, stdin}, num::ParseIntError, process::{ExitCode, Termination}};

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    let v = solve(&s)?;
    v.into_iter().for_each(|v| println!("{v}"));

    Ok(())
}

/// ## Example
/// ### Given
/// **Edges:**
/// - 1 -> 2
/// - 2 -> 3
/// - 2 -> 4
/// - 6 -> 7
/// - 3 -> 5
/// - 7 -> 3
///
/// ### Step 1
/// Create an *adjacency list*.
/// - \[1\] -> \[2\]
/// - \[2\] -> \[1, 3, 4\]
/// - \[3\] -> \[2, 5, 7\]
/// - \[4\] -> \[2\]
/// - \[5\] -> \[3\]
/// - \[6\] -> \[7\]
/// - \[7\] -> \[3, 6\]
/// ### Step 2
/// DFS from node 1. 
fn solve(input: &str) -> Result<Vec<usize>, Error> {
    let mut iter = input.lines();
    let n = iter.next().ok_or(Error::Iter)?.parse()?;

    let mut adjacency_list = vec!{ Vec::new(); n };
    let q = loop {
        let line = iter.next().ok_or(Error::Iter)?;
        match line.split_once(' ') {
            Some((u, v)) => {
                let (u, v) = (u.parse::<usize>()? - 1, v.parse::<usize>()? - 1);
                adjacency_list[u].push(v);
                adjacency_list[v].push(u);
            },
            None => break line.parse::<usize>()?,
        }
    };

    adjacency_list.iter().enumerate().for_each(|(k, v)| println!("{k} -> {v:?}"));

    let ans = Vec::with_capacity(q);

    let mut depth = vec!{ 0; n };
    let mut first = vec!{ 0; n };
    let mut euler = Vec::with_capacity(2 * n - 1);

    /// Recursively calculate the **depth**, **first** occurance of value `v`. Also update the euler sequence.
    fn dfs(adj: &Vec<Vec<usize>>, v: usize, parent: usize, d: usize, depth: &mut Vec<usize>, first: &mut Vec<usize>, euler: &mut Vec<usize>) {
        depth[v] = d;
        first[v] = euler.len();
        euler.push(v);

        for u in adj[v].iter().filter(|x| **x != parent) {
            dfs(adj, *u, v, d + 1, depth, first, euler);
            euler.push(v);
        }
    }

    dfs(&adjacency_list, 0, 0, 0, &mut depth, &mut first, &mut euler);

    println!("depth: {depth:?}");
    println!("first: {first:?}");
    println!("euler: {euler:?}");

    #[derive(Debug)]
    enum Node {
        /// A white node present at the leaf
        White,
        /// A black node present at the leaf representing index i
        Black(usize),
        Sum {
            // the left node (0..n)
            l: usize,
            // the right node (0..n)
            r: usize,
            len: usize,
        },
    }

    #[derive(Debug)]
    struct SegTree(Vec<Node>);

    impl SegTree {
        fn new() -> Self {
            todo!();
        }

        fn process_query(&mut self, i: usize) {
            fn update(data: &mut Vec<Node>, node_idx: usize, l: usize, r: usize, i: usize) {
                let Some(node) = data.get_mut(node_idx) else {
                    // index out of bounds
                    return;
                };

                // l == r == i
                if l == r {
                    // leaf node found. toggle.
                    *node = match node {
                        Node::White => Node::Black(i),
                        Node::Black(_) => Node::White,
                        _ => unreachable!("leaf node can never represent Sum variant"),
                    };

                    return;
                }

                let mid = (l + r) / 2;
                if i <= mid {
                    update(data, 2 * node_idx, l, mid, i);
                }
                else {
                    update(data, 2 * node_idx + 1, mid + 1, r, i);
                }

                // recompute this node
                todo!("merge");
            }

            let len = self.0.len();
            update(&mut self.0, 1, 1, len, i);
        }
    }

    for _ in 0..q {
        let query = iter.next().ok_or(Error::Iter)?.parse::<usize>()?;
    }

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
    use crate::solve;

    #[test]
    fn test_1() {
        let input = r#"7
1 2
2 3
2 4
6 7
3 5
7 3
9
1
4
2
6
3
1
1
4
6
"#;

        let output = [4, 3, 3, 2, 2, 3, 2, 3, 4];
        std::assert_matches!(solve(input), Ok(v) if v == output)
    }
}
