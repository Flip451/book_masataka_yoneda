use std::{collections::HashMap, fmt::Debug, io, str::FromStr};

fn main() {
    let nk = read_line::<usize>("Error at reading N, K");
    let (n, k) = (nk[0], nk[1]);
    let mut v = read_line::<usize>("Error at reading V");

    // // 以下の操作はやってもやらなくてもよいが、やった方が処理が速くなる
    // v.sort();
    // // どの二者も互いにもう一方の倍数でないようにvを取捨選択する
    // let mut v_prime = HashMap::<usize, bool>::new();
    // (0..k).for_each(|i| {
    //     v_prime.insert(i, true);
    // });
    // for i in 0..k {
    //     for j in (i + 1)..k {
    //         if v[j] % v[i] == 0 {
    //             v_prime.insert(j, false);
    //         }
    //     }
    // }
    // let v = v_prime
    //     .into_iter()
    //     .filter(|&(_, b)| b)
    //     .map(|(k, _)| v[k])
    //     .collect::<Vec<usize>>();
    // let k = v.iter().count();

    let mut count = 0;
    for i in 1..2_usize.pow(k as u32) {
        let mut v_contained = vec![];
        (0..k).for_each(|k| {
            if (i >> k) % 2 == 1 {
                v_contained.push(v[k]);
            }
        });
        let sign = (-1_isize).pow((v_contained.iter().count() % 2 + 1) as u32);

        // 最小公倍数
        let lcm = lcm_array(v_contained);

        // println!(
        //     "v_contained: {:?}, sign: {}, mul: {}",
        //     v_contained, sign, mul
        // );
        count += sign * ((n / lcm) as isize);
    }
    let output = count;
    println!("{}", output);
}

fn gcd(a: usize, b: usize) -> (usize, usize) {
    if a == 0 {
        (b, 0)
    } else if b == 0 {
        (a, 0)
    } else if a > b {
        gcd(a % b, b)
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    let gcd = gcd(a, b).0;
    a * b / gcd
}

fn lcm_array(a: Vec<usize>) -> usize {
    let mut output = a[0];
    for a_i in a.into_iter() {
        output = lcm(a_i, output);
    }
    output
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
