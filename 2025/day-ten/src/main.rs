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

mod two {
    struct Manual {
        matrix: Vec<Vec<i32>>,
        rows: usize,
        cols: usize,
        bound: i32,
    }

    impl Manual {
        fn presses(&mut self) -> i32 {
            let pivots = self.gaussian_elimination();
            let free_vars = (0..self.cols - 1)
                .filter(|c| !pivots.contains(c))
                .collect::<Vec<_>>();

            let mut result = i32::MAX;
            let mut solution = vec![0; self.cols - 1];

            self.search(0, 0, &free_vars, &pivots, &mut solution, &mut result);
            result
        }

        fn gaussian_elimination(&mut self) -> Vec<usize> {
            let mut pivots = Vec::new();
            let mut pivot_row = 0;

            for col in 0..self.cols - 1 {
                let Some(row) = (pivot_row..self.rows).find(|&i| self.matrix[i][col] != 0) else {
                    continue;
                };

                self.matrix.swap(pivot_row, row);
                pivots.push(col);

                for r in (pivot_row + 1)..self.rows {
                    if self.matrix[r][col] != 0 {
                        self.eliminate_col(col, r, pivot_row);
                        self.reduce_row(r);
                    }
                }

                pivot_row += 1;
                if pivot_row >= self.rows {
                    break;
                }
            }
            pivots
        }

        fn eliminate_col(&mut self, col: usize, row: usize, pivot_row: usize) {
            let (a, b) = (self.matrix[pivot_row][col], self.matrix[row][col]);
            let g = gcd(a, b);
            let (a, b) = (a / g, b / g);

            self.matrix[row] = self.matrix[row]
                .iter()
                .zip(&self.matrix[pivot_row])
                .map(|(&r, &p)| r * a - p * b)
                .collect();
        }

        fn reduce_row(&mut self, row: usize) {
            let g = self.matrix[row]
                .iter()
                .copied()
                .filter(|&v| v != 0)
                .fold(0, gcd);

            if g > 1 {
                self.matrix[row].iter_mut().for_each(|v| *v /= g);
            }
        }

        fn substitue(&self, pivots: &[usize], solution: &mut [i32]) -> bool {
            for (r, &col) in pivots.iter().enumerate().rev() {
                let rhs = self.matrix[r]
                    .last()
                    .map(|&b| {
                        b - ((col + 1)..self.cols - 1)
                            .map(|c| self.matrix[r][c] * solution[c])
                            .sum::<i32>()
                    })
                    .unwrap();

                let coefficient = self.matrix[r][col];
                if rhs % coefficient != 0 {
                    return false;
                }
                let value = rhs / coefficient;
                if value < 0 {
                    return false;
                }
                solution[col] = value;
            }
            true
        }

        fn search(
            &self,
            free_at: usize,
            free_sum: i32,
            free_vars: &[usize],
            pivots: &[usize],
            solution: &mut [i32],
            result: &mut i32,
        ) {
            if free_sum >= *result {
                return;
            }
            if free_at == free_vars.len() {
                if self.substitue(pivots, solution) {
                    *result = solution.iter().sum::<i32>().min(*result);
                }
            } else {
                (0..=self.bound).for_each(|value| {
                    solution[free_vars[free_at]] = value;
                    self.search(
                        free_at + 1,
                        free_sum + value,
                        free_vars,
                        pivots,
                        solution,
                        result,
                    );
                })
            }
        }
    }

    impl From<&crate::Machine> for Manual {
        fn from(value: &crate::Machine) -> Self {
            let descriptions = value.split_whitespace().collect::<Vec<_>>();
            let buttons = descriptions[1..descriptions.len() - 1]
                .iter()
                .map(|button| {
                    button[1..button.len() - 1]
                        .split(',')
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let joltage = descriptions
                .last()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .filter_map(|v| v.parse::<i32>().ok())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            let bound = joltage.iter().max().copied().unwrap_or_default();

            let rows = joltage.len();
            let cols = buttons.len() + 1;
            let matrix = (0..rows)
                .map(|i| {
                    buttons
                        .iter()
                        .map(|button| i32::from(button.contains(&i)))
                        .chain(std::iter::once(joltage[i]))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            // [A | joltage] matrix
            Manual {
                matrix,
                rows,
                cols,
                bound,
            }
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        match (a.abs(), b.abs()) {
            (x, 0) => x,
            (0, y) => y,
            (x, y) => gcd(y, x % y),
        }
    }

    pub(crate) fn solve(machines: &[crate::Machine]) -> i32 {
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
    println!("{}", two::solve(&machines));
    Ok(())
}
