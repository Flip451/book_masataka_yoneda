use core::fmt::Debug;
use std::{cmp, io, str::FromStr};

fn main() {
    // 入力読み込み
    let nw: Vec<usize> = read_line("Error at reading N, W");
    let n = nw[0];
    let w_total_max = nw[1];

    // 入力読み込み
    let mut w: Vec<usize> = Vec::new();
    let mut v: Vec<usize> = Vec::new();
    for _ in 0..n {
        let wv = read_line("Error at readin w, v");
        let w_new = wv[0];
        let v_new = wv[1];
        w.push(w_new);
        v.push(v_new);
    }

    // 動的計画法
    let mut dp: Vec<Vec<Option<usize>>> = Vec::new();

    // item を何も含まない集合(空集合)に対する dp
    dp.push(vec![]);
    for w_total in 0..=w_total_max {
        dp[0].push(if w_total == 0 { Some(0) } else { None });
    }

    for item_id in 0..n {
        dp.push(vec![]);
        for w_total in 0..=w_total_max {
            let new_dp = {
                // i 番目の item まで含む集合を考える場合

                // i - 1 番目の item まで含む集合を考えた場合の dp をもとに計算する
                // 最初に item を何も含まない集合(空集合)に対する dp をdp には push してあるので dp[item_id - 1] ではなく dp[item_id] を参照すればよい
                let dp_last = &dp[item_id];

                // w_total - w_i >= 0 かどうかで条件分岐
                if w_total >= w[item_id] {
                    // w_total - w[item_id] >= 0 なら dp_last[w_total - w[item_id]] が存在する
                    // これを dp_last[w_total] と比較する
                    // そのために、これらの値が Some(...) か None かを判定して場合分けする
                    match (dp_last[w_total - w[item_id]], dp_last[w_total]) {
                        (Some(dp_last_lighter), Some(dp_last_same_wight)) => {
                            Some(cmp::max(dp_last_lighter + v[item_id], dp_last_same_wight))
                        }
                        (Some(dp_last_lighter), None) => Some(dp_last_lighter + v[item_id]),
                        (None, Some(dp_last_same_wight)) => Some(dp_last_same_wight),
                        (None, None) => None,
                    }
                } else {
                    // w_total - w[item_id] < 0 なら dp_last[w_total - w[item_id]] は存在しないので dp_last[w_total] を返す
                    dp_last[w_total]
                }
            };
            dp[item_id + 1].push(new_dp);
        }
    }

    println!("{:?}", dp[n].iter().max().unwrap().unwrap());
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
