use std::{io::BufRead, ops::RangeInclusive};

#[derive(PartialEq, Eq, Debug)]
struct IdRange {
    value: RangeInclusive<u64>,
}

impl IdRange {
    fn new(b: u64, e: u64) -> Self {
        IdRange {
            value: RangeInclusive::new(b, e),
        }
    }

    fn contains(&self, elem: u64) -> bool {
        self.value.contains(&elem)
    }
}

impl PartialOrd for IdRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IdRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.value.start().cmp(other.value.start()) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.value.end().cmp(other.value.end())
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
    let mut lines = std::io::BufReader::new(file).lines().map_while(Result::ok);

    let mut ranges = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let range = line.splitn(2, '-').collect::<Vec<_>>();
            IdRange::from(range)
        })
        .collect::<Vec<_>>();

    ranges.sort();

    let count = lines
        .filter_map(|line| line.parse::<u64>().ok())
        .filter(|n| ranges.iter().any(|r| r.contains(*n)))
        .count();

    println!("{}", count);
    Ok(())
}
