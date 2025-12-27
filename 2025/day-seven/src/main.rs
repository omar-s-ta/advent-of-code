use std::{collections::HashSet, io::BufRead};

type Position = (usize, usize);

enum Cell {
    Start,
    Splitter,
    Empty,
}

impl Cell {
    fn from_char(ch: char) -> Self {
        match ch {
            'S' => Cell::Start,
            '^' => Cell::Splitter,
            _ => Cell::Empty,
        }
    }

    fn is_start(&self) -> bool {
        matches!(self, Cell::Start)
    }

    fn compute(&self, (i, j): Position, cols: usize, count: &mut [Vec<usize>]) {
        match self {
            Cell::Start => count[i][j] = 1,
            Cell::Splitter => {
                count[i][j - 1] += count[i - 1][j];
                if j + 1 < cols {
                    count[i][j + 1] += count[i - 1][j];
                }
            }
            Cell::Empty if i > 0 => count[i][j] += count[i - 1][j],
            _ => count[i][j] = 0,
        }
    }
}

struct Grid {
    repr: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn from_lines(lines: &[String]) -> Self {
        let rows = lines.len();
        let cols = lines.first().map_or(0, |r| r.len());
        let repr = lines
            .iter()
            .map(|l| l.chars().map(Cell::from_char).collect())
            .collect();

        Grid { repr, rows, cols }
    }

    fn splitters_count(&self) -> usize {
        self.find_start()
            .map(|start| {
                let mut visited = HashSet::new();
                self.pt_one(start, &mut visited)
            })
            .unwrap_or_default()
    }

    fn paths_count(&self) -> usize {
        self.cells()
            .fold(vec![vec![0; self.cols]; self.rows], |mut count, (i, j)| {
                self.repr[i][j].compute((i, j), self.cols, &mut count);
                count
            })
            .last()
            .map(|row| row.iter().sum())
            .unwrap_or_default()
    }

    fn pt_one(&self, p: Position, visited: &mut HashSet<Position>) -> usize {
        if !self.contains(p) || visited.contains(&p) {
            return 0;
        }
        visited.insert(p);
        let (i, j) = p;
        match self.repr[i][j] {
            Cell::Splitter => {
                1 + self.pt_one((i + 1, j - 1), visited) + self.pt_one((i + 1, j + 1), visited)
            }
            _ => self.pt_one((i + 1, j), visited),
        }
    }

    fn find_start(&self) -> Option<Position> {
        self.cells().find(|&(i, j)| self.repr[i][j].is_start())
    }

    fn cells(&self) -> impl Iterator<Item = Position> {
        (0..self.rows).flat_map(|i| (0..self.cols).map(move |j| (i, j)))
    }

    fn contains(&self, (i, j): Position) -> bool {
        i < self.rows && j < self.cols
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
    let grid = Grid::from_lines(&lines);
    println!("{} {}", grid.splitters_count(), grid.paths_count());
    Ok(())
}
