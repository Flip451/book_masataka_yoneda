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

    // 各ノードに到達したかを管理するハッシュマップ
    let mut visited: Vec<bool> = vec![false; n + 1];
    visited[0] = true;

    // 適当なノードから DFS（深さ優先探索）をスタート
    let first_node = 1;
    dfs(&nodes, first_node, &mut visited);

    let is_all = visited.iter().all(|&b| b);
    let output = if is_all {
        "The graph is connected."
    } else {
        "The graph is not connected."
    };

    println!("{}", output);
}

fn dfs<'a>(nodes: &'a Vec<Vec<usize>>, current_node: usize, visited: &mut Vec<bool>) {
    // 着地したノードに到達のチェックをつける
    visited[current_node] = true;
    // 隣接するノードのうち、未到達のノードに対し dfs を行う
    nodes[current_node].iter().for_each(|&next_node| {
        if !(visited[next_node]) {
            dfs(nodes, next_node, visited);
        }
    });
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
