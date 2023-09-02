use std::{error::Error, f64::MIN, fmt::Debug, io, str::FromStr, vec, ops::{Add, Mul}};

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let n = read_line::<usize>()?[0];

    let mut lines: Vec<Line<f64>> = vec![];
    for _ in 0..n {
        let abc = read_line::<f64>()?;
        let line = Line {
            a: abc[0],
            b: abc[1],
            c: abc[2],
        };
        lines.push(line);
    }

    let mut max: f64 = MIN;

    for i in 0..n {
        for j in (i + 1)..n {
            let a_i = lines[i].a;
            let b_i = lines[i].b;
            let c_i = lines[i].c;
            let a_j = lines[j].a;
            let b_j = lines[j].b;
            let c_j = lines[j].c;
            let det = a_i * b_j - b_i * a_j;
            if det != 0_f64 {
                let x = (b_j * c_i - b_i * c_j) / det;
                let y = (c_j * a_i - c_i * a_j) / det;
                let point = Point{ x, y };
                if lines.iter().all(|line| line.is_above(&point)){
                    max = if x + y > max { x + y } else { max };
                }
            }
        }
    }

    println!("{}", max);
    Ok(())
}

struct Line<T> {
    a: T,
    b: T,
    c: T,
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Line<T>
where
    T: PartialOrd + Mul<Output = T> + Add<Output = T> + Clone + Copy,
{
    fn is_above(&self, point: &Point<T>) -> bool {
        self.a * point.x + self.b * point.y <= self.c
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
