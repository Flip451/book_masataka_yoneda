use std::{
    fmt::Debug,
    io,
    ops::{Add, Mul, Rem},
    str::FromStr,
};

#[rustfmt::skip]
fn main() {
    let m = 10_usize.pow(9);
    let n = read_line::<usize>("Error at reading N")[0];
    
    let a = Matrix2::<usize> {
        a_11: 1, a_12: 1,
        a_21: 1, a_22: 0,
    };
    let a = mod_pow(a, n - 2, m);
    let output = (a.a_11 + a.a_12) % m;

    println!("{}", output);
}

// 標準入力をベクタに読み込むための関数
fn read_line<T>(err: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect(err);

    let output: Vec<T> = input
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|w| w.parse().expect(err))
        .collect();

    output
}

#[rustfmt::skip]
#[derive(Copy, Clone, Debug, PartialEq)]
struct Matrix2<T> {
    a_11: T, a_12: T,
    a_21: T, a_22: T,
}

#[rustfmt::skip]
impl<T> Mul for Matrix2<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a_11 = self.a_11 * rhs.a_11 + self.a_12 * rhs.a_21;
        let a_12 = self.a_11 * rhs.a_12 + self.a_12 * rhs.a_22;
        let a_21 = self.a_21 * rhs.a_11 + self.a_22 * rhs.a_21;
        let a_22 = self.a_21 * rhs.a_12 + self.a_22 * rhs.a_22;
        Matrix2 {
            a_11, a_12,
            a_21, a_22
        }
    }
}

impl<T> Rem<usize> for Matrix2<T>
where
    T: Rem<usize, Output = T> + Add<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn rem(self, rhs: usize) -> Self::Output {
        let a_11 = self.a_11 % rhs;
        let a_12 = self.a_12 % rhs;
        let a_21 = self.a_21 % rhs;
        let a_22 = self.a_22 % rhs;
        Matrix2 {
            a_11,
            a_12,
            a_21,
            a_22,
        }
    }
}

// a^b % m を求める関数
fn mod_pow(a: Matrix2<usize>, b: usize, m: usize) -> Matrix2<usize> {
    let max = (b as f64).log2().floor() as usize;
    let mut a = a;
    let mut output = Matrix2 {
        a_11: 1,
        a_12: 0,
        a_21: 0,
        a_22: 1,
    };
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
    use super::Matrix2;

    #[test]
    fn test_mod_pow() {
        assert_eq!(
            Matrix2 {
                a_11: 172_716_053,
                a_12: 683_670_769,
                a_21: 173_459_470,
                a_22: 540_057_584
            },
            super::mod_pow(
                Matrix2 {
                    a_11: 2,
                    a_12: 3,
                    a_21: 11,
                    a_22: 8
                },
                12,
                10_usize.pow(9) + 7
            )
        );
    }
}
