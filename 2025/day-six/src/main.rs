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

    fn count(&self) -> usize {
        self.value
            .iter()
            .map(|r| {
                let parsed = r
                    .iter()
                    .take(r.len() - 1)
                    .filter_map(|s| s.parse::<usize>().ok());

                match r.last() {
                    Some(s) if s == "*" => parsed.product(),
                    Some(s) if s == "+" => parsed.sum(),
                    _ => 0,
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

    let matrix = Matrix::new(matrix);

    // part-one
    {
        let matrix = matrix.transpose();
        println!("{}", matrix.count());
    }

    Ok(())
}
