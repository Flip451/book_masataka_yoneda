use std::{
    collections::HashMap,
    fmt::Debug,
    io,
    str::FromStr,
};

// 手順１：すべての頂点を白く塗る
// 手順２：キューに頂点１を追加。頂点１の距離を０に設定。頂点１を灰色に塗る
// 手順３：キューが空になるまで以下の操作を繰り返す：
//      a：キューの先頭の要素を取り出して一時変数 current_node に記録する
//      b：current_node に隣接するノード next のうち、白色のものに対し、dist[next] = dist[current_node] + 1; とする
//      c：キューに next を追加して next を灰色にぬる

fn main() {
    let nm: Vec<usize> = read_line("Error at reading N, M");
    let (n, m) = (nm[0], nm[1]);

    let mut nodes: HashMap<Node, Vec<Node>> = HashMap::new();
    (1..=n).for_each(|i| {
        nodes.insert(Node(i), vec![]);
    });
    (1..=m).for_each(|i| {
        let ab: Vec<usize> = read_line(&format!("Error at reading A_{0}, B_{0}", i));
        let (a, b) = (ab[0], ab[1]);
        let a_handle = nodes.entry(Node(a)).or_insert(vec![]);
        a_handle.push(Node(b));
        let b_handle = nodes.entry(Node(b)).or_insert(vec![]);
        b_handle.push(Node(a));
    });

    let mut count = 0;
    nodes.into_iter().for_each(|(node, neighbors)| {
        let Node(node_index) = node;
        let less_node_number = neighbors.into_iter().filter(|Node(neighbors_index)| *neighbors_index < node_index).collect::<Vec<Node>>().len();
        if less_node_number == 1 {
            count += 1;
        }
    });

    println!("{}", count);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Node(usize);

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
