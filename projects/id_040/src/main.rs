use std::{fmt::Debug, io, str::FromStr};

fn main() {
    // 標準入力の読み込み
    let _ = read_line::<usize>("Error at reading N")[0];
    let mut a = read_line::<isize>("Error at reading A");
    let m = read_line::<usize>("Error at reading M")[0];
    let b: Vec<usize> = (0..m)
        .map(|i| {
            let b_i = read_line::<usize>(&format!("Error at reading M_{}", i))[0];
            b_i
        })
        .collect();

    // 駅 1 からの距離を求める
    let mut distances = vec![];
    a.insert(0, 0);
    let mut distance = 0;
    for a_i in a {
        distance += a_i;
        distances.push(distance);
    }

    let mut sum = 0;
    let mut current_distance = distances[b[0] - 1];
    for b_i in b {
        sum += (distances[b_i - 1] - current_distance).abs();
        current_distance = distances[b_i - 1];
    }

    println!("{}", sum);
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
