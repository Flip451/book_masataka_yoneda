use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    // 入力読み込み
    let nx: Vec<usize> = read_line("Error at reading N, X");
    let _ = nx[0];
    let x = nx[1];
    let mut a: Vec<usize> = read_line("Error at reading A");

    a.sort();

    let mut a_sub = &a[..];
    let output: &str;

    // 探索を実行
    loop {
        if a_sub.len() == 0 {
            output = "No";
            break;
        }
        if a_sub.len() == 1 {
            output = if a_sub[0] == x { "Yes" } else { "No" };
            break;
        }

        // 配列の中央の位置を求める
        let m = a_sub.len() / 2;

        // 中央の値が x に一致したらloopを抜ける
        // そうでなければ、配列をに分割し、x を含む可能性のあるほうに注目する
        a_sub = if a_sub[m] == x {
            output = "Yes";
            break;
        } else if a_sub[m] > x {
            &a_sub[..m]
        } else {
            &a_sub[(m + 1)..]
        }
    }

    println!("{}", output);
}

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
