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
        let mut value = vec![vec!["".to_owned(); n]; m];
        (0..n).for_each(|i| {
            (0..m).for_each(|j| {
                value[j][i] = self.value[i][j].clone();
            });
        });
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

    let lines = reader
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<_>>()
        .into_iter()
        .map(|l| l.trim().to_owned())
        .collect::<Vec<_>>();

    let matrix = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.trim().to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (matrix, ops) = matrix.split_at(matrix.len() - 1);
    let ops = ops.first().expect("one row");
    let matrix = Matrix::new(matrix.to_owned());

    {
        let matrix = matrix.transpose();
        println!("{}", matrix.compute_pt_one(ops));
    }

    Ok(())
}
