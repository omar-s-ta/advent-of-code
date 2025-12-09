use std::{collections::VecDeque, io::BufRead};

type Position = (usize, usize);

struct Grid {
    grid: Vec<String>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(grid: Vec<String>) -> Self {
        let rows = grid.len();
        let cols = grid.first().map_or(0, |r| r.len());
        Grid { grid, rows, cols }
    }

    fn is_roll_paper(&self, (x, y): Position) -> bool {
        (x < self.rows)
            .then(|| &self.grid[x])
            .and_then(|row| row.chars().nth(y))
            .is_some_and(|c| c == '@')
    }

    fn neighbor_roll_papers(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        (-1..=1)
            .flat_map(|i| (-1..=1).map(move |j| (i, j)))
            .filter(|&(i, j)| !(i == 0 && j == 0))
            .map(move |(i, j)| ((x as i64 + i) as usize, (y as i64 + j) as usize))
            .filter(|&(i, j)| self.is_roll_paper((i, j)))
    }

    fn adjacent_roll_paper_count(&self, position: Position) -> usize {
        if self.is_roll_paper(position) {
            self.neighbor_roll_papers(position).count()
        } else {
            0
        }
    }

    fn positions(&self) -> impl Iterator<Item = Position> {
        (0..self.rows).flat_map(|i| (0..self.cols).map(move |j| (i, j)))
    }

    fn in_degrees(&self) -> Vec<Vec<usize>> {
        (0..self.rows)
            .map(|i| {
                (0..self.cols)
                    .map(|j| self.adjacent_roll_paper_count((i, j)))
                    .collect()
            })
            .collect()
    }

    fn accessible_rolls_pt_one(&self) -> usize {
        self.positions()
            .filter(|&position| {
                self.is_roll_paper(position) && self.adjacent_roll_paper_count(position) < 4
            })
            .count()
    }

    fn accessible_rolls_pt_two(&self) -> usize {
        let mut in_degrees = self.in_degrees();
        let mut taken = vec![vec![false; self.cols]; self.rows];

        let queue = self
            .positions()
            .filter(|&(i, j)| {
                self.is_roll_paper((i, j)) && self.adjacent_roll_paper_count((i, j)) < 4
            })
            .collect::<VecDeque<_>>();

        self.process_queue(queue, &mut in_degrees, &mut taken)
    }

    fn process_queue(
        &self,
        mut queue: VecDeque<Position>,
        in_degrees: &mut [Vec<usize>],
        taken: &mut [Vec<bool>],
    ) -> usize {
        let mut removed = 0;
        while let Some(position @ (x, y)) = queue.pop_front() {
            if taken[x][y] {
                continue;
            }
            taken[x][y] = true;
            removed += 1;
            self.neighbor_roll_papers(position).for_each(|(i, j)| {
                in_degrees[i][j] = in_degrees[i][j].saturating_sub(1);
                if in_degrees[i][j] < 4 {
                    queue.push_back((i, j));
                }
            });
        }
        removed
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);
    let grid = reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect::<Vec<_>>();

    let grid = Grid::new(grid);
    println!("{}", grid.accessible_rolls_pt_one());
    println!("{}", grid.accessible_rolls_pt_two());
    Ok(())
}
