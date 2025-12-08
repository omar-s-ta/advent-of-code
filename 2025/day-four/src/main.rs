use std::io::BufRead;

struct Grid {
    path: Vec<String>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(grid: Vec<String>) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        Grid {
            path: grid,
            rows: n,
            cols: m,
        }
    }

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

    fn accessible(&self, x: usize, y: usize) -> bool {
        let count = (-1..=1)
            .map(|i| {
                (-1..=1)
                    .filter(|j| {
                        let nx = (x as i64 + i) as usize;
                        let ny = (y as i64 + j) as usize;
                        !(i == 0 && *j == 0) && self.is_roll_of_paper(nx, ny)
                    })
                    .count()
            })
            .sum::<usize>();

        count < 4
    }

    fn inside(&self, i: usize, j: usize) -> bool {
        i < self.rows && j < self.cols
    }

    fn is_roll_of_paper(&self, x: usize, y: usize) -> bool {
        self.inside(x, y) && self.path[x].chars().nth(y).is_some_and(|c| c == '@')
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
    Ok(())
}
