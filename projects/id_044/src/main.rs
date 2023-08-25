use std::{
    collections::{HashMap, VecDeque},
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

    // 各頂点の頂点１からの最短距離
    let mut dist: HashMap<&Node, Option<usize>> = HashMap::new();
    nodes.keys().for_each(|node| {
        dist.insert(node, None);
    });

    // BFS に使用するキュー
    let mut queue: VecDeque<&Node> = VecDeque::new();

    // 頂点１に対する初期化
    let (first_node, _) = nodes
        .iter()
        .find(|(node, _)| **node == Node(1))
        .expect("Node 1 is NOT found.");
    queue.push_back(first_node);
    dist.insert(first_node, Some(0));

    // BFS
    while queue.len() > 0 {
        let current_node = queue.pop_front().unwrap();
        // println!("current_node: {:?}", current_node);
        nodes[current_node].iter().for_each(|next_node| {
            // println!("next_node: {:?}", next_node);
            // println!("dist.get(next_node): {:?}", dist.get(next_node));
            if dist.get(next_node) == Some(&None) {
                // println!("called");
                let dist_current = dist.get(current_node).unwrap().unwrap();
                dist.insert(next_node, Some(dist_current + 1));
                queue.push_back(next_node);
            }
            // println!("queue: {:?}", queue);
        });
    }

    (1..=n).for_each(|i| {
        let output = match dist.get(&Node(i)) {
            Some(Some(x)) => x.to_string(),
            Some(None) => "-1".to_string(),
            _ => panic!()
        };
        println!("{}", output);
    })
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
