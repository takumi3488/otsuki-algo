use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [[usize; 2]; n],
        expected: usize
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; w+1]; n+1];
    for i in 0..n {
        for j in 0..w+1 {
            if j >= wv[i][0] {
                dp[i+1][j] = max(dp[i+1][j], dp[i][j - wv[i][0]] + wv[i][1])
            }
            dp[i+1][j] = max(dp[i+1][j], dp[i][j])
        }
    }
    let res = dp[n][w];
    assert_eq!(res, expected);
}
