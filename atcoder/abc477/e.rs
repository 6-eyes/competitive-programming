use std::{cmp::Reverse, collections::BinaryHeap, io::{Read, stdin}};
use ac::Error;

fn main() -> Result<(), Error> {
    let mut s = String::new();
    stdin().read_to_string(&mut s).map_err(Error::Input)?;

    print!("{}", solve(&s)?);

    Ok(())
}

/// S -> C -> T = 1 path
/// S -> T = 2 paths
fn solve(input: &str) -> Result<String, Error> {
    let mut iter = input.split_ascii_whitespace();
    let (n, q) = (parse!(iter), parse!(iter));

    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(parse!(iter));
    }

    let mut h = BinaryHeap::new();
    for i in 0..n {
        h.push(Reverse((parse!(iter), i)));
    }

    // prefix sum of a
    let pa = std::iter::once(0).chain(a.iter().scan(0, |acc, x| {
        *acc += x;
        Some(*acc)
    })).collect::<Vec<usize>>();

    let mut c = vec!{ usize::MAX; n + 1 };
    c[n] = 0;

    while let Some(Reverse((d, i))) = h.pop() {
        if d > c[i] { continue }
        c[i] = d;
        // add next nodes
        let (left, right) = ((i + n - 1) % n, (i + 1) % n);
        for (j, v) in [ (left, a[left]), (right, a[i]) ] {
            h.push(Reverse((d + v, j)));
        }
    }

    let mut ans = String::new();

    for _ in 0..q {
        let (s, t) = (parse!(iter), parse!(iter));
        let on_rim = if s > n || t > n {
            usize::MAX
        }
        else {
            let d1 = pa[s - 1].abs_diff(pa[t - 1]);
            d1.min(pa[n] - d1)
        };

        use std::fmt::Write;
        writeln!(ans, "{}", on_rim.min(c[s - 1] + c[t - 1]))?;
    }

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        let input = "5 3
1 3 4 2 5
5 2 4 3 7
2 5
5 6
1 4
";

        let output = "6
5
6
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }

    #[test]
    fn test_2() {
        let input = "10 12
95786828 55052989 7398452 90695030 73248844 120697567 77915140 68865651 15301338 68674005
690874817 21524935 244587369 529970100 736247510 757265588 993115119 576136368 21553212 219853538
5 6
5 8
2 6
3 7
1 3
3 4
8 11
1 10
2 6
6 8
8 9
5 8
";

        let output = "73248844
265090269
226395315
244911927
150839817
7398452
90418863
68674005
226395315
198612707
68865651
265090269
";

        std::assert_matches!(solve(input), Ok(o) if o == output);
    }
}

mod ac {
    #![allow(unused)]

    use std::{fmt::Display, num::ParseIntError, ops::Range, process::{ExitCode, Termination}};

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
    pub fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
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

    /// # Eratosthenes Sieve
    /// creates a vector of boolean representing primes if the value is true.
    pub fn eratosthenes_sieve(n: usize) -> Vec<bool> {
        let mut s = vec!{ true; n + 1 };
        s[0] = false;
        if n > 1 {
            s[1] = false;
        }

        let mut i = 2;
        while i * i < n {
            if s[i] {
                // start at i * i because smaller multiples are crossed by smaller primes like 2i, 3i, ..
                let mut j = i * i;
                while j <= n {
                    s[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
        s
    }

    /// # Segment Tree
    /// The query allows determining the max of the range. By default this returns a **MAX** value for the given query.
    #[derive(Debug)]
    pub struct SegmentTree<T: Ord + PartialOrd + Clone> {
        data: Vec<Option<T>>,
        n: usize,
    }

    impl<T: Ord + PartialOrd + Clone> SegmentTree<T> {
        pub fn new(s: impl AsRef<[T]>) -> Self {
            let s = s.as_ref();
            let n = s.len();

            let mut t = Self {
                data: vec!{ None; 2 * n.next_power_of_two() },
                n,
            };

            if n > 0 {
                t.new_inner(1, 0, n - 1, s);
            }

            t
        }

        fn new_inner(&mut self, node: usize, nl: usize, nr: usize, s: &[T]) {
            if nl == nr {
                self.data[node] = Some(s[nl].clone());
                return;
            }

            // determine midpoint
            let m = nl.midpoint(nr);

            // build left node
            self.new_inner(2 * node, nl, m, s);
            // build right node
            self.new_inner(2 * node + 1, m + 1, nr, s);

            // set max for the node
            self.data[node] = self.data[2 * node].as_ref().max(self.data[2 * node + 1].as_ref()).cloned();
        }

        /// queries the maximum for the given range.
        pub fn range_query(&self, r: Range<usize>) -> Option<&T> {
            // check bounds
            if r.is_empty() || r.end > self.n {
                return None;
            }

            self.query_inner(1, 0, self.n - 1, r.start, r.end - 1)
        }

        fn query_inner(&self, node: usize, nl: usize, nr: usize, l: usize, r: usize) -> Option<&T> {
            // check range out of bounds
            if nr < l || r < nl {
                return None;
            }

            if l <= nl && r >= nr {
                return self.data[node].as_ref();
            }

            let m = nr.midpoint(nl);
            let a = self.query_inner(2 * node, nl, m, l, r);
            let b = self.query_inner(2 * node + 1, m + 1, nr, l, r);

            a.max(b)
        }

        /// sets the value to the given index while returning the older value.
        /// returns `None` if the index is out of bounds.
        pub fn set(&mut self, index: usize, value: T) -> Option<T> {
            if index >= self.n {
                return None;
            }

            Some(self.set_inner(1, 0, self.n - 1, index, value))
        }

        fn set_inner(&mut self, node: usize, nl: usize, nr: usize, index: usize, value: T) -> T {
            // check if we reach a leaf
            if nl == nr {
                return self.data[node].replace(value).expect("leaves are always some after build");
            }

            let m = nl.midpoint(nr);
            let old = if index <= m {
                self.set_inner(2 * node, nl, m, index, value)
            }
            else {
                self.set_inner(2 * node + 1, m + 1, nr, index, value)
            };

            // recompute the node's max because the values might have changed
            self.data[node] = self.data[2 * node].as_ref().max(self.data[2 * node + 1].as_ref()).cloned();

            old
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::{assert_matches, cmp::Reverse};

        #[test]
        fn test_euclid() {
            assert_eq!(gcd(48, 36), 12);
            assert_eq!(gcd(1071, 462), 21);
            assert_eq!(gcd(5, 0), 5);
            assert_eq!(gcd(0, 5), 5);
            assert_eq!(gcd(usize::MAX, usize::MAX), usize::MAX);
        }

        #[test]
        fn test_st_query_max() {
            let s = &[3, 6, 5, 2, 4, 1];
            let st = SegmentTree::new(s);

            assert_matches!(st.range_query(0..6), Some(6));
            assert_matches!(st.range_query(2..5), Some(5));
            assert_matches!(st.range_query(5..6), Some(1));
            assert_matches!(st.range_query(3..6), Some(4));
            assert_matches!(st.range_query(2..4), Some(5));
        }

        #[test]
        fn test_st_query_min() {
            let s = &[3, 6, 5, 2, 4, 1];
            let rs = s.map(|v| Reverse(v));
            let st = SegmentTree::new(&rs);

            assert_matches!(st.range_query(0..6), Some(Reverse(1)));
            assert_matches!(st.range_query(0..3), Some(Reverse(3)));
            assert_matches!(st.range_query(2..5), Some(Reverse(2)));
            assert_matches!(st.range_query(1..3), Some(Reverse(5)));
            assert_matches!(st.range_query(1..2), Some(Reverse(6)));
        }

        #[test]
        fn test_st_query_out_of_bounds() {
            let s = &[3, 6, 5, 2, 4, 1];
            let st = SegmentTree::new(s);

            assert_matches!(st.range_query(5..10), None);
            assert_matches!(st.range_query(15..100), None);
        }

        #[test]
        fn test_st_set() {
            let s = &[3, 6, 5, 2, 4, 1];
            let mut st = SegmentTree::new(s);

            assert_matches!(st.set(5, 7), Some(1));
            assert_matches!(st.range_query(0..6), Some(7));

            assert_matches!(st.set(1, 1), Some(6));
            assert_matches!(st.range_query(1..4), Some(5));
        }

        #[test]
        fn test_st_set_out_of_bounds() {
            let s = &[3, 6, 5, 2, 4, 1];
            let mut st = SegmentTree::new(s);

            assert_matches!(st.set(7, 0), None);
        }
    }
}