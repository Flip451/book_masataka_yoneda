use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let nk = read_line::<usize>("Error at reading N");
    let (_, k) = (nk[0], nk[1]);

    let a = read_line::<usize>("Error at reading A");

    let sum: usize = a.iter().sum();
    if sum <= k && sum % 2 == k % 2 {
        println!("Yes");
    } else {
        println!("No");
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
