use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n: usize = read_line::<usize>("Error at reading N")[0];

    match n % 4 {
        1 => {
            println!("First")
        }
        2 => {
            println!("First")
        }
        3 => {
            println!("First")
        }
        0 => {
            println!("Second")
        }
        _ => {
            panic!("ありえないことが起こりましたわ！")
        }
    };
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
