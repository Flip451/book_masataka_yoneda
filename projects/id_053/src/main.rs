use std::{fmt::Debug, io, str::FromStr};

fn main() {
    const M: usize = 1000000007_usize;
    const D: usize = (M - 1) / 2;
    let n = read_line::<usize>("Error at reading X, Y")[0];
    let (q, r) = (n / D, n % D);

    let inv_3 = mod_pow(3, M - 2, M);
    let sum_last: usize = ((mod_pow(4, D, M) - 1) * inv_3) % M;
    let sum_r: usize = ((mod_pow(4, r + 1, M) - 1) * inv_3) % M;
    let output = ((q * sum_last) % M + sum_r) % M;
    // let output = ((mod_pow(4, n + 1, M) - 1) * inv_3) % M;
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

// a^b % m を求める関数
fn mod_pow(a: usize, b: usize, m: usize) -> usize {
    let max = (b as f64).log2().floor() as usize;
    let mut a = a;
    let mut output = 1;
    for i in 0..=max {
        if (b >> i) % 2 == 1 {
            output = (output % m) * (a % m);
        }
        a = (a % m) * (a % m);
    }
    output % m
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mod_pow() {
        assert_eq!(
            934801994,
            super::mod_pow(97, 998244353, 10_usize.pow(9) + 7)
        );
    }
}
