use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    io,
    str::FromStr,
};

fn main() {
    // ノード数とエッジ数を読み込み
    let nm: Vec<usize> = read_line("Error at readin N, M");
    let n = nm[0];
    let m = nm[1];

    // ノードの読み込み
    // 各ノードに隣接するノードを管理するベクタ
    let mut nodes: HashMap<Node<usize>, Vec<Node<usize>>> = HashMap::new();
    for i in 1..=n {
        nodes.insert(Node(i), vec![]);
    }

    // エッジの読み込み
    (1..=m).for_each(|i| {
        // 入力読み込み
        let ab_i: Vec<usize> = read_line(&format!("Error at reading A_{0}, B_{0}", i));
        let (a, b) = (ab_i[0], ab_i[1]);

        // 各ノードの隣接ノードを読み込み
        let handle_a = nodes.entry(Node(a)).or_insert(vec![]);
        handle_a.push(Node(b));
        let handle_b = nodes.entry(Node(b)).or_insert(vec![]);
        handle_b.push(Node(a));
    });

    // 足跡を記録するためのスタック
    let mut stack: Vec<&Node<usize>> = vec![];

    // 各ノードに到達したかを管理するハッシュマップ
    let mut visited: HashMap<&Node<usize>, bool> = HashMap::new();
    for (node, _) in nodes.iter() {
        visited.insert(node, false);
    }

    // 適当なノードから DFS（深さ優先探索）をスタート
    let first_node = nodes.keys().next().unwrap();
    stack.push(first_node);
    visited.insert(first_node, true);
    dfs(&nodes, &mut stack, &mut visited);

    let nodes_not_visited = visited.iter().find(|&(_, b)| !b);
    match nodes_not_visited {
        Some(_) => {
            println!("The graph is not connected.");
        }
        None => {
            println!("The graph is connected.");
        }
    };
}

#[derive(Eq, Hash, PartialEq)]
struct Node<T>(T);

fn dfs<'a, T>(
    nodes: &'a HashMap<Node<T>, Vec<Node<T>>>,
    stack: &mut Vec<&'a Node<T>>,
    visited: &mut HashMap<&'a Node<T>, bool>,
) where
    T: PartialEq,
    Node<T>: Hash + Eq,
{
    // スタックに中身が入っているか否かで分岐
    // 入っていなかったら dfs 終了
    match stack.last() {
        Some(current_node) => {
            // 現在の stack.last() と接続している中で未到達のノード .. (*1)
            let next_node = nodes.get(current_node).unwrap().iter()  // 現在のノードの隣接ノード
                .find(|node| !(visited.get(node).unwrap()));                // のうち未到達のノード
            // (*1) が存在すれば、そちらへ移動
            // (*1) が存在しなければ、一歩戻る
            match next_node {
                Some(next_node) => {
                    stack.push(next_node);
                    visited.insert(next_node, true);
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
