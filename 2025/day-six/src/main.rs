use std::io::BufRead;

#[derive(Debug)]
struct Matrix {
    value: Vec<Vec<String>>,
}

impl Matrix {
    fn new(value: Vec<Vec<String>>) -> Self {
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

    fn compute_pt_one(&self, ops: &[String]) -> usize {
        self.value
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let parsed = row.iter().filter_map(|s| s.parse::<usize>().ok());
                if ops[i] == "*" {
                    parsed.product::<usize>()
                } else {
                    parsed.sum::<usize>()
                }
            })
            .sum()
    }
}

fn main() -> std::io::Result<()> {
    let f = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(f);
    let lines = reader.lines().map_while(Result::ok).collect::<Vec<_>>();
    let ops = lines
        .last()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
        })
        .expect("operations row");

    {
        let matrix = lines
            .iter()
            .map(|l| {
                l.split_whitespace()
                    .map(|s| s.to_owned())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let matrix = Matrix::new(matrix).transpose();
        println!("{}", matrix.compute_pt_one(&ops));
    }

    Ok(())
}
