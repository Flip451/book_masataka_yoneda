use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let hw = read_line::<usize>("Error at reading N");
    let (h, w) = (hw[0], hw[1]);

    // 入力行列
    let mut a: Vec<Vec<isize>> = Vec::new();
    // 出力行列
    let mut b: Vec<Vec<isize>> = Vec::new();

    // 水平方向の和
    let mut sum_h: Vec<isize> = Vec::new();
    // 垂直方向の和
    let mut sum_w: Vec<isize> = vec![0; w];

    (0..h).for_each(|h| {
        let a_h = read_line::<isize>(&format!("Error at reading A_{},*", h));

        // 水平方向の合計の計算
        sum_h.push(a_h.iter().sum::<isize>());

        // B_h,* の初期化（A_h,* の符号を変えたものを代入）
        // 垂直方向の合計の計算
        b.push(vec![]);
        (0..w).for_each(|w| {
            b[h].push(-a_h[w]);
            sum_w[w] += a_h[w];
        });

        a.push(a_h);
    });

    (0..h).for_each(|h| {
        (0..w).for_each(|w| {
            b[h][w] += sum_h[h] + sum_w[w];
            print!("{} ", b[h][w]);
        });
        println!();
    });

    // println!("sum_h: {:?}", sum_h);
    // println!("sum_w: {:?}", sum_w);

    // for b_h in b {
    //     for b_h_w in b_h {
    //         print!("{} ", b_h_w);
    //     }
    //     println!();
    // }
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
