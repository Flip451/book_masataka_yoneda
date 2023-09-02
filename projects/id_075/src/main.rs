use std::{error::Error, fmt::Debug, io, str::FromStr, vec};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    const M: usize = 1000000007;
    let n = read_line::<usize>()?[0];
    let a = read_line::<usize>()?;

    // factorial[i] = i! % M となるように計算
    let factorials = create_factorials(n - 1, M);

    let output: usize = (0..n)
        .map(|i| {
            let combination = ((factorials[n - 1] * mod_inv(factorials[i], M) % M)
                * mod_inv(factorials[n - 1 - i], M))
                % M;
            // println!("comb: {}", combination);
            a[i] * combination % M
        })
        .sum();

    println!("{}", output % M);
    Ok(())
}

// a ^ -1 を求める関数
// m が素数の時、a ^ -1 = a ^ (m -2) mod m を利用
fn mod_inv(a: usize, prime: usize) -> usize {
    mod_pow(a, prime - 2, prime)
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

// 0!, 1!, 2!, ..., n! までを格納するベクタを生成する関数
fn create_factorials(n: usize, m: usize) -> Vec<usize> {
    let mut factorials = vec![1_usize];
    let mut mul = 1;
    for i in 1..=n {
        mul = mul * i % m;
        factorials.push(mul);
    }

    factorials
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

#[cfg(test)]
mod tests {
    use crate::create_factorials;

    #[test]
    fn test_create_factorials() {
        const M: usize = 1000000007;
        let expectation = vec![1, 1, 2, 6, 24, 120, 720];
        let result = create_factorials(6, M);
        assert_eq!(expectation, result);
        assert_eq!(720, result[6]);
    }
}
