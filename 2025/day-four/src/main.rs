use std::{collections::VecDeque, io::BufRead};

struct Grid {
    path: Vec<String>,
    rows: usize,
    cols: usize,
    taken: Vec<Vec<bool>>,
}

impl Grid {
    fn new(grid: Vec<String>) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        Grid {
            path: grid,
            rows: n,
            cols: m,
            taken: vec![vec![false; m]; n],
        }
    }

    #[allow(dead_code)]
    fn accessible_rolls_pt_one(&self) -> usize {
        self.path
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.char_indices()
                    .filter(|(j, c)| *c == '@' && self.accessible(i, *j))
                    .count()
            })
            .sum::<usize>()
    }

    fn accessible_rolls_pt_two(&mut self) -> usize {
        let mut in_degrees = self.in_degrees();
        let mut queue = VecDeque::new();

        for (i, row) in in_degrees.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                if self.is_roll_of_paper(i, j) && *value < 4 {
                    queue.push_back((i, j));
                }
            }
        }

        let mut removed = 0;
        while let Some((x, y)) = queue.pop_front() {
            if self.taken[x][y] {
                continue;
            }
            self.taken[x][y] = true;
            removed += 1;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let nx = (x as i64 + i) as usize;
                    let ny = (y as i64 + j) as usize;
                    if self.is_roll_of_paper(nx, ny) {
                        in_degrees[nx][ny] = in_degrees[nx][ny].wrapping_sub(1);
                        if in_degrees[nx][ny] < 4 {
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }
        removed
    }

    fn adjacent_counts(&self, x: usize, y: usize) -> usize {
        if !self.is_roll_of_paper(x, y) {
            0
        } else {
            let mut count = 0;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let nx = (x as i64 + i) as usize;
                    let ny = (y as i64 + j) as usize;
                    if self.is_roll_of_paper(nx, ny) {
                        count += 1;
                    }
                }
            }
            count
        }
    }

    fn in_degrees(&self) -> Vec<Vec<usize>> {
        let mut degrees = vec![vec![0; self.cols]; self.rows];
        for (i, row) in degrees.iter_mut().enumerate() {
            for (j, degree) in row.iter_mut().enumerate() {
                *degree += self.adjacent_counts(i, j);
            }
        }
        degrees
    }

    fn accessible(&self, x: usize, y: usize) -> bool {
        self.adjacent_counts(x, y) < 4
    }

    fn is_roll_of_paper(&self, x: usize, y: usize) -> bool {
        x < self.rows && self.path[x].chars().nth(y).is_some_and(|c| c == '@')
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);
    let grid = reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<_>>();

    let mut grid = Grid::new(grid);
    println!("{}", grid.accessible_rolls_pt_two());
    Ok(())
}
