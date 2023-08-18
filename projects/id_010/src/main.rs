use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: u64 = input.trim().parse().expect("incorrect value");

    let mut output: u64 = 1;

    for i in 1..=n {
        output *= i;
    }

    println!("{}", output);

    Ok(())
}
