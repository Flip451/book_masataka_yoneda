use core::fmt::Debug;
use std::{io, str::FromStr, cmp};

fn main() {
    // 入力の読み込み
    let n: Vec<usize> = read_line("Error at reading N");
    let n = n[0];
    let a: Vec<isize> = read_line("Error at reading A");

    let mut cost_last = 0;
    let mut cost_before_last = 0;

    for i in 0..n {
        let diff_last: isize = (a[i] - a[cmp::max(i, 1) - 1]).abs();
        let diff_before_last: isize = (a[i] - a[cmp::max(i, 2) - 2]).abs();
        let new_cost = cmp::min(cost_last + diff_last, cost_before_last + diff_before_last);
        // (cost_before_last, cost_last) = (cost_last, new_cost);
        cost_before_last = cost_last;
        cost_last = new_cost;
    }

    // 出力
    println!("{}", cost_last);
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
mod tests {
}
