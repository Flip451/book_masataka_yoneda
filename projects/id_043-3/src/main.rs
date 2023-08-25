use std::{fmt::Debug, io, str::FromStr};

fn main() {
    // ノード数とエッジ数を読み込み
    let nm: Vec<usize> = read_line("Error at readin N, M");
    let n = nm[0];
    let m = nm[1];

    // ノードの読み込み
    // 各ノードに隣接するノードを管理するベクタ
    let mut nodes: Vec<Vec<usize>> = Vec::new();
    for _ in 0..=n {
        nodes.push(vec![]);
    }

    // エッジの読み込み
    (1..=m).for_each(|i| {
        // 入力読み込み
        let ab_i: Vec<usize> = read_line(&format!("Error at reading A_{0}, B_{0}", i));
        let (a, b) = (ab_i[0], ab_i[1]);

        // 各ノードの隣接ノードを読み込み
        nodes[a].push(b);
        nodes[b].push(a);
    });

    // 足跡を記録するためのスタック
    let mut stack: Vec<usize> = vec![];

    // 各ノードに到達したかを管理するハッシュマップ
    let mut visited: Vec<bool> = vec![false; n + 1];
    visited[0] = true;

    // 適当なノードから DFS（深さ優先探索）をスタート
    let first_node = 1;
    // for i in 2..n {
    //     let neighbors = nodes[first_node].len();
    //     let neighbors_i = nodes[i].len();
    //     first_node = if neighbors < neighbors_i {first_node} else {i};
    // }
    stack.push(first_node);
    visited[first_node] = true;
    dfs(&nodes, &mut stack, &mut visited);

    let is_all = visited.iter().all(|&b| b);
    let output = if is_all {
        "The graph is connected."
    } else {
        "The graph is not connected."
    };

    println!("{}", output);
}

fn dfs<'a>(nodes: &'a Vec<Vec<usize>>, stack: &mut Vec<usize>, visited: &mut Vec<bool>) {
    // スタックに中身が入っているか否かで分岐
    // 入っていなかったら dfs 終了
    match stack.last() {
        Some(current_node) => {
            // 現在の stack.last() と接続している中で未到達のノード .. (*1)
            let next_node = nodes[*current_node]
                .iter() // 現在のノードの隣接ノード
                .find(|&node| !(visited[*node])); // のうち未到達のノード

            // (*1) が存在すれば、そちらへ移動
            // (*1) が存在しなければ、一歩戻る
            match next_node {
                Some(next_node) => {
                    stack.push(*next_node);
                    visited[*next_node] = true;
                }
                None => {
                    stack.pop();
                }
            }
            dfs(nodes, stack, visited);
        }
        None => {
            return ();
        }
    }
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
