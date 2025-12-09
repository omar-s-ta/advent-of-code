use std::io::BufRead;

const MOD: u16 = 100;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Default)]
struct Tracker {
    value: u16,
    result: u16,
}

impl Tracker {
    fn new() -> Self {
        Self {
            value: 50,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    fn advance_pt_one(&mut self, dir: Direction, offset: u16) {
        let offset = offset.rem_euclid(MOD);
        match dir {
            Direction::Left => self.sub(offset),
            Direction::Right => self.add(offset),
        }
    }

    fn advance_pt_two(&mut self, dir: Direction, offset: u16) {
        let (rem, cycles) = (offset % MOD, offset / MOD);
        match dir {
            Direction::Left => self.sub_pt_two(rem, cycles),
            Direction::Right => self.add_pt_two(rem, cycles),
        }
    }

    #[inline]
    fn add(&mut self, offset: u16) {
        self.value = (self.value + offset).rem_euclid(MOD);
        self.tick_on_zero();
    }

    #[inline]
    fn sub(&mut self, offset: u16) {
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
    fn add_pt_two(&mut self, rem: u16, cycles: u16) {
        self.value += rem;
        if self.value >= MOD {
            self.value -= MOD;
            self.result += 1;
        }
        self.result += cycles;
    }

    #[inline]
    fn sub_pt_two(&mut self, rem: u16, cycles: u16) {
        if self.value > 0 && self.value <= rem {
            self.result += 1;
        }
        self.value = (self.value + MOD - rem).rem_euclid(MOD);
        self.result += cycles;
    }
}

fn main() -> std::io::Result<()> {
    let tracker = std::io::BufReader::new(std::fs::File::open("src/in.txt")?)
        .lines()
        .map_while(Result::ok)
        .fold(Tracker::new(), |mut tracker, cmd| {
            let (dir_str, offset_str) = cmd.split_at(1);
            Direction::from_str(dir_str)
                .and_then(|dir| offset_str.parse::<u16>().ok().map(|offset| (dir, offset)))
                .iter()
                .for_each(|(dir, offset)| tracker.advance_pt_two(*dir, *offset));
            tracker
        });

    println!("{}", tracker.result);
    Ok(())
}
