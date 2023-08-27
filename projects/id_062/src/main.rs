use std::{collections::HashMap, fmt::Debug, hash::Hash, io, str::FromStr};

fn main() {
    let nk = read_line::<usize>("Error at reading N, K");
    let (n, k) = (nk[0], nk[1]);

    let mut teleporter = HashMap::new();
    let a = read_line::<usize>("Error at reading A");
    (1..=n).for_each(|i| {
        teleporter.insert(Town(i), Town(a[i - 1]));
    });

    let mut reports: HashMap<Town, TownReport> = HashMap::new();
    // 現在地
    let mut current_town = Town(1);
    // テレポート回数
    let mut index = 0;

    loop {
        // k 回移動したらテレポートの繰り返しは終了
        if index == k {
            break;
        }

        // 街の来訪回数と何回テレポートしたときに訪れた街かの記録
        let TownReport {
            count,
            index: last_index,
        } = reports
            .entry(current_town)
            .or_insert(TownReport { count: 0, index });

        // すでに訪れた街の場合、ループが起こるので最終到達地点を計算できる
        if *count > 0 {
            let loop_size = index - *last_index;
            // 最終地点に移動する処理を記述
            let result_index = *last_index + (k - *last_index) % loop_size;
            current_town = *reports
                .iter()
                .find(|(_, TownReport { count: _, index })| *index == result_index)
                .unwrap().0;
            break;
        }

        // 街の来訪回数を加算
        *count += 1;

        // テレポート処理（街の移動＋テレポート回数の加算）
        current_town = *teleporter.get(&current_town).unwrap();
        index = index + 1;
    }

    let Town(town) = current_town;
    println!("{}", town);
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Town(usize);

struct TownReport {
    count: usize,
    index: usize,
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
