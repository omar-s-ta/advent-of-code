use std::io::BufRead;

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
struct IdRange {
    b: usize,
    e: usize,
}

impl IdRange {
    fn new(b: usize, e: usize) -> Self {
        IdRange { b, e }
    }

    fn contains(&self, elem: usize) -> bool {
        self.b <= elem && elem <= self.e
    }

    fn mergeable(&self, other: &Self) -> bool {
        other.b <= self.e
    }

    fn merge(&mut self, other: &Self) {
        self.e = self.e.max(other.e)
    }

    fn len(&self) -> usize {
        if self.b > self.e {
            0
        } else {
            self.e - self.b + 1
        }
    }
}

impl From<Vec<&str>> for IdRange {
    fn from(value: Vec<&str>) -> Self {
        let (s, e) = (value[0], value[1]);
        let s = s.trim().parse::<usize>().unwrap();
        let e = e.trim().parse::<usize>().unwrap();
        IdRange::new(s, e)
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let lines = std::io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>();

    let empty_line_idx = lines
        .iter()
        .position(|l| l.is_empty())
        .unwrap_or(lines.len());

    let ranges = {
        let mut ranges_vec = lines
            .iter()
            .take(empty_line_idx)
            .map(|l| {
                let range = l.splitn(2, '-').collect::<Vec<_>>();
                IdRange::from(range)
            })
            .collect::<Vec<_>>();

        ranges_vec.sort();
        ranges_vec
    };

    let merge_fn = |mut ingredients: Vec<IdRange>, ingredient| match ingredients.last_mut() {
        Some(last) if last.mergeable(ingredient) => {
            last.merge(ingredient);
            ingredients
        }
        _ => {
            ingredients.push(ingredient.clone());
            ingredients
        }
    };

    let fresh = ranges
        .iter()
        .fold(Vec::new(), merge_fn)
        .iter()
        .map(|r| r.len())
        .sum::<usize>();

    let count = lines
        .iter()
        .skip(empty_line_idx)
        .filter_map(|line| line.parse::<usize>().ok())
        .filter(|n| ranges.iter().any(|r| r.contains(*n)))
        .count();

    println!("{} {}", count, fresh);
    Ok(())
}
