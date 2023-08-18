use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: u64 = input.trim().parse().expect("incorrect value");

    let limit = (n as f32).sqrt().floor();

    for i in 2..=(limit as u64) {
        if n % i == 0 {
            println!("No");
            return Ok(());
        }
    }

    println!("Yes");

    Ok(())
}
