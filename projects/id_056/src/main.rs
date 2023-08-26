use std::{
    fmt::Debug,
    io,
    ops::{Add, Mul, Rem},
    str::FromStr,
};

#[rustfmt::skip]
fn main() {
    let m = 10_usize.pow(9) + 7;
    let n = read_line::<usize>("Error at reading N")[0];
    
    let a = Matrix3::<usize> {
        a_11: 1, a_12: 1, a_13: 1,
        a_21: 1, a_22: 0, a_23: 0,
        a_31: 0, a_32: 1, a_33: 0,
    };
    let a = mod_pow(a, n - 3, m);
    let output = (2*a.a_11 + a.a_12 + a.a_13) % m;

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
struct Matrix3<T> {
    a_11: T, a_12: T, a_13: T,
    a_21: T, a_22: T, a_23: T,
    a_31: T, a_32: T, a_33: T,
}

#[rustfmt::skip]
impl<T> Mul for Matrix3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a_11 = self.a_11 * rhs.a_11 + self.a_12 * rhs.a_21 + self.a_13 * rhs.a_31;
        let a_12 = self.a_11 * rhs.a_12 + self.a_12 * rhs.a_22 + self.a_13 * rhs.a_32;
        let a_13 = self.a_11 * rhs.a_13 + self.a_12 * rhs.a_23 + self.a_13 * rhs.a_33;
        let a_21 = self.a_21 * rhs.a_11 + self.a_22 * rhs.a_21 + self.a_23 * rhs.a_31;
        let a_22 = self.a_21 * rhs.a_12 + self.a_22 * rhs.a_22 + self.a_23 * rhs.a_32;
        let a_23 = self.a_21 * rhs.a_13 + self.a_22 * rhs.a_23 + self.a_23 * rhs.a_33;
        let a_31 = self.a_31 * rhs.a_11 + self.a_32 * rhs.a_21 + self.a_33 * rhs.a_31;
        let a_32 = self.a_31 * rhs.a_12 + self.a_32 * rhs.a_22 + self.a_33 * rhs.a_32;
        let a_33 = self.a_31 * rhs.a_13 + self.a_32 * rhs.a_23 + self.a_33 * rhs.a_33;
        Matrix3 {
            a_11, a_12, a_13,
            a_21, a_22, a_23,
            a_31, a_32, a_33,
        }
    }
}

#[rustfmt::skip]
impl<T> Rem<usize> for Matrix3<T>
where
    T: Rem<usize, Output = T> + Add<Output = T> + Copy + Clone,
{
    type Output = Self;

    fn rem(self, rhs: usize) -> Self::Output {
        let a_11 = self.a_11 % rhs;
        let a_12 = self.a_12 % rhs;
        let a_13 = self.a_13 % rhs;
        let a_21 = self.a_21 % rhs;
        let a_22 = self.a_22 % rhs;
        let a_23 = self.a_23 % rhs;
        let a_31 = self.a_31 % rhs;
        let a_32 = self.a_32 % rhs;
        let a_33 = self.a_33 % rhs;
        Matrix3 {
            a_11, a_12, a_13,
            a_21, a_22, a_23,
            a_31, a_32, a_33,
        }
    }
}

// a^b % m を求める関数
fn mod_pow(a: Matrix3<usize>, b: usize, m: usize) -> Matrix3<usize> {
    let max = (b as f64).log2().floor() as usize;
    let mut a = a;
    let mut output = Matrix3 {
        a_11: 1,
        a_12: 0,
        a_13: 0,
        a_21: 0,
        a_22: 1,
        a_23: 0,
        a_31: 0,
        a_32: 0,
        a_33: 1,
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
    use super::Matrix3;

    #[test]
    fn test_mod_pow() {
        assert_eq!(
            Matrix3 {
                a_11: 1,
                a_12: 0,
                a_13: 0,
                a_21: 0,
                a_22: 1,
                a_23: 0,
                a_31: 0,
                a_32: 0,
                a_33: 1,
            },
            super::mod_pow(
                Matrix3 {
                    a_11: 1,
                    a_12: 0,
                    a_13: 0,
                    a_21: 0,
                    a_22: 1,
                    a_23: 0,
                    a_31: 0,
                    a_32: 0,
                    a_33: 1,
                },
                12,
                10_usize.pow(9) + 7
            )
        );
    }
}
