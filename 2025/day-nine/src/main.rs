use std::io::BufRead;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: Point) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

impl From<String> for Point {
    fn from(value: String) -> Self {
        let parsed = value
            .split(',')
            .map_while(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>();

        Point {
            x: parsed[1],
            y: parsed[0],
        }
    }
}

fn largest_area(points: &[Point]) -> i64 {
    let n = points.len();
    (0..n)
        .flat_map(|i| ((i + 1)..n).map(move |j| points[i].area(points[j])))
        .max()
        .unwrap_or_default()
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);
    let points = reader
        .lines()
        .map_while(Result::ok)
        .map(Point::from)
        .collect::<Vec<_>>();

    println!("{}", largest_area(&points));
    Ok(())
}
