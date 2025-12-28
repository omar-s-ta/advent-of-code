use std::io::BufRead;

const CLOSEST: usize = 1000;
const BIGGEST: usize = 3;

struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        DisjointSet {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, set: usize) -> usize {
        if self.parent[set] != set {
            self.parent[set] = self.find(self.parent[set]);
        }
        self.parent[set]
    }

    fn is_same_set(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn len(&mut self, set: usize) -> usize {
        let set = self.find(set);
        self.size[set]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return;
        }
        if self.rank[a] > self.rank[b] {
            std::mem::swap(&mut a, &mut b);
        }
        if self.rank[a] == self.rank[b] {
            self.rank[b] += 1;
        }
        self.parent[a] = b;
        self.size[b] += self.size[a];
    }
}

#[derive(Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn dist_square(&self, other: Position) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl From<String> for Position {
    fn from(value: String) -> Self {
        let parsed = value
            .split(',')
            .map_while(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>();

        Position {
            x: parsed[0],
            y: parsed[1],
            z: parsed[2],
        }
    }
}

#[derive(Debug)]
struct Edge {
    i: usize,
    j: usize,
    dist: i64,
}

impl Edge {
    fn new(i: usize, j: usize, dist: i64) -> Self {
        Edge { i, j, dist }
    }

    fn edges(positions: &[Position]) -> Vec<Edge> {
        let n = positions.len();
        (0..n)
            .flat_map(|i| {
                ((i + 1)..n)
                    .map(move |j| Edge::new(i, j, positions[i].dist_square(positions[j])))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("src/in.txt")?;
    let reader = std::io::BufReader::new(file);
    let positions = reader
        .lines()
        .map_while(Result::ok)
        .map(Position::from)
        .collect::<Vec<_>>();

    let mut edges = Edge::edges(&positions);
    edges.sort_by(|a, b| a.dist.cmp(&b.dist));

    let n = positions.len();
    let mut set = DisjointSet::new(n);
    edges.iter().take(CLOSEST).for_each(|e| set.union(e.i, e.j));

    let mut sizes = (0..n).map(|i| set.len(i)).collect::<Vec<_>>();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.dedup();

    println!("{}", sizes.iter().take(BIGGEST).product::<usize>());
    Ok(())
}
