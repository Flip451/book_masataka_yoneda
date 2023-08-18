use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let v: Vec<&str> = input.trim().split_whitespace().collect();

    let a = v[0].trim().parse().expect("a is incorrect value");
    let b = v[1].trim().parse().expect("b is incorrect value");
    let (output, _) = gcd(a, b);

    println!("{}", output);

    Ok(())
}

fn gcd(a: u64, b: u64) -> (u64, u64) {
    if a == 0 {
        (b, 0)
    } else if b == 0 {
        (a, 0)
    } else if a > b {
        gcd(a % b, b)
    } else {
        gcd(b % a, a)
    }
}
