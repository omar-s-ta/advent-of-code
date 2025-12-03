use std::io::BufRead;

const MOD: u16 = 100;

struct Tracker {
    value: u16,
    result: u16,
}

impl Tracker {
    pub fn new() -> Self {
        Tracker {
            value: 50,
            result: 0,
        }
    }

    pub fn advance(&mut self, dir: &str, offset: u16) {
        let _: () = match dir {
            "L" => self.value = (self.value + MOD - offset).rem_euclid(MOD),
            "R" => self.value = (self.value + offset).rem_euclid(MOD),
            _ => panic!("no good"),
        };
        self.tick_on_zero()
    }

    fn tick_on_zero(&mut self) {
        if self.value == 0 {
            self.result += 1;
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);

    let mut tracker = Tracker::new();
    for line in reader.lines() {
        let line = line?;
        let (dir, offset) = line.split_at(1);
        offset
            .parse::<u16>()
            .iter()
            .for_each(|&u| tracker.advance(dir, u.rem_euclid(MOD)));
    }
    println!("{}", tracker.result);
    Ok(())
}
