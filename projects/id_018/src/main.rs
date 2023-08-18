use std::{collections::HashMap, io};

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let _: usize = input.trim().parse().expect("n is incorrect.");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let a: Vec<&str> = input.trim().split_whitespace().collect();

    // let mut m: HashMap<&str, u32> = HashMap::from([("100", 0), ("200", 0), ("300", 0), ("400", 0)]);
    // let mut m: HashMap<&str, u32> = [("100", 0), ("200", 0), ("300", 0), ("400", 0)].into();
    let mut m: HashMap<&str, u32> = HashMap::new();
    m.insert("100", 0);
    m.insert("200", 0);
    m.insert("300", 0);
    m.insert("400", 0);

    for a_i in a {
        let a_i: &str = a_i;
        let count = m.entry(a_i).or_insert(0);
        *count += 1;
    }

    // println!("{:?}", m);

    let output: u32 = m.get("100").unwrap() * m.get("400").unwrap()
        + m.get("200").unwrap() * m.get("300").unwrap();
    println!("{}", output);
    Ok(())
}
