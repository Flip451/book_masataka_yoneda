use std::io;

fn main() -> io::Result<()> {

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n = input;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let v: Vec<&str> = input.trim().split_whitespace().collect();
    let mut sum = 0;

    for i in v.iter() {
        let value: i32 = i.trim().parse().expect("uncorrect value is detected.");
        sum += value;
    }

    println!("{}", sum);

    Ok(())
}
