use std::io::BufRead;

struct Matrix {
    value: Vec<Vec<String>>,
}

impl Matrix {
    fn from_lines(lines: &[&String]) -> Self {
        let value = lines
            .iter()
            .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
            .collect();

        Matrix { value }
    }

    fn as_chars(lines: &[&String]) -> Self {
        let value = lines
            .iter()
            .map(|r| r.chars().map(|c| c.to_string()).collect())
            .collect();

        Matrix { value }
    }

    fn transpose(&self) -> Self {
        let n = self.value.len();
        let m = self.value.first().map_or(0, |c| c.len());
        let value = (0..m)
            .map(|j| (0..n).map(|i| self.value[i][j].clone()).collect())
            .collect();

        Matrix { value }
    }

    fn compute_pt_one(&self, ops: &[&str]) -> usize {
        self.value
            .iter()
            .zip(ops)
            .map(|(row, &op)| {
                Self::aggregate(row.iter().filter_map(|s| s.parse::<usize>().ok()), op)
            })
            .sum()
    }

    fn compute_pt_two(&self, ops: &[&str]) -> usize {
        self.value
            .split(|r| r.iter().all(|c| c == " "))
            .zip(ops)
            .map(|(row, &op)| {
                Self::aggregate(
                    row.iter()
                        .filter_map(|col| col.concat().trim().parse::<usize>().ok()),
                    op,
                )
            })
            .sum()
    }

    fn aggregate<I>(values: I, op: &str) -> usize
    where
        I: Iterator<Item = usize>,
    {
        match op {
            "+" => values.sum(),
            _ => values.product(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
    let (ops, lines) = lines.split_last().expect("operations row");
    let ops = ops.split_whitespace().collect::<Vec<_>>();
    let lines = lines.iter().collect::<Vec<_>>();

    let one = Matrix::from_lines(&lines).transpose().compute_pt_one(&ops);
    let two = Matrix::as_chars(&lines).transpose().compute_pt_two(&ops);

    println!("{} {}", one, two);
    Ok(())
}
