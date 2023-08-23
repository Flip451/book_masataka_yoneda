use std::{fmt::Debug, io, str::FromStr};

fn main() {
    // 入力読み込み
    let nq: Vec<usize> = read_line("Error at reading N, Q");
    let n = nq[0];
    let q = nq[1];
    let a: Vec<usize> = read_line("Error at reading A");
    let lr: Vec<(usize, usize)> = (0..q)
        .map(|i| {
            let input: Vec<usize> = read_line(&format!("Error at reading N_{}, Q_{}", i, i));
            let lr = (input[0], input[1]);
            lr
        })
        .collect();

    // i 日目までの累積来場者数
    // let mut s = a
    //     .into_iter()
    //     .map(|a| vec![a])
    //     .reduce(|mut a, b| {
    //         a.push(a.last().unwrap() + b[0]);
    //         a
    //     })
    //     .unwrap();
    // s.insert(0, 0);
    let mut s = vec![0];
    let mut sum_tmp = 0;
    for a_i in a.into_iter() {
        sum_tmp += a_i;
        s.push(sum_tmp);
    }

    lr.iter().for_each(|(l, r)| {
        println!("{}", s[*r] - s[*l - 1]);
    })
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
