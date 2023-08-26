use std::{fmt::Debug, io, str::FromStr};

// 繰り返し2乗法による (a^b) % m の計算
fn main() {
    let ab = read_line::<usize>("Error at reading N");
    let (a, b) = (ab[0], ab[1]);
    let m: usize = 10_usize.pow(9) + 7;
    let max = (b as f64).log2().floor() as usize;

    let mut a = a;
    let mut output = 1;
    for i in 0..=max {
        // if (b >> i) % 2 == 1 {
        if b & (1 << i) != 0 {
            output = (output % m) * (a % m);
        }
        a = (a % m) * (a % m);
    }
    output = output % m;
    println!("{}", output);
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
