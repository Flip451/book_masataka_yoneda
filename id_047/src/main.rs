use std::{collections::HashMap, fmt::Debug, io, str::FromStr};

fn main() {
    // N, M の読み込み
    let nm: Vec<usize> = read_line("Error at reading N, M");
    let (n, m) = (nm[0], nm[1]);

    let mut nodes = HashMap::<Node, Vec<Node>>::new();
    (1..=n).for_each(|i| {
        nodes.insert(Node(i), vec![]);
    });
    let edges = (1..=m).map(|i| {
        let ab = read_line::<usize>(&format!("Error at reading A_{0}, B_{0}", i));
        let (a, b) = (ab[0], ab[1]);
        let handle_a = nodes.entry(Node(a)).or_insert(vec![]);
        handle_a.push(Node(b));
        let handle_b = nodes.entry(Node(b)).or_insert(vec![]);
        handle_b.push(Node(a));
        (Node(a), Node(b))
    }).collect::<Vec<(Node, Node)>>();

    let mut colors = HashMap::<Node, bool>::new();
    loop {
        let start_node = nodes.keys().find(|node| {
            let color = colors.get(node);
            match color {
                Some(_) => {
                    false
                }
                None => {
                    true
                }
            }
        });
        match start_node {
            Some(&start_node) => {
                dfs(start_node, &nodes, &mut colors, true);
            }
            None => {
                break;
            }
        }
    }

    // println!("{:?}", colors);
    
    let mut output = "Yes";
    edges.iter().for_each(|(n1, n2)| {
        let color1 = colors.get(n1).expect("色が割り振られていないノードがあるようです");
        let color2 = colors.get(n2).expect("色が割り振られていないノードがあるようです");
        if color1 == color2 {
            output = "No";
        }
    });
    println!("{}", output);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

fn dfs(
    current_node: Node,
    nodes: &HashMap<Node, Vec<Node>>,
    colors: &mut HashMap<Node, bool>,
    color: bool,
) {
    // 着地したノードに色を塗る
    colors.entry(current_node).or_insert(color);
    // 隣接するノードのうち、色が塗られていないノードにdfsを再帰的に実行
    let next_nodes = nodes.get(&current_node).unwrap();
    next_nodes.iter().for_each(|next_node| {
        if let None = colors.get(next_node) {
            dfs(*next_node, nodes, colors, !color);
        }
    });
}
