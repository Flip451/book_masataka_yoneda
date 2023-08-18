use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let n: u32 = input.trim().parse().expect("Please type a number!");

    println!("{}", n + 5);

    Ok(())
}
