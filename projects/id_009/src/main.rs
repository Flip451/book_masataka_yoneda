use std::io;

fn main() -> io::Result<()> {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;

    let input: Vec<&str> = input.trim().split_whitespace().collect();

    let n: u32 = input[0].trim().parse().expect("n is incorrect.");
    let s: i32 = input[1].trim().parse().expect("n is incorrect.");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input)?;

    let a: Vec<&str> = input.trim().split_whitespace().collect();

    let mut result = "No";

    for i in 1..=(2_u32.pow(n)) {
        let mut sum = 0;
        for j in 0..n {
            if (i >> j) % 2 == 0 {
                let a_j: i32 = a[j as usize]
                    .trim()
                    .parse()
                    .expect(&format!("A_{} is incorrect.", j));
                sum += a_j;
            }
        }
        if sum == s {
            result = "Yes"
        }
    }

    println!("{}", result);

    Ok(())
}
