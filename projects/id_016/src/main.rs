use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let _: u64 = input.trim().parse().expect("n is incorrect");

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let a: Vec<&str> = input.trim().split_whitespace().collect();

    let mut output: u64 = 0;

    for a_i in a.iter() {
        let a_i = a_i.parse().expect("incorrect value in A_i");
        output = gcd(a_i, output).0;
    }

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
