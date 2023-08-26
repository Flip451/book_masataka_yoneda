use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    io,
    str::FromStr,
};

fn main() {
    // N, M の読み込み
    let nm: Vec<usize> = read_line("Error at reading N, M");
    let (n, m) = (nm[0], nm[1]);

    let mut nodes = HashMap::<Node, Vec<Node>>::new();
    (1..=n).for_each(|i| {
        nodes.insert(Node(i), vec![]);
    });
    let edges = (1..=m)
        .map(|i| {
            let ab = read_line::<usize>(&format!("Error at reading A_{0}, B_{0}", i));
            let (a, b) = (ab[0], ab[1]);
            let handle_a = nodes.entry(Node(a)).or_insert(vec![]);
            handle_a.push(Node(b));
            let handle_b = nodes.entry(Node(b)).or_insert(vec![]);
            handle_b.push(Node(a));
            (Node(a), Node(b))
        })
        .collect::<Vec<(Node, Node)>>();

    let mut colors = HashMap::<Node, bool>::new();

    for start_node in nodes.keys() {
        // 着色済みのノードから開始することを回避
        if let Some(_) = colors.get(start_node) {
            continue;
        }

        // キューの初期化
        let mut queue = VecDeque::<&Node>::new();
        queue.push_back(start_node);
        colors.insert(*start_node, true);

        // BFS
        while queue.len() > 0 {
            // 処理の基準点となるノードをキューから取得
            let current_node = queue.pop_front().unwrap();
            // 基準点の色を取得
            let current_color = *colors.get(current_node).unwrap();
            // 基準点に隣接するノードを取得
            let next_nodes = nodes.get(current_node).unwrap();
            // 未着色の隣接ノードに着色して、着色したノードをキューに追加
            for next in next_nodes {
                match colors.get(next) {
                    None => {
                        queue.push_back(next);
                        colors.insert(*next, !current_color);
                    }
                    Some(&next_color) => {
                        if current_color == next_color {
                            println!("No");
                            return ();
                        }
                    }
                }
            }
        }
    }

    // println!("{:?}", colors);

    // let mut output = "Yes";
    // edges.iter().for_each(|(n1, n2)| {
    //     let color1 = colors
    //         .get(n1)
    //         .expect("色が割り振られていないノードがあるようです");
    //     let color2 = colors
    //         .get(n2)
    //         .expect("色が割り振られていないノードがあるようです");
    //     if color1 == color2 {
    //         output = "No";
    //     }
    // });
    // println!("{}", output);
    println!("Yes");
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
