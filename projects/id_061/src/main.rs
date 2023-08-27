use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: usize = read_line::<usize>("Error at reading N")[0];

    let mut tmp = n + 1;
    let mut counter = 0;
    while tmp > 0 {
        if tmp % 2 == 1 {
            counter += 1;
        }
        tmp = tmp >> 1;
    }
    if counter == 1 {
        println!("Second");
    } else {
        println!("First");
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
