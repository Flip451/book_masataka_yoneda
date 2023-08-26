use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let n = read_line::<usize>("Error at reading N")[0];
    let m: usize = 10_usize.pow(9) + 7;
    let a_1 = 1;
    let a_2 = 1;
    match n {
        1 => {
            println!("{}", 1);
        }
        2 => {
            println!("{}", 1);
        }
        n => {
            let mut before_last = a_1;
            let mut last = a_2;
            for _ in 3..=n {
                let new = (last + before_last) % m;
                before_last = last % m;
                last = new;
            }
            println!("{}", last);
        }
    }
}

// 標準入力をベクタに格納するための関数
fn read_line<T>(err: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(err);

    input
        .trim()
        .split_whitespace()
        .map(|w| w.trim().parse::<T>().expect(err))
        .collect::<Vec<T>>()
}
