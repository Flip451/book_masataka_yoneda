use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let v: Vec<&str> = input.trim().split_whitespace().collect();
    let n: i32 = v[0].trim().parse().expect("N is incorrect.");
    let x: i32 = v[1].trim().parse().expect("X is incorrect.");
    let y: i32 = v[2].trim().parse().expect("Y is incorrect.");

    let mut output = 0;

    for i in 1..=n {
        if i % x == 0 || i % y == 0 {
            output += 1;
        }
    }
    println!("{}", output);

    Ok(())
}
