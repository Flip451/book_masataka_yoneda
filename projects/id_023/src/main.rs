use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let n: Vec<u128> = read_line("Error at reading N");
    let n: f32 = n[0] as f32;
    let b: Vec<f32> = read_line("Error at reading B");
    let r: Vec<f32> = read_line("Error at reading R");

    let output: f32 = b.into_iter().sum::<f32>() / n + r.into_iter().sum::<f32>() / n;

    println!("{:?}", output);
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
