use core::fmt::Debug;
use std::{cmp, io, str::FromStr};

fn main() {
    // 入力読み込み
    let n: Vec<usize> = read_line("Error at reading N, W");
    let n = n[0];
    let a: Vec<usize> = read_line("Error at reading A");

    let mut s_last = a[0];
    let mut s_before_last = 0;

    for i in 1..n {
        let s = cmp::max(s_before_last + a[i], s_last);
        s_before_last = s_last;
        s_last = s;
    }
    println!("{:?}", s_last);
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
