use std::{fmt::Debug, io, str::FromStr};

// エラトステネスの篩
// N 以下の素数を見つけ出す
fn main() {
    // 入力受付(N を読み込み)
    let n = read_line::<usize>("Error at reading N")[0];

    // 〇 or (印なし) / ✕ を true/false で表す
    // 0 ~ N までの整数に対応する N+1 個の要素を持つ配列を準備
    // （0,1 は使用しないが、配列の番号と整数の値を一致させるために用意）
    let mut primes: Vec<bool> = vec![true; n + 1];

    // 操作は √N まで繰り返せば十分
    let sqrt_n = (n as f64).sqrt().floor() as usize;

    for i in 2..=sqrt_n {
        // まだ印が打たれていない値（true の値）に対してのみ操作を行う
        if primes[i] {
            // i 自身以外の i の倍数に✕印をつける
            for j in 2..=(n / i) {
                primes[i * j] = false;
            }
        }
    }
    
    // 素数の一覧を表示
    for i in 2..=n {
        if primes[i] {
            println!("{}", i);
        }
    }
}

// 標準入力をベクタに格納する関数
fn read_line<T>(err: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(err);

    let output = input
        .trim()
        .split_whitespace()
        .map(|w| w.trim().parse::<T>().expect(err))
        .collect();
    output
}
