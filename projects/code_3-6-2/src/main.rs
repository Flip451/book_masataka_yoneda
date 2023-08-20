use core::fmt::Debug;
use std::{io, str::FromStr};

fn main() {
    let a: Vec<u128> = read_line("Error at reading N");

    let output = selection_sort(a);

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

fn selection_sort<T>(mut v: Vec<T>) -> Vec<T>
where
    T: PartialOrd,
{
    let n = v.len();

    for i in 0..n {
        let mut min = i;
        for j in (i + 1)..n {
            if v[j] < v[min] {
                min = j
            }
        }
        v.swap(i, min);
    }

    v
}

#[cfg(test)]
mod tests{
    use super::selection_sort;

    #[test]
    fn test_selection_sort() {
        let v = vec![3, 99, 7, 10 , 1011, 7932, -32, 7, 55, 0];
        let v_sorted = vec![-32, 0, 3, 7, 7, 10, 55, 99, 1011, 7932];
    
        assert_eq!(selection_sort(v), v_sorted);
    }
}