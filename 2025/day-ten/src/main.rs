use std::io::BufRead;

type Machine = String;

mod one {
    struct IndicatorLight(u32);
    struct Button(u32);

    struct Manual {
        target: IndicatorLight,
        buttons: Vec<Button>,
    }

    impl Manual {
        fn presses(&self) -> u32 {
            let n = self.buttons.len();
            (0_u32..(1 << n))
                .filter_map(|mask| {
                    let pressed = self
                        .buttons
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << i) != 0)
                        .fold(0, |mask, (_, button)| mask ^ button.0);

                    (pressed == self.target.0).then_some(mask.count_ones())
                })
                .min()
                .unwrap_or_default()
        }
    }

    impl From<&crate::Machine> for Manual {
        fn from(value: &crate::Machine) -> Self {
            let descriptions = value.split_whitespace().collect::<Vec<_>>();
            let target = IndicatorLight::from(descriptions[0]);
            let buttons = descriptions[1..descriptions.len() - 1]
                .iter()
                .copied()
                .map(Button::from)
                .collect::<Vec<_>>();

            Manual { target, buttons }
        }
    }

    impl From<&str> for IndicatorLight {
        fn from(value: &str) -> Self {
            let mask = value[1..value.len() - 1]
                .chars()
                .enumerate()
                .filter(|&(_, ch)| ch == '#')
                .fold(0, |mask, (i, _)| mask | (1 << i));

            IndicatorLight(mask)
        }
    }

    impl From<&str> for Button {
        fn from(value: &str) -> Self {
            let mask = value[1..value.len() - 1]
                .chars()
                .filter_map(|c| c.to_digit(10))
                .fold(0, |mask, i| mask | (1 << i));

            Button(mask)
        }
    }

    pub(crate) fn solve(machines: &[crate::Machine]) -> u32 {
        machines.iter().map(|m| Manual::from(m).presses()).sum()
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);
    let machines = reader
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<Machine>>();

    println!("{}", one::solve(&machines));
    Ok(())
}
