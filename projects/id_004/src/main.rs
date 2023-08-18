use std::io;

fn main() -> io::Result<()>{
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let v: Vec<&str> = input.trim().split_whitespace().collect();
    let a_1: i32 = v[0].trim().parse().expect("First parameter is incorrect.");
    let a_2: i32 = v[1].trim().parse().expect("Second parameter is incorrect.");
    let a_3: i32 = v[2].trim().parse().expect("Third parameter is incorrect.");
    let output = a_1 * a_2 * a_3;

    println!("{}", output);

    Ok(())
}
