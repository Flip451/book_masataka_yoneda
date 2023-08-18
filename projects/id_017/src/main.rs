use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let _: u128 = input.trim().parse().expect("n is incorrect");

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let a: Vec<&str> = input.trim().split_whitespace().collect();

    let mut output: u128 = a[0].parse().expect("incorrect value in A_i");

    for a_i in a.into_iter() {
        let a_i = a_i.parse().expect("incorrect value in A_i");
        output = lcm(a_i, output);
    }

    println!("{}", output);

    Ok(())
}

fn gcd(a: u128, b: u128) -> (u128, u128) {
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

fn lcm(a: u128, b: u128) -> u128 {
    let gcd = gcd(a, b).0;
    a * b / gcd
}