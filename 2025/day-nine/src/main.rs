use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::io::BufRead;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: &Point) -> i64 {
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

#[derive(Ord, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn new(min: i64, max: i64) -> Self {
        Range { min, max }
    }

    fn contains(&self, other: &Range) -> bool {
        self.min <= other.min && other.max <= self.max
    }

    fn contained_in(&self, ranges: &[Range]) -> bool {
        ranges.iter().any(|r| r.contains(self))
    }
}

impl From<&Line> for Range {
    fn from(line: &Line) -> Self {
        Range::new(line.min, line.max)
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Line {
    x_or_y: i64,
    min: i64,
    max: i64,
}

impl Line {
    fn new(x_or_y: i64, min: i64, max: i64) -> Self {
        Line { x_or_y, min, max }
    }

    fn blocks(&self, other: &Line) -> bool {
        self.x_or_y == other.x_or_y && self.min <= other.min && other.max <= self.max
    }
}

fn cells(rows: usize, cols: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..rows).flat_map(move |i| (0..cols).map(move |j| (i, j)))
}

fn largest_area(points: &[Point]) -> i64 {
    let n = points.len();
    (0..n)
        .flat_map(|i| ((i + 1)..n).map(move |j| points[i].area(&points[j])))
        .max()
        .unwrap_or_default()
}

fn compress<F>(points: &[Point], extract: F) -> Vec<i64>
where
    F: Fn(&Point) -> i64,
{
    let unique = points.iter().map(extract).collect::<BTreeSet<_>>();
    let (min, max) = (*unique.first().unwrap(), *unique.last().unwrap());
    std::iter::once(min - 1)
        .chain(unique)
        .chain(std::iter::once(max + 1))
        .collect()
}

fn build_lines(points: &[Point]) -> (HashSet<Line>, HashSet<Line>) {
    (0..points.len())
        .map(|i| (points[i], points[(i + 1) % points.len()]))
        .fold(
            (HashSet::new(), HashSet::new()),
            |(mut h_lines, mut v_lines), (p1, p2)| {
                if p1.x == p2.x {
                    h_lines.insert(Line::new(p1.x, p1.y.min(p2.y), p1.y.max(p2.y)));
                } else {
                    v_lines.insert(Line::new(p1.y, p1.x.min(p2.x), p1.x.max(p2.x)));
                }
                (h_lines, v_lines)
            },
        )
}

fn flood_fill_outside(
    xs: &[i64],
    ys: &[i64],
    h_lines: &HashSet<Line>,
    v_lines: &HashSet<Line>,
) -> Vec<Vec<bool>> {
    let rows = xs.len() - 1;
    let cols = ys.len() - 1;

    let h_blocked = cells(rows - 1, cols)
        .filter(|&(i, j)| {
            let h_line = Line::new(xs[i + 1], ys[j], ys[j + 1]);
            h_lines.iter().any(|&line| line.blocks(&h_line))
        })
        .collect::<HashSet<_>>();

    let v_blocked = cells(rows, cols - 1)
        .filter(|&(i, j)| {
            let v_line = Line::new(ys[j + 1], xs[i], xs[i + 1]);
            v_lines.iter().any(|&line| line.blocks(&v_line))
        })
        .collect::<HashSet<_>>();

    let mut outside = vec![vec![false; cols]; rows];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((0, 0));
    outside[0][0] = true;

    while let Some((i, j)) = queue.pop_front() {
        if i + 1 < rows && !outside[i + 1][j] && !h_blocked.contains(&(i, j)) {
            outside[i + 1][j] = true;
            queue.push_back((i + 1, j));
        }
        if i > 0 && !outside[i - 1][j] && !h_blocked.contains(&(i - 1, j)) {
            outside[i - 1][j] = true;
            queue.push_back((i - 1, j));
        }
        if j + 1 < cols && !outside[i][j + 1] && !v_blocked.contains(&(i, j)) {
            outside[i][j + 1] = true;
            queue.push_back((i, j + 1));
        }
        if j > 0 && !outside[i][j - 1] && !v_blocked.contains(&(i, j - 1)) {
            outside[i][j - 1] = true;
            queue.push_back((i, j - 1));
        }
    }

    outside
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    ranges.sort();
    ranges.into_iter().fold(Vec::new(), |mut acc, range| {
        match acc.last_mut() {
            Some(last) if range.min <= last.max => last.max = last.max.max(range.max),
            _ => acc.push(range),
        }
        acc
    })
}

fn polygon_h_ranges(
    xs: &[i64],
    ys: &[i64],
    h_lines: &HashSet<Line>,
    outside: &[Vec<bool>],
) -> HashMap<i64, Vec<Range>> {
    let rows = xs.len() - 1;
    let cols = ys.len() - 1;
    xs.iter()
        .enumerate()
        .map(|(i, &row)| {
            let lines = h_lines.iter().filter(|l| l.x_or_y == row).map(Range::from);
            let interior = (0..cols)
                .filter(|&j| i < rows && !outside[i][j])
                .map(|j| Range::new(ys[j], ys[j + 1]));

            (row, merge_ranges(lines.chain(interior).collect()))
        })
        .collect()
}

fn polygon_v_ranges(
    xs: &[i64],
    ys: &[i64],
    v_lines: &HashSet<Line>,
    outside: &[Vec<bool>],
) -> HashMap<i64, Vec<Range>> {
    let rows = xs.len() - 1;
    let cols = ys.len() - 1;
    ys.iter()
        .enumerate()
        .map(|(j, &col)| {
            let lines = v_lines.iter().filter(|l| l.x_or_y == col).map(Range::from);
            let interior = (0..rows)
                .filter(|&i| j < cols && !outside[i][j])
                .map(|i| Range::new(xs[i], xs[i + 1]));

            (col, merge_ranges(lines.chain(interior).collect()))
        })
        .collect()
}

fn is_valid_rectangle(
    pa: &Point,
    pb: &Point,
    x_idx: &HashMap<i64, usize>,
    y_idx: &HashMap<i64, usize>,
    outside: &[Vec<bool>],
    h_ranges: &HashMap<i64, Vec<Range>>,
    v_ranges: &HashMap<i64, Vec<Range>>,
) -> Option<i64> {
    let (xi, xj) = (pa.x.min(pb.x), pa.x.max(pb.x));
    let (yi, yj) = (pa.y.min(pb.y), pa.y.max(pb.y));

    let all_inside =
        (x_idx[&xi]..x_idx[&xj]).all(|i| (y_idx[&yi]..y_idx[&yj]).all(|j| !outside[i][j]));

    let y_range = Range::new(yi, yj);
    let x_range = Range::new(xi, xj);

    let valid_h = [xi, xj]
        .iter()
        .all(|x| h_ranges.get(x).is_some_and(|rs| y_range.contained_in(rs)));

    let valid_v = [yi, yj]
        .iter()
        .all(|y| v_ranges.get(y).is_some_and(|rs| x_range.contained_in(rs)));

    (all_inside && valid_h && valid_v).then(|| pa.area(pb))
}

fn largest_area_part_two(points: &[Point]) -> i64 {
    let xs = compress(points, |p| p.x);
    let ys = compress(points, |p| p.y);
    let (h_lines, v_lines) = build_lines(points);

    let outside = flood_fill_outside(&xs, &ys, &h_lines, &v_lines);
    let h_ranges = polygon_h_ranges(&xs, &ys, &h_lines, &outside);
    let v_ranges = polygon_v_ranges(&xs, &ys, &v_lines, &outside);

    let x_idx = xs.iter().copied().zip(0..).collect::<HashMap<_, _>>();
    let y_idx = ys.iter().copied().zip(0..).collect::<HashMap<_, _>>();

    let n = points.len();
    (0..n)
        .flat_map(|i| ((i + 1)..n).map(move |j| (points[i], points[j])))
        .filter_map(|(pa, pb)| {
            is_valid_rectangle(&pa, &pb, &x_idx, &y_idx, &outside, &h_ranges, &v_ranges)
        })
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

    println!("Part 1: {}", largest_area(&points));
    println!("Part 2: {}", largest_area_part_two(&points));
    Ok(())
}
