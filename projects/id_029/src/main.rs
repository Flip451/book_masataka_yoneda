use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    // 入力の読み込み
    let n: Vec<usize> = read_line("Error at reading N");
    let n = n[0];

    // 動的計画法を用いた方法
    let mut total_last = 1;
    let mut total_bofore_last = 1;
    
    for _ in 1..n {
        let new_total = total_last + total_bofore_last;
        total_bofore_last = total_last;
        total_last = new_total;
    }

    // // 一般項を用いた解法
    // let ap = (1_f64 + 5_f64.sqrt()) / 2_f64;
    // let am = (1_f64 - 5_f64.sqrt()) / 2_f64;
    // let total_last = (ap.powf((n + 1) as f64) - am.powf((n + 1) as f64)) / 5_f64.sqrt();

    // 出力
    println!("{:.0}", total_last);
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

#[cfg(test)]
mod tests {}
