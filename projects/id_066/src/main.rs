use std::{fmt::Debug, io, str::FromStr};

fn main() {
    let nk = read_line::<isize>("Error at reading N");
    let (n, k) = (nk[0], nk[1]);

    let all = n.pow(3);
    let mut count = n;
    // for i in 1..=n {
    //     for j in 1..=n{
    //         for l in 1..=n {
    //             if (i - j).abs() < k && (j - l).abs() < k && (l - i).abs() < k {
    //                 count += 1;
    //             }
    //         }
    //     }
    // }
    for min in 1..=n {
        if min <= n - k + 1 {
            count += 3 * ((k - 1) * (k - 1) + (k - 1));
        } else {
            count += 3 * ((n - min) * (n - min) + (n - min));
        }
    }
    let output = all - count;
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
