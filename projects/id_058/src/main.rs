use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let nxy = read_line::<isize>("Error at reading N, X, Y");
    let (n, x, y) = (nxy[0], nxy[1], nxy[2]);

    if x.abs() + y.abs() <= n && (x + y) % 2 == n % 2 {
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
