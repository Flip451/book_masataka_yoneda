use std::{
    fmt::Debug,
    io,
    ops::{Add, Sub},
    str::FromStr,
};

fn main() {
    // 入力読み込み
    let p1: Vec<i128> = read_line("Error at reading x1, y1");
    let p2: Vec<i128> = read_line("Error at reading x2, y2");
    let p3: Vec<i128> = read_line("Error at reading x3, y3");
    let p4: Vec<i128> = read_line("Error at reading x4, y4");

    // 読み取った座標をベクトル化
    let p1: Point<i128> = Point { x: p1[0], y: p1[1] };
    let p2: Point<i128> = Point { x: p2[0], y: p2[1] };
    let p3: Point<i128> = Point { x: p3[0], y: p3[1] };
    let p4: Point<i128> = Point { x: p4[0], y: p4[1] };

    // x1 < x2, x3 < x4 となるようにラベルをつけなおし
    let (p1, p2) = if p1.x < p2.x {(p1, p2)} else {(p2, p1)};
    let (p3, p4) = if p3.x < p4.x {(p3, p4)} else {(p4, p3)};

    let l_12 = |p: Point<i128>| (p2.x - p1.x) * (p.y - p1.y) - (p2.y - p1.y) * (p.x - p1.x);
    let l_34 = |p: Point<i128>| (p4.x - p3.x) * (p.y - p3.y) - (p4.y - p3.y) * (p.x - p3.x);

    let output = if l_12(p3) * l_12(p4) < 0_i128 && l_34(p1) * l_34(p2) < 0_i128 {
        "Yes"
    } else if l_12(p3) == 0_i128 && p1.x <= p3.x && p3.x <= p2.x {
        // p3 が 直線 1-2 上にある時
        "Yes"
    } else if l_12(p4) == 0_i128 && p1.x <= p4.x && p4.x <= p2.x {
        // p4 が 直線 1-2 上にある時
        "Yes"
    } else if l_34(p1) == 0_i128 && p3.x <= p1.x && p1.x <= p4.x {
        // p1 が 直線 3-4 上にある時
        "Yes"
    } else if l_34(p2) == 0_i128 && p3.x <= p2.x && p2.x <= p4.x {
        // p2 が 直線 3-4 上にある時
        "Yes"
    } else {
        // それ以外の場合
        "No"
    };

    println!("{}", output);
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

#[derive(Clone, Copy)]
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

