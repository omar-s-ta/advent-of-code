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

    pub fn advance_pt_one(&mut self, dir: &str, offset: &u16) {
        match dir {
            "L" => self.sub(&offset.rem_euclid(MOD)),
            "R" => self.add(&offset.rem_euclid(MOD)),
            _ => panic!("no good"),
        }
    }

    fn add(&mut self, offset: &u16) {
        self.value = (self.value + offset).rem_euclid(MOD);
        self.tick_on_zero();
    }

    #[inline]
    fn sub(&mut self, offset: &u16) {
        self.value = (self.value + MOD - offset).rem_euclid(MOD);
        self.tick_on_zero();
    }

    #[inline]
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
    reader.lines().for_each(|line| {
        line.iter().for_each(|cmd| {
            let (dir, offset) = cmd.split_at(1);
            offset
                .parse::<u16>()
                .iter()
                .for_each(|u| tracker.advance_pt_one(dir, u));
        });
    });
    println!("{}", tracker.result);

    Ok(())
}
