use std::{fmt::Debug, io, str::FromStr};

fn main() {
    // 入力読み込み
    let nq: Vec<usize> = read_line("Error at readin N, Q");
    let n = nq[0];
    let q = nq[1];
    // lrxs の i 番目の要素 (l_i, r_i, x_i) は i 日目に 区画 l_i から 区画 r_i に x_i の量の雪が降ることを表す
    let lrxs: Vec<(usize, usize, usize)> = (0..q)
        .map(|i| {
            let lrx_i: Vec<usize> = read_line(&format!("Error at reading L_{0}, R_{0}, Q_{0}", i));
            let (l, r, q) = (lrx_i[0], lrx_i[1], lrx_i[2]);
            (l, r, q)
        })
        .collect();

    // 隣り合う区画同士の積雪量の差分を計算
    // diffs[i] は 区画 i と 区画 i + 1 の積雪量の差分
    // （diffs[0], diffs[n+1] はそれぞれ、区画外と区画１および区画Nとの差分を表す）
    let mut diffs: Vec<isize> = vec![0; n + 1];
    lrxs.into_iter().for_each(|(l, r, x)| {
        diffs[l - 1] += x as isize;
        diffs[r] -= x as isize;
    });

    // 区画外と区画１、区画外と区画Nの差分を除去
    diffs.pop();
    diffs.remove(0);

    // 差分を適切な出力にフォーマット
    let output = diffs
        .into_iter()
        .map(|diff| {
            if diff > 0 {
                "<"
            } else if diff == 0 {
                "="
            } else {
                ">"
            }
        })
        .collect::<Vec<&str>>()
        .join("");

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
