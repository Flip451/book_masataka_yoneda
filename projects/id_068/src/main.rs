use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let nk = read_line::<usize>("Error at reading N, K");
    let (n, k) = (nk[0], nk[1]);
    let v = read_line::<usize>("Error at reading V");

    let mut check = vec![false; n + 1];
    for v_k in v {
        if !check[v_k] {
            for i in 1..=n / v_k {
                check[i * v_k] = true;
            }
        }
    }
    let output = check.into_iter().filter(|&b| b).count();
    println!("{}", output);
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
