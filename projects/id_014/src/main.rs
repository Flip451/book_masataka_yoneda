use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let mut n: u64 = input.trim().parse().expect("incorrect value");

    let limit = (n as f32).sqrt().floor();

    let mut v: Vec<String> = vec![];

    for i in 2..=(limit as u64) {
        while n % i == 0 {
            n /= i;
            v.push(i.to_string());
        }
    }

    if n != 1 {
        v.push(n.to_string());
    }

    println!("{}", v.join(" "));

    Ok(())
}
