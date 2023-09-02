use std::{cmp, error::Error, fmt::Debug, io, str::FromStr};

fn main() {
    let abcd = read_line::<isize>().unwrap();
    let (a, b, c, d) = (abcd[0], abcd[1], abcd[2], abcd[3]);
    let output = cmp::max(cmp::max(a * c, b * c), cmp::max(a * d, b * d));
    println!("{}", output);
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
