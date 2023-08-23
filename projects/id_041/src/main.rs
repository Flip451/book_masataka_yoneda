use std::{fmt::Debug, io, str::FromStr};

fn main() {
    // 標準入力の読み込み
    let t = read_line::<usize>("Error at reading T")[0];
    let n = read_line::<usize>("Error at reading N")[0];
    let lrs: Vec<(usize, usize)> = (0..n)
        .map(|i| {
            let lr = read_line::<usize>(&format!("Error at reading L_{0}, R_{0}", i));
            let (l, r) = (lr[0], lr[1]);
            (l, r)
        })
        .collect();

    // 各シフト開始・終了時間における人数の増減
    let mut diffs: Vec<isize> = vec![0; t + 2];
    lrs.into_iter().for_each(|(l, r)| {
        diffs[l] += 1;
        diffs[r] -= 1;
    });

    let mut number = 0;
    for diff in &diffs[0..t] {
        number += diff;
        println!("{}", number);
    }
}

// 標準入力をベクタに読み込むための関数
fn read_line<T>(err: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(err);

    let output: Vec<T> = input
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|w| w.parse().expect(err))
        .collect();

    output
}
