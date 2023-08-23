use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n = read_line::<usize>("Error at reading N")[0];

    let mut f = vec![0_usize; n + 1];
    for i in 1..=n {
        for j in 1..=(n / i) {
            f[i * j] += i * j;
        }
    }
    let output: usize = f.iter().sum();
    println!("{}", output)
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
