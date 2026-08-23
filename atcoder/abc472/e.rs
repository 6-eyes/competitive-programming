use std::{collections::{HashSet, VecDeque}, io::{Read, stdin}};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();
    let t = parse!(iter);
    let mut ans = String::new();

    for _ in 0..t {
        let (n, m) = (parse!(iter), parse!(iter));
        let mut adj = vec!{ HashSet::new(); n };
        let mut edges = Vec::with_capacity(m);

        for _ in 0..m {
            let (a, b) = (parse!(iter) - 1, parse!(iter) - 1);
            edges.push((a, b));
            adj[a].insert(b);
            adj[b].insert(a);
        }

        let mut color = vec!{ false; n };
        let mut parent = vec!{ usize::MAX; n };
        let mut q = VecDeque::new();

        q.push_back(0);
        while let Some(s) = q.pop_front() {
            for &e in adj[s].iter() {
                if parent[e] == usize::MAX {
                    color[e] = ! color[s];
                    parent[e] = s;
                    q.push_back(e);
                }
            }
        }

        use std::fmt::Write;
        match edges.iter().find(|&&(a, b)| color[a] == color[b]) {
            Some(&(a, b)) => {
                // to root (0) from a remembering position of each node
                let mut pos = vec!{ usize::MAX; n };
                let mut path_a = vec!{ a };

                while *path_a.last().unwrap() != 0 {
                    path_a.push(parent[*path_a.last().unwrap()]);
                }

                // index the path
                // a -> 0
                // 0 -> len till root
                path_a.iter().enumerate().for_each(|(i, &v)| pos[v] = i);

                let mut path_b = vec!{ b };
                // walk to  root until we hit a's path
                while pos[*path_b.last().unwrap()] == usize::MAX {
                    path_b.push(parent[*path_b.last().unwrap()]);
                }

                // lca
                let lca = *path_b.last().unwrap();

                // cycle a -> .. -> lca -> .. -> b
                // this cycle is closed by edge (a, b)
                let cycle = path_a[..=pos[lca]].iter().chain(path_b[..path_b.len() - 1].iter().rev()).map(|v| (v + 1).to_string()).collect::<Vec<String>>();
                writeln!(ans, "{}", cycle.len())?;
                writeln!(ans, "{}", cycle.join(" "))?;
            },
            None => writeln!(ans, "-1")?,
        }
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "4
3 3
1 2
2 3
1 3
7 7
1 2
2 3
3 4
1 4
4 5
5 6
6 7
5 5
1 2
2 3
3 4
4 5
1 5
9 10
1 2
2 3
3 4
4 5
1 5
6 7
7 8
8 9
6 9
1 6
";

        let output = "3
2 1 3
-1
5
3 2 1 5 4
5
3 2 1 5 4
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }
}

mod ac {
    #![allow(unused)]

    use std::{fmt::Display, num::ParseIntError, process::{ExitCode, Termination}};

    /// atcoder prime const
    pub const MOD: usize = 998244353;

    #[derive(Debug)]
    pub enum Error {
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
    			Error::Write(e) => write!(f, "error writing to the string: {e}"),
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


    #[macro_export]
    macro_rules! parse {
        ($iter: expr) => {
            $iter.next().ok_or(Error::Iter)?.parse::<usize>()?
        };
        ($iter: expr, $t: ty) => {
            $iter.next().ok_or(Error::Iter)?.parse::<$t>()?
        };
    }

    /// Calculates the gcd/hcf of two numbers a and b
    pub fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    pub fn mod_pow(mut base: usize, mut exp: usize, m: usize) -> usize {
        let mut res = 1;
        base %= m;
        while exp > 0 {
            if exp & 1 == 1 {
                res = (res * base) % m;
            }

            base = (base * base) % m;
            exp >>= 1;
        }

        res
    }
}