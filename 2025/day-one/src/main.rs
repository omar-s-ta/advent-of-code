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

    #[allow(dead_code)]
    pub fn advance_pt_one(&mut self, dir: &str, offset: &u16) {
        match dir {
            "L" => self.sub(&offset.rem_euclid(MOD)),
            "R" => self.add(&offset.rem_euclid(MOD)),
            _ => panic!("no good"),
        }
    }

    pub fn advance_pt_two(&mut self, dir: &str, offset: &u16) {
        let (rem, cycles) = (offset % MOD, offset / MOD);
        match dir {
            "L" => self.sub_pt_two(&rem, &cycles),
            "R" => self.add_pt_two(&rem, &cycles),
            _ => panic!("no good"),
        }
    }

    #[inline]
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

    #[inline]
    fn add_pt_two(&mut self, rem: &u16, cycles: &u16) {
        self.value += rem;
        if self.value >= MOD {
            self.value -= MOD;
            self.result += 1;
        }
        self.result += cycles;
    }

    #[inline]
    fn sub_pt_two(&mut self, rem: &u16, cycles: &u16) {
        if self.value > 0 && self.value <= *rem {
            self.result += 1;
        }
        self.value = (self.value + MOD - rem).rem_euclid(MOD);
        self.result += cycles;
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
                .for_each(|u| tracker.advance_pt_two(dir, u));
        });
    });
    println!("{}", tracker.result);

    Ok(())
}
