use std::{
    cmp,
    error::Error,
    fmt::Debug,
    io,
    ops::{Mul, Sub},
    str::FromStr,
    isize::MAX,
    vec,
};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let nk = read_line::<usize>()?;
    let (n, k) = (nk[0], nk[1]);

    let mut points: Vec<Point<isize>> = vec![];
    for _ in 0..n {
        let xy = read_line::<isize>()?;
        let point = Point { x: xy[0], y: xy[1] };
        points.push(point);
    }

    let mut min: isize = MAX;

    for i_top in 0..n {
        for i_bottom in 0..n {
            for i_left in 0..n {
                for i_right in 0..n {
                    let y_top = points[i_top].y;
                    let y_bottom = points[i_bottom].y;
                    let x_left = points[i_left].x;
                    let x_right = points[i_right].x;
                    if y_top <= y_bottom || x_right <= x_left {
                        continue;
                    }
                    let lect = Lectangle {
                        y_top,
                        y_bottom,
                        x_left,
                        x_right,
                    };
                    let mut count = 0;
                    for point in points.iter() {
                        if lect.contains(point) {
                            count += 1;
                        }
                    }
                    if count == k {
                        min = cmp::min(min, lect.area());
                    }
                }
            }
        }
    }

    println!("{}", min);
    Ok(())
}

struct Point<T> {
    x: T,
    y: T,
}

struct Lectangle<T> {
    y_top: T,
    y_bottom: T,
    x_left: T,
    x_right: T,
}

impl<T> Lectangle<T>
where
    T: PartialOrd + Sub<Output = T> + Mul<Output = T> + Clone + Copy,
{
    fn contains(&self, point: &Point<T>) -> bool {
        let x_p = point.x;
        let y_p = point.y;
        self.x_left <= x_p && x_p <= self.x_right && self.y_bottom <= y_p && y_p <= self.y_top
    }

    fn area(&self) -> T {
        let width = self.x_right - self.x_left;
        let height = self.y_top - self.y_bottom;
        width * height
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
