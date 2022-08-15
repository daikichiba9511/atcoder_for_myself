//! https://qiita.com/murai_mart/items/1a7a4d10abc0c3b5b53f
//! 例題２：|{k; 0 <= k <= n, k % 3 == 0}|
use proconio::input;

fn solve() {
    input! {
        s: String,
    }
    let s: Vec<char> = s.chars().collect();

    let digit = s.len();

    // 0以上n以下で3を含む数の個数を求める
    // dp[i][j][k] i: i桁目、j: 未満フラグ（０ならn未満）、k：３を含む（１ならすでに３を含んでいる
    let mut dp = vec![vec![vec![0; 2]; 2]; digit];
    let x0 = s[0] as usize - '0' as usize;

    if x0 > 3 {
        dp[0][0][0] = x0 - 1; // 先頭の桁がx0未満かつ３出ないものは0~(x0 - 1)で３を除いた(x0 - 1)個
        dp[0][0][1] = 1; // 先頭の桁がx0未満かつ３のものは一つ(3)
        dp[0][1][0] = 1; // 先頭の桁がx0かつ３でないものは一つ(x0)
        dp[0][1][1] = 0; // 先頭の桁がx0かつ３のものは存在しない(x0>3)
    } else if x0 == 3 {
        dp[0][0][0] = x0; // 先頭の桁が3未満かつ３でないものは0~2の三つ
        dp[0][0][1] = 0; // 先頭の桁が３未満かつ３のものは存在しない
        dp[0][1][0] = 0; //先頭の桁が３かつ３でないものは存在しない
        dp[0][1][1] = 1; // 先頭の桁が3かつ3であるものは一つ
    } else {
        // 0 <= x0 < 3
        dp[0][0][0] = x0; // 先頭の桁がx0未満かつ３でないものは0~(x0-1)のx0個(0~1)
        dp[0][0][1] = 0; // 先頭の桁がx0未満かつ３であるものは存在しない
        dp[0][1][0] = 1; // 先頭の桁がx0かつ３でないものは一つ(0,1,2)
        dp[0][1][1] = 0; // 先頭の桁がx0かつ3であるものは存在しない(x0<3)
    }

    for i in 1..digit {
        let x = s[i] as usize - '0' as usize;
        if x > 0 {
            // i桁目まででn未満が確定であり3を含まない
            // 1: 0~9で3以外ならok, 2: すでに3が含まれている, 3: 0~(x-1)で3意外ならok, 4:すでに3が含まれている
            dp[i][0][0] = dp[i - 1][0][0] * 9
                + dp[i - 1][0][1] * 0
                + dp[i - 1][1][0] * (x - 1)
                + dp[i - 1][1][1] * 0;
            // i桁目まででn未満が確定であり3を含む
            // 1: 3ならok, 2: 0~9のなんでもok, 3: 3ならok, 4: 0~(x-1)ならok
            dp[i][0][1] = dp[i - 1][0][0] * 1
                + dp[i - 1][0][1] * 10
                + dp[i - 1][1][0] * 1
                + dp[i - 1][1][1] * (x - 1);
            // i桁目まで一致して3を含まない
            // 1: すでにn未満, 2: すでにn未満, 3: xならok, 4: すでに3が含まれている
            dp[i][1][0] = dp[i - 1][0][0] * 0
                + dp[i - 1][0][1] * 0
                + dp[i - 1][1][0] * 1
                + dp[i - 1][1][1] * 0;
            // i桁目まで一致して3を含む
            // 1: すでにn未満, 2: すでにn未満, 3: xかつ3のものはない, 4: xならok
            dp[i][1][1] = dp[i - 1][0][0] * 0
                + dp[i - 1][0][1] * 0
                + dp[i - 1][1][0] * 0
                + dp[i - 1][1][1] * 1;
        } else if x == 3 {
            // i桁目まででn未満が確定であり3を含まない
            // 1: 0~9で3以外ならok, 2: すでに3が含まれている, 3: 0~2で3以外ならok, 4: すでに3が含まれている
            dp[i][0][0] = dp[i - 1][0][0] * 9
                + dp[i - 1][0][1] * 0
                + dp[i - 1][1][0] * x
                + dp[i - 1][1][1] * 0;
            // i桁目まででn未満が確定であり3を含む
            // 1: 3ならok, 2: 0~9のなんでもok, 3: 3未満かつ3のものは存在しない,4: 0~(x-1)ならok
            dp[i][0][1] = dp[i - 1][0][0] * 1
                + dp[i - 1][0][1] * 10
                + dp[i - 1][1][0] * 0
                + dp[i - 1][1][1] * x;
            // i桁目まで一致してi桁目が3であり3を含まない（そんなものはない
            dp[i][1][0] = 0;
            // i桁目が一致してi桁目が3であり3を含む
            // 1: すでにn未満, 2: すでにn未満, 3: 3ならok, 4: xならok
            dp[i][1][1] = dp[i - 1][0][0] * 0
                + dp[i - 1][0][1] * 0
                + dp[i - 1][1][0] * 1
                + dp[i - 1][1][1] * 1;
        } else {
            // 0 <= x < 3
            // i桁目まででn未満が確定であり3を含まない
            // 1: 0~9で3以外ならok, 2: すでに3が含まれている, 3: 0~(x-1)ならok, 4: すでに3が含まれている
            dp[i][0][0] = dp[i - 1][0][0] * 9
                + dp[i - 1][0][1] * 0
                + dp[i - 1][1][0] * x
                + dp[i - 1][1][1] * 0;
            // i桁目まででn未満が確定であり3を含む
            // 1: 3ならok, 2: 0~9のなんでもok, 3: 3未満かつ3のものは存在しない, 4: 0~(x-1)ならok
            dp[i][0][1] = dp[i - 1][0][0] * 1
                + dp[i - 1][0][1] * 10
                + dp[i - 1][1][0] * 0
                + dp[i - 1][1][1] * x;
            // i桁目まで一致して3を含まない
            // 1: すでにn未満, 2: すでにn未満, 3: xならok, 4: すでに3が含まれている
            dp[i][1][0] = dp[i - 1][0][0] * 0
                + dp[i - 1][0][1] * 0
                + dp[i - 1][1][0] * 1
                + dp[i - 1][1][1] * 0;
            // i桁目まで一致して3を含む
            // 1: すでにn未満, 2: すでにn未満,3: xかつ3のものはない, 4: xならok
            dp[i][1][1] = dp[i - 1][0][0] * 0
                + dp[i - 1][0][1] * 0
                + dp[i - 1][1][0] * 0
                + dp[i - 1][1][1] * 1;
        }
    }
    println!("{}", dp[digit - 1][0][1] + dp[digit - 1][1][1]);
}

fn main() {
    solve()
}