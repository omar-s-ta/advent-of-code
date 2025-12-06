use std::{ops::RangeInclusive, panic};

struct IdRange {
    range: RangeInclusive<u64>,
}

impl IdRange {
    fn new(b: &u64, e: &u64) -> Self {
        IdRange {
            range: RangeInclusive::new(*b, *e),
        }
    }

    fn empty() -> Self {
        IdRange {
            range: RangeInclusive::new(1, 0),
        }
    }

    #[allow(dead_code)]
    fn invalids_count_pt_one(self) -> u64 {
        self.range
            .filter(is_even_size)
            .filter(is_invalid_pt_one)
            .sum()
    }

    fn invalids_count_pt_two(self) -> u64 {
        self.range.filter(is_invalid_pt_two).sum()
    }
}

impl From<(&str, &str)> for IdRange {
    fn from(value: (&str, &str)) -> Self {
        let b = value
            .0
            .trim()
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("Failed to parse {}", value.0));
        let e = value
            .1
            .trim()
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("Failed to parse {}", value.1));
        IdRange::new(&b, &e)
    }
}

fn main() -> std::io::Result<()> {
    let invalids = std::fs::read_to_string("src/in.txt").map(|ranges| {
        ranges
            .split(',')
            .map(|range| {
                range
                    .split_once('-')
                    .map(IdRange::from)
                    .unwrap_or_else(IdRange::empty)
                    .invalids_count_pt_two()
            })
            .sum::<u64>()
    })?;
    println!("{}", invalids);
    Ok(())
}

fn is_even_size(n: &u64) -> bool {
    !n.ilog10().is_multiple_of(2)
}

fn is_invalid_pt_one(n: &u64) -> bool {
    let size = n.ilog10() + 1;
    let d = 10_u64.pow(size / 2);
    (n / d) == (n % d)
}

fn can_be_divided(mut n: u64, divisor: u64) -> bool {
    let bucket = n % divisor;
    while n != 0 {
        let next_bucket = n % divisor;
        if bucket != next_bucket {
            return false;
        }
        n /= divisor;
    }
    true
}

fn is_invalid_pt_two(n: &u64) -> bool {
    let size = n.ilog10() + 1;
    match size {
        1 => false,
        2 | 3 | 5 | 7 => can_be_divided(*n, 10),
        _ => (1..size)
            .filter(|&i| size.is_multiple_of(i))
            .any(|d| can_be_divided(*n, 10_u64.pow(d))),
    }
}
