use std::{cmp, error::Error, fmt::Debug, io, str::FromStr, vec};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let ab = read_line::<usize>()?;
    let (a, b) = (ab[0], ab[1]);

    // let mut max = 1;

    // for x in a..b {
    //     for y in (x + 1)..=b {
    //         let gcd = gcd(x, y).0;
    //         max = cmp::max(max, gcd);
    //     }
    // }

    let max = (1..=b)
        .filter(|&gcd| {
            // A 以上の最小の gcd の倍数
            let min_multiple = ((a - 1) / gcd + 1) * gcd;
            // min_multiple より一つ大きな gcd の倍数
            let next_multiple = min_multiple + gcd;
            // println!(
            //     "gcd: {}, a: {}, x: {}, y: {}, b: {}",
            //     gcd, a, min_multiple, next_multiple, b
            // );
            next_multiple <= b
        })
        .last()
        .unwrap();

    println!("{}", max);
    Ok(())
}

fn gcd(a: usize, b: usize) -> (usize, usize) {
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

fn read_line<T>() -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr + Debug,
    T::Err: Error + 'static,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let inputs = input.trim().split_whitespace();
    let mut output = vec![];
    for input in inputs {
        let input = input.parse::<T>()?;
        output.push(input);
    }
    Ok(output)
}
