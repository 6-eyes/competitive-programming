use std::io::{Read, stdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = String::new();
    stdin().read_to_string(&mut s)?;

    let n = s.trim_end().parse::<usize>()?;
    println!("{}", solve(n));

    Ok(())
}

fn solve(n: usize) -> usize {
    const MOD: usize = 998244353;
    const MAX_B: usize = 14;

    // find divisors of n
    let mut divs: Vec<usize> = Vec::with_capacity(MAX_B + 1);
    let mut i = 1;
    while i * i <= n {
        if n.is_multiple_of(i) {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
        i += 1;
    }

    let max_len = divs.len().min(MAX_B);

    divs.sort_unstable();

    let div_idx = |d: usize| divs.partition_point(|&x| x < d);

    // The array dp[0][b][c] represents size b subsets with product divs[c]
    // The array dp[1][b][c] represents the sum of elements of b subsets with product divs[c]
    let mut dp = [[[0usize; 2304]; MAX_B + 1]; 2];

    dp[0][0][div_idx(1)] = 1;

    for d in &divs {
        // Number of elements in subset (b) is less than `divs.len`.
        // Since the smallest possible product of distinct positive integers is `1 x 2 x 3 x ... x b = b!`, and this should be less than `10^10`, **b can take values upto 14**.
        for b in (1..=max_len).rev() {
            // fot index which has multiple of d
            for c_i in (0..divs.len()).filter(|c_i| divs[*c_i].is_multiple_of(*d)) {
                let prev = div_idx(divs[c_i] / d);
                let count = dp[0][b - 1][prev];
                let sum = dp[1][b - 1][prev];
                dp[0][b][c_i] = (dp[0][b][c_i] + count) % MOD;
                dp[1][b][c_i] = (dp[1][b][c_i] + sum + count * d) % MOD;
            }
        }
    }

    let n_idx = div_idx(n);

    (1..MAX_B)
        .fold((0, 1), |(ans, mut fact), b| {
            fact *= b;
            ((ans + dp[1][b][n_idx] * fact) % MOD, fact)
        })
        .0
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_1() {
        assert_eq!(solve(8), 80);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(461), 1385);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve(100), 1702);
    }
}
