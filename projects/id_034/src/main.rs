use std::{
    f64::INFINITY,
    fmt::Debug,
    io,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

fn main() {
    let n: Vec<f32> = read_line("Error at reading N");
    let n: f32 = n[0];
    let mut points: Vec<Point<f64>> = Vec::new();
    for _ in 0..(n as usize) {
        let input = read_line("Error at reading x, y");
        let x = input[0];
        let y = input[1];
        points.push(Point { x, y });
    }

    let mut d_min = INFINITY;

    for point_1 in points.iter() {
        for point_2 in points.iter() {
            if point_1 == point_2 {
                continue;
            }
            let d = point_1.distance_2(point_2).sqrt();
            d_min = if d < d_min { d } else { d_min };
        }
    }

    println!("{:?}", d_min);
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

#[derive(Clone, PartialEq, PartialOrd)]
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
