use std::{f64::consts::PI, fmt::Debug, io, str::FromStr};

fn main() {
    // 入力読み込み
    let inputs: Vec<f64> = read_line("Error at reading parmeters");

    // 変数への代入
    let a = inputs[0];
    let b = inputs[1];
    let h = inputs[2];
    let m = inputs[3];

    // 時計の針の間の角度
    let theta = 2_f64 * PI * ((h + m / 60_f64) / 12_f64 - m / 60_f64);

    // 余弦定理
    let output_2 = a.powf(2_f64) + b.powf(2_f64) - 2_f64 * a * b * theta.cos();

    println!("{}", output_2.sqrt());
}

// 標準入力をベクタに取り込むための関数
fn read_line<T>(err: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(err);

    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let mut elements: Vec<T> = Vec::new();

    for e in input.into_iter() {
        let new_elemnt: T = e.trim().parse::<T>().expect(err);
        elements.push(new_elemnt);
    }

    elements
}
