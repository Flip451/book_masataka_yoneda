use std::{error::Error, fmt::Debug, io, str::FromStr, vec};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let n = read_line::<isize>()?[0];
    let mut a = read_line::<isize>()?;
    a.sort();

    let output: isize = (0..n).map(|i| (2 * i - n + 1) * a[i as usize]).sum();
    println!("{}", output);
    Ok(())
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
