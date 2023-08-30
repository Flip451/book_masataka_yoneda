use std::{fmt::Debug, io, str::FromStr};
fn main() {
    let nm = read_line::<usize>("Error at reading N, M");
    let (n, m) = (nm[0], nm[1]);

    let mut next_nodes: Vec<Vec<usize>> = vec![vec![]];

    (1..=n).for_each(|_| {
        next_nodes.push(vec![]);
    });

    (1..=m).for_each(|i| {
        let ab_i = read_line::<usize>(&format!("Error at reading A_{0}, B_{0}", i));
        let (a_i, b_i) = (ab_i[0], ab_i[1]);

        next_nodes[a_i].push(b_i);
        next_nodes[b_i].push(a_i);
    });

    let mut visited: Vec<bool> = vec![true];

    (1..=n).for_each(|_| {
        visited.push(false);
    });

    // nodes.iter().for_each(|node| {
    //     visited.insert(Rc::clone(&node), false);
    // });
    let first_node = 1;
    dfs(first_node, &next_nodes, &mut visited);

    let output = visited.iter().all(|&b| b);
    let output = if output {
        "The graph is connected."
    } else {
        "The graph is not connected."
    };
    println!("{}", output);
}

fn dfs(current_node: usize, next_nodes: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[current_node] = true;
    for next_node in next_nodes[current_node].iter() {
        if !visited[*next_node] {
            dfs(*next_node, next_nodes, visited)
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
