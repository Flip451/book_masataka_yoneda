use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() -> io::Result<()> {
    let n: Vec<usize> = read_line("Error at reading N");
    let n = n[0];
    let a: Vec<u16> = read_line("Error at reading A_1, ..., A_N");

    let mut output: u128 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                for l in (k + 1)..n {
                    for m in (l + 1)..n {
                        if a[i] + a[j] + a[k] + a[l] + a[m] == 1000 {
                            output += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", output);

    Ok(())
}

fn read_line<T>(err: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(err);

    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let mut elements: Vec<T> = Vec::new();

    for e in input.into_iter() {
        let new_elemnt: T = e.trim().parse::<T>().expect(err);
        elements.push(new_elemnt);
    }

    elements
}
