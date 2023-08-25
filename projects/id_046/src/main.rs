use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    io,
    str::FromStr,
};

// 巾優先探索
//

fn main() {
    // R, C の読み込み
    let rc: Vec<usize> = read_line("Error at reading R, C");
    let (r, c) = (rc[0], rc[1]);

    // スタートの読み込み
    let s: Vec<usize> = read_line("Error at readin sx, sy");
    let (sy, sx) = (s[0], s[1]);
    let start = Position { x: sx, y: sy };

    // ゴールの読み込み
    let g: Vec<usize> = read_line("Error at readin gx, gy");
    let (gy, gx) = (g[0], g[1]);
    let goal = Position { x: gx, y: gy };

    let mut nodes: HashMap<Position, Node> = HashMap::new();
    (1..=r).for_each(|y| {
        let c_y = read_line::<String>(&format!("Error at reading c_({} , ...)", y));
        let c_y = c_y[0].chars().collect::<Vec<char>>();
        (1..=c).for_each(|x| {
            let p = Position { x, y };
            let node = if c_y[x - 1] == '#' {
                Node::Wall
            } else {
                Node::Road { dist: None }
            };
            nodes.insert(p, node);
        });
    });

    // BFS に使用するキュー
    let mut queue: VecDeque<Position> = VecDeque::new();

    // BFS のための初期化
    nodes.insert(start, Node::Road { dist: Some(0) });
    queue.push_back(start);

    // BFS
    while queue.len() > 0 {
        let current_position = queue.pop_front().unwrap();
        let current_node = nodes.get(&current_position).unwrap();
        let Position { x, y } = current_position;

        match current_node {
            &Node::Road {
                dist: Some(current_dist),
            } => {
                // 現在地の左のマス
                if x > 1 {
                    let left_pos = Position { x: x - 1, y };
                    let left_node = nodes.get(&left_pos).unwrap();
                    if let Node::Road { dist: None } = left_node {
                        queue.push_back(left_pos);
                        nodes.insert(
                            left_pos,
                            Node::Road {
                                dist: Some(current_dist + 1),
                            },
                        );
                    };
                }
                // 現在地の上のマス
                if y > 1 {
                    let upper_pos = Position { x, y: y - 1 };
                    let upper_node = nodes.get(&upper_pos).unwrap();
                    if let Node::Road { dist: None } = upper_node {
                        queue.push_back(upper_pos);
                        nodes.insert(
                            upper_pos,
                            Node::Road {
                                dist: Some(current_dist + 1),
                            },
                        );
                    };
                }
                // 現在地の右のマス
                if x < c {
                    let right_pos = Position { x: x + 1, y };
                    let right_node = nodes.get(&right_pos).unwrap();
                    if let Node::Road { dist: None } = right_node {
                        queue.push_back(right_pos);
                        nodes.insert(
                            right_pos,
                            Node::Road {
                                dist: Some(current_dist + 1),
                            },
                        );
                    };
                }
                // 現在地の下のマス
                if y < r {
                    let lower_pos = Position { x, y: y + 1 };
                    let lower_node = nodes.get(&lower_pos).unwrap();
                    if let Node::Road { dist: None } = lower_node {
                        queue.push_back(lower_pos);
                        nodes.insert(
                            lower_pos,
                            Node::Road {
                                dist: Some(current_dist + 1),
                            },
                        );
                    };
                }
            }
            &Node::Road { .. } => {
                panic!("スタート地点からの距離の記録に問題があるようです...")
            }
            Node::Wall => {
                panic!("壁の中に突入したようです...");
            }
        }
    }

    // (1..=r).for_each(|y| {
    //     (1..=c).for_each(|x| {
    //         let node = nodes.get(&Position { x, y }).unwrap();
    //         match node {
    //             Node::Road { dist: Some(dist) } => {
    //                 print!("{}", dist % 10);
    //             }
    //             Node::Road { dist: None } => {
    //                 print!(".");
    //             }
    //             Node::Wall => {
    //                 print!("#");
    //             }
    //         }
    //     });
    //     println!();
    // });

    if let Node::Road { dist } = nodes.get(&goal).unwrap() {
        let output = dist.unwrap();
        println!("{:?}", output);
    }
}

#[derive(Debug)]
enum Node {
    Wall,
    Road { dist: Option<usize> },
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
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
