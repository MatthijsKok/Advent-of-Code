use std::cmp::Ordering;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Point {
    /// `sqrt()` is not needed, can still sort on squared distances.
    /// And then we can stay in integer land and don't have to deal with the garbage datatype known as floats.
    pub fn dist_squared(&self, other: &Self) -> u64 {
        let dx = u64::from(self.x.abs_diff(other.x));
        let dy = u64::from(self.y.abs_diff(other.y));
        let dz = u64::from(self.z.abs_diff(other.z));
        dx * dx + dy * dy + dz * dz
    }
}

impl TryFrom<&str> for Point {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut parts = s.split(',');
        Ok(Self {
            x: parts.next().ok_or(())?.parse().map_err(|_| ())?,
            y: parts.next().ok_or(())?.parse().map_err(|_| ())?,
            z: parts.next().ok_or(())?.parse().map_err(|_| ())?,
        })
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        match self.rank[root_x].cmp(&self.rank[root_y]) {
            Ordering::Less => {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            Ordering::Greater => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            Ordering::Equal => {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.rank[root_x] += 1;
            }
        }
        true
    }
}

/// has to be a separate function otherways rust starts crying about re-borrowing mo
fn all_pairs(points: &[Point]) -> impl Iterator<Item = (u64, usize, usize)> {
    let n = points.len();
    (0..n).flat_map(move |i| (i + 1..n).map(move |j| (points[i].dist_squared(&points[j]), i, j)))
}

/// separate for the test
fn solve(input: &str, num_connections: usize) -> usize {
    let points: Vec<Point> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Point::try_from(line).unwrap())
        .collect();

    let mut edges = all_pairs(&points).collect::<Vec<_>>();
    edges.sort_unstable(); // unstable = faaast

    let mut uf = UnionFind::new(points.len());
    for &(_, i, j) in edges.iter().take(num_connections) {
        uf.union(i, j);
    }

    let mut sizes = (0..uf.parent.len())
        .filter_map(|i| (uf.find(i) == i).then_some(uf.size[i]))
        .collect::<Vec<_>>();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes.iter().take(3).product()
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 171503
    solve(input, 1000)
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = ?
    0
}

#[test]
fn part1_examples() {
    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    assert_eq!(solve(EXAMPLE, 10), 40);
}

#[test]
fn part2_examples() {}
