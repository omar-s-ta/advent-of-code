use std::io::BufRead;

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
struct IdRange {
    b: u64,
    e: u64,
}

impl IdRange {
    fn new(b: u64, e: u64) -> Self {
        IdRange { b, e }
    }

    fn contains(&self, elem: u64) -> bool {
        (self.b..=self.e).contains(&elem)
    }
}

impl From<Vec<&str>> for IdRange {
    fn from(value: Vec<&str>) -> Self {
        let (s, e) = (value[0], value[1]);
        let s = s.trim().parse::<u64>().unwrap();
        let e = e.trim().parse::<u64>().unwrap();
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
        let mut ranges_to_sort = lines
            .iter()
            .take(empty_line_idx)
            .map(|l| {
                let range = l.splitn(2, '-').collect::<Vec<_>>();
                IdRange::from(range)
            })
            .collect::<Vec<_>>();

        ranges_to_sort.sort();
        ranges_to_sort
    };

    let count = lines
        .iter()
        .skip(empty_line_idx)
        .filter_map(|line| line.parse::<u64>().ok())
        .filter(|n| ranges.iter().any(|r| r.contains(*n)))
        .count();

    println!("{}", count);
    Ok(())
}
