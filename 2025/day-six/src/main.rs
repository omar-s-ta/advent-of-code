use std::{borrow::Cow, io::BufRead};

#[derive(Debug)]
struct Matrix<'a> {
    value: Vec<Vec<Cow<'a, str>>>,
}

impl<'a> Matrix<'a> {
    fn from_lines(lines: &'a [&String]) -> Self {
        let value = lines
            .iter()
            .map(|l| l.split_whitespace().map(Cow::Borrowed).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Matrix { value }
    }

    fn as_chars(lines: &'a [&String]) -> Self {
        let value = lines
            .iter()
            .map(|r| r.chars().map(|c| Cow::Owned(c.to_string())).collect())
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

    fn compute_pt_two(&self, ops: &[&str]) -> usize {
        self.value
            .split(|r| r.iter().all(|c| c == " "))
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let values = row
                    .iter()
                    .filter_map(|col| col.join("").trim().parse::<usize>().ok());

                if ops[i] == "*" {
                    values.product::<usize>()
                } else {
                    values.sum::<usize>()
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
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .expect("operations row");

    let lines = lines.iter().take(lines.len() - 1).collect::<Vec<_>>();
    {
        let matrix = Matrix::from_lines(&lines).transpose();
        println!("{}", matrix.compute_pt_one(&ops));
    }
    {
        let matrix = Matrix::as_chars(&lines).transpose();
        println!("{}", matrix.compute_pt_two(&ops))
    }
    Ok(())
}
