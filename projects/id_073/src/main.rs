use std::{error::Error, fmt::Debug, io, str::FromStr, vec};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    const M: usize = 1000000007;
    let n = read_line::<usize>()?[0];
    let a = read_line::<usize>().unwrap();

    let mut sum = 0;
    // 2^{ a[i] 未満の整数の個数 }
    let mut mul = 1;

    (0..n).for_each(|i| {
        sum = (sum + mul * a[i] % M) % M;
        mul = (mul * 2) % M;
    });

    println!("{}", sum);
    Ok(())
}

// a^b % m を求める関数
fn mod_pow(a: usize, b: usize, m: usize) -> usize {
    let max = (b as f64).log2().floor() as usize;
    let mut a = a;
    let mut output = 1;
    for i in 0..=max {
        if (b >> i) % 2 == 1 {
            output = (output % m) * (a % m);
        }
        a = (a % m) * (a % m);
    }
    output % m
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mod_pow() {
        assert_eq!(
            934801994,
            super::mod_pow(97, 998244353, 10_usize.pow(9) + 7)
        );
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
