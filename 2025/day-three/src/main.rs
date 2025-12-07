use std::io::BufRead;

struct BatteryBank {
    value: String,
}

impl BatteryBank {
    fn max_joltage_pt_one(&self) -> u32 {
        self.value
            .char_indices()
            .max_by_key(|(_, ch)| *ch)
            .map(|(i, ch)| {
                let (l, r) = self.value.split_at(i);
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
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);
    let joltage = reader
        .lines()
        .map(|line| {
            line.map(|bank| BatteryBank { value: bank })
                .map(|bank| bank.max_joltage_pt_one())
                .unwrap_or(0)
        })
        .sum::<u32>();

    println!("{}", joltage);
    Ok(())
}
