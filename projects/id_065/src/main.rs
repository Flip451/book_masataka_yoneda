use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let hw = read_line::<usize>("Error at reading N");
    let (h, w) = (hw[0], hw[1]);

    if h == 1 || w == 1 {
        println!("{}", 1);
    } else if h % 2 == 0 {
        println!("{}", h * w / 2);
    } else {
        println!("{}", (h / 2) * w + (w / 2) + (w % 2));
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
