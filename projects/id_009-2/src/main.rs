use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() -> io::Result<()> {
    let ns: Vec<usize> = read_line("Error at reading N, S");
    let n = ns[0];
    let s_max = ns[1];
    let a: Vec<usize> = read_line("Error at reading A");

    // 動的計画法で解く
    let mut dp: Vec<Vec<bool>> = Vec::new();

    // 行列の初期化
    dp.push(vec![]);
    for s in 0..=s_max {
        dp[0].push(s == 0);
    }

    for item_id in 0..n {
        dp.push(vec![]);
        for s in 0..=s_max {
            let new_dp = if s >= a[item_id] {
                dp[item_id][s] || dp[item_id][s - a[item_id]]
            } else {
                dp[item_id][s]
            };
            dp[item_id + 1].push(new_dp);
        }
    }

    let output = if dp[n][s_max] { "Yes" } else { "No" };
    println!("{}", output);

    Ok(())
}

// 標準入力をベクタに取り込むための関数
fn read_line<T>(err: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(err);

    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let mut elements: Vec<T> = Vec::new();

    for e in input.into_iter() {
        let new_elemnt: T = e.trim().parse::<T>().expect(err);
        elements.push(new_elemnt);
    }

    elements
}
