use std::{
    cell::RefCell,
    fmt::Debug,
    hash::Hash,
    io,
    rc::{Rc, Weak},
    str::FromStr,
};
fn main() {
    let nm = read_line::<usize>("Error at reading N, M");
    let (n, m) = (nm[0], nm[1]);

    let nodes: Vec<Rc<Node>> = (1..=n)
        .map(|i| {
            Rc::new(Node {
                id: i,
                next_nodes: RefCell::new(vec![]),
            })
        })
        .collect();

    (1..=m).for_each(|i| {
        let ab_i = read_line::<usize>(&format!("Error at reading A_{0}, B_{0}", i));
        let (a_i, b_i) = (ab_i[0], ab_i[1]);

        let node_a = nodes
            .iter()
            .find(|node| node.id == a_i)
            .expect(&format!("Node {} cannot be found", a_i));
        let node_b = nodes
            .iter()
            .find(|node| node.id == b_i)
            .expect(&format!("Node {} cannot be found", a_i));

        node_a.next_nodes.borrow_mut().push(Rc::downgrade(node_b));
        node_b.next_nodes.borrow_mut().push(Rc::downgrade(node_a));
    });

    let mut visited: Vec<Rc<Node>> = vec![];
    // nodes.iter().for_each(|node| {
    //     visited.insert(Rc::clone(&node), false);
    // });
    let first_node = nodes.first().unwrap();
    dfs(first_node, &mut visited);

    let output = visited.len() == n;
    let output = if output {
        "The graph is connected."
    } else {
        "The graph is not connected."
    };
    println!("{}", output);
}

#[derive(Debug)]
struct Node {
    id: usize,
    next_nodes: RefCell<Vec<Weak<Node>>>,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn dfs<'a>(current_node: &'a Rc<Node>, visited: &mut Vec<Rc<Node>>) {
    visited.push(Rc::clone(current_node));
    let next_nodes = current_node.next_nodes.borrow();
    for next_node in next_nodes.iter() {
        let next_node = &next_node.upgrade().unwrap();
        if !visited.contains(next_node) {
            dfs(next_node, visited)
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
