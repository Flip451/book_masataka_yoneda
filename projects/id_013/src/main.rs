use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: u64 = input.trim().parse().expect("incorrect value");

    let limit = (n as f32).sqrt().floor();

    // let mut v: Vec<String> = vec![];

    for i in 1..=(limit as u64) {
        if n % i == 0 {
            // v.push(i.to_string());
            // v.push((n / i).to_string());
            print!("{} {} ", i, n / i);
        }
    }

    // println!("{}", v.join(" "));
    println!("");

    Ok(())
}
