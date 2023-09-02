use std::{error::Error, fmt::Debug, io, str::FromStr, vec};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let n = read_line::<isize>()?[0];
    let mut xys: Vec<(isize, isize)> = vec![];
    for _ in 0..n {
        let xy = read_line::<isize>()?;
        let (x, y) = (xy[0], xy[1]);
        xys.push((x, y));
    }
    let (mut x, mut y): (Vec<isize>, Vec<isize>) = xys.into_iter().unzip();
    x.sort();
    y.sort();

    let sum_x: isize = (0..n).map(|i| (2 * i - n + 1) * x[i as usize]).sum();
    let sum_y: isize = (0..n).map(|i| (2 * i - n + 1) * y[i as usize]).sum();

    println!("{}", sum_x + sum_y);
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
