use std::{fmt::Debug, io, str::FromStr};

// {(x+y)/3}C(x-(x+y)/3) % mの計算
// 先に 1! % m, 2! % m, 3! % m , ..., (x+y)/3! % m の値を計算しておいて、それを流用する
fn main() {
    let m = 10_usize.pow(9) + 7;
    let xy = read_line::<usize>("Error at reading X, Y");
    let (x, y) = (xy[0], xy[1]);

    if (x + y) % 3 == 0 && x <= 2 * y && y <= 2 * x {
        let mut factorials = vec![1];
        for i in 1..=((x + y) / 3) {
            let fact_i = (factorials.last().unwrap() * (i % m)) % m;
            factorials.push(fact_i);
        }

        let fact_x_inv = mod_pow(factorials[(2 * x - y) / 3], m - 2, m);
        let fact_y_inv = mod_pow(factorials[(2 * y - x) / 3], m - 2, m);
        let output = factorials[(x + y) / 3];
        let output = output * fact_x_inv % m;
        let output = output * fact_y_inv % m;
        println!("{}", output);
    } else {
        println!("{}", 0)
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
