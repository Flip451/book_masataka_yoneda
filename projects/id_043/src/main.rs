use std::{collections::HashMap, fmt::Debug, hash::Hash, io, str::FromStr};

fn main() {
    // 入力読み込み
    let nm: Vec<usize> = read_line("Error at readin N, M");
    let n = nm[0];
    let m = nm[1];
    // ノードの一覧
    let nodes: Vec<Node<usize>> = (1..=n).map(|i| Node(i)).collect();

    let edges: Vec<(&Node<usize>, &Node<usize>)> = (0..m)
        .map(|i| {
            let ab_i: Vec<usize> = read_line(&format!("Error at reading A_{0}, B_{0}", i));
            let (a, b) = (&nodes[ab_i[0] - 1], &nodes[ab_i[1] - 1]);
            (a, b)
        })
        .collect();

    // 足跡を記録するためのスタック
    let mut stack: Vec<&Node<usize>> = vec![];
    // 各ノードが到達済みかを管理するためのハッシュマップを作成
    let mut visited: HashMap<&Node<usize>, bool> = HashMap::new();
    for i in 0..n {
        visited.insert(&nodes[i], false);
    }

    // 1 番ノードから DFS（深さ優先探索）をスタート
    stack.push(&nodes[0]);
    visited.insert(&nodes[0], true);
    dfs(&mut visited, &mut stack, &edges);

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

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Node<T>(T);

fn dfs<'a, T>(
    graph_visited: &mut HashMap<&'a Node<T>, bool>,
    stack: &mut Vec<&'a Node<T>>,
    edges: & Vec<(&'a Node<T>, &'a Node<T>)>,
) where
    T: PartialEq,
    Node<T>: Eq + Hash,
    // Node<T>: FromIterator,
{
    // スタックに中身が入っているか否かで分岐
    // 入っていなかったら dfs 終了
    match stack.last() {
        Some(current_node) => {
            // 現在の stack.last() と接続している中で未到達のノード .. (*1)
            let next_node = graph_visited
                .iter()
                .filter(|&(_, b)| !b) // 未到達ノードに限定
                .find(|(n, _)| {
                    // 未到達ノードと現在のノードを結ぶエッジがあるか否か
                    let edge = edges.iter().find(|(n1, n2)| {
                        (n1, n2) == (current_node, n) || (n2, n1) == (current_node, n)
                    });
                    match edge {
                        Some(_) => true,
                        None => false,
                    }
                });

            // (*1) が存在する場合は、stack にそのノードをpushして、そのノードに到達のチェックをつける
            // (*1) が存在しない場合は、stack を pop
            match next_node {
                Some((n, _)) => {
                    stack.push(n);
                    graph_visited.insert(n, true);
                }
                None => {
                    stack.pop();
                }
            };
            dfs(graph_visited, stack, edges);
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
