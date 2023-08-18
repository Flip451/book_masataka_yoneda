use std::io;

fn main() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let v: Vec<&str> = input.trim().split_whitespace().collect();
    let n: i32 = v[0].trim().parse().expect("n is incorrect.");
    let s: i32 = v[1].trim().parse().expect("s is incorrect.");
    let output = if s == 1 {
        0
    } else if s >= 2 * n {
        n ^ 2
    } else if n < s {
        (s - n) * n + (2 * n - s) * (s - 1) / 2
    } else {
        (s - 1) * s / 2
    };
    // let mut output = 0;
    // for i in 1..=n {
    //     for j in 1..=n {
    //         if i + j <= s {
    //             output += 1;
    //         }
    //     }
    // }

    println!("{}", output);

    Ok(())
}
