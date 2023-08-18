use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let n: u32 = input.trim().parse().expect("incorrect value");

    let mut v: Vec<String> = vec![];

    'outer: for i in 2..=n {
        for j in 2..i {
            if i % j == 0 {
                continue 'outer;
            }
        }

        v.push(i.to_string());
    }

    println!("{}", v.join(" "));

    Ok(())
}
