use std::io::BufRead;

const LIMIT: usize = 12;

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);
    let joltage = reader
        .lines()
        .map(|line| line.map(max_joltage_pt_two).unwrap_or(0))
        .sum::<usize>();

    println!("{}", joltage);
    Ok(())
}

fn max_joltage_pt_two(string: String) -> usize {
    let mut to_remove = string.len() - LIMIT;
    let mut stack = Vec::new();
    for ch in string.chars() {
        while let Some(digit) = stack.last() {
            if to_remove > 0 && ch > *digit {
                stack.pop();
                to_remove -= 1;
            } else {
                break;
            }
        }
        stack.push(ch);
    }
    stack.truncate(LIMIT);
    let mut joltage = 0;
    for d in stack {
        joltage *= 10;
        joltage += d.to_digit(10).unwrap_or_default() as usize;
    }
    joltage
}

#[allow(dead_code)]
fn max_joltage_pt_one(string: String) -> u32 {
    string
        .char_indices()
        .max_by_key(|(_, ch)| *ch)
        .map(|(i, ch)| {
            let (l, r) = string.split_at(i);
            let d = ch.to_digit(10).unwrap_or_default();
            let bl = l
                .chars()
                .max()
                .and_then(|ch| ch.to_digit(10))
                .map(|dl| dl * 10 + d)
                .unwrap_or_default();
            let br = r
                .chars()
                .skip(1)
                .max()
                .and_then(|ch| ch.to_digit(10))
                .map(|dr| d * 10 + dr)
                .unwrap_or_default();
            bl.max(br)
        })
        .unwrap_or_default()
}
