use std::{
    fmt::Debug,
    io,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

fn main() {
    // 入力読み込み
    let c1: Vec<f64> = read_line("Error at reading Ciecle 1");
    let c2: Vec<f64> = read_line("Error at reading Ciecle 2");

    // 値を変数に格納
    let r1 = c1[2];
    let c1 = Point { x: c1[0], y: c1[1] };
    let r2 = c2[2];
    let c2 = Point { x: c2[0], y: c2[1] };

    // r1 と r2 の大小を比較
    let (r_min, r_max) = if r1 < r2 { (r1, r2) } else { (r2, r1) };

    // 二つの円の原点の距離
    let d_2 = c1.distance_2(&c2);
    let d = d_2.sqrt();

    // println!("c1: {:?}", c1);
    // println!("c2: {:?}", c2);
    // println!("r_min: {}, r_max: {}", r_min, r_max);
    // println!("d: {}", d);

    let case = if d + r_min < r_max {
        "1"
    } else if d + r_min == r_max {
        "2"
    } else if d_2 == (r_min + r_max).powf(2_f64) {
        "4"
    } else if d > r_min + r_max {
        "5"
    } else {
        "3"
    };

    println!("{}", case);
}

// 標準入力をベクタに取り込むための関数
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

#[derive(Clone, PartialEq, PartialOrd, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// ベクトルの和の実装
impl<T> Add for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Point<T>) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// ベクトルの差の実装
impl<T> Sub for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// ベクトルの定数倍の実装
impl<T> Point<T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul(&self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// ベクトルの定数分の１の実装
impl<T> Point<T>
where
    T: Div<Output = T> + Copy,
{
    fn div(&self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

// 内積の実装
// 長さの二乗の実装
// 距離の二乗の実装
impl<T> Point<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    fn inner_product(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    fn length_2(&self) -> T {
        self.inner_product(self)
    }

    fn distance_2(&self, rhs: &Self) -> T {
        let d = self.clone() - rhs.clone();
        d.length_2()
    }
}

// 外積の実装
impl<T> Point<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Copy,
{
    fn outer_product(&self, rhs: &Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}
