#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::fallible_impl_from)]
use std::cmp::Ordering;

use rayon::prelude::*;

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

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',').map(|s| s.parse::<u32>().unwrap());
        Self {
            x: parts.next().unwrap(),
            y: parts.next().unwrap(),
            z: parts.next().unwrap(),
        }
    }
}

struct UnionFind {
    parent: Vec<u16>,
    rank: Vec<u16>,
    size: Vec<u16>,
}

impl UnionFind {
    fn new(n: u16) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n as usize],
            size: vec![1; n as usize],
        }
    }

    fn find(&mut self, x: u16) -> u16 {
        if self.parent[x as usize] != x {
            self.parent[x as usize] = self.find(self.parent[x as usize]);
        }
        self.parent[x as usize]
    }

    fn union(&mut self, x: u16, y: u16) -> Option<u16> {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return None;
        }

        let (child, parent) = match self.rank[root_x as usize].cmp(&self.rank[root_y as usize]) {
            Ordering::Less => (root_x, root_y),
            Ordering::Greater => (root_y, root_x),
            Ordering::Equal => {
                self.rank[root_x as usize] += 1;
                (root_y, root_x)
            }
        };
        self.parent[child as usize] = parent;
        self.size[parent as usize] += self.size[child as usize];
        Some(self.size[parent as usize])
    }
}

/// separate for the test
fn solve(input: &str, num_connections: usize) -> usize {
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    let n = points.len() as u16;

    let mut edges = (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .map(|(i, j)| (points[i as usize].dist_squared(&points[j as usize]), i, j))
        .collect::<Vec<_>>();
    edges.par_sort_unstable(); // unstable = faaast

    let mut uf = UnionFind::new(n);
    for &(_, i, j) in edges.iter().take(num_connections) {
        uf.union(i, j);
    }

    let mut sizes = (0..n)
        .filter_map(|i| (uf.find(i) == i).then_some(uf.size[i as usize] as usize))
        .collect::<Vec<_>>();
    sizes.par_sort_unstable();

    sizes.iter().rev().take(3).product()
}

#[tracing::instrument(skip_all)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 171503
    solve(input, 1000)
}

#[tracing::instrument(skip_all)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = 9069509600
    let points: Vec<Point> = input.lines().map(Point::from).collect();
    let n = points.len() as u16;
    let mut edges = (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .map(|(i, j)| (points[i as usize].dist_squared(&points[j as usize]), i, j))
        .collect::<Vec<_>>();
    edges.par_sort_unstable(); // unstable = faaast

    let mut uf = UnionFind::new(n);
    let (i, j) = edges
        .iter()
        .filter_map(|&(_, i, j)| uf.union(i, j).map(|size| (size, i, j)))
        .find(|&(size, _, _)| size == n)
        .map(|(_, i, j)| (i, j))
        .unwrap();
    points[i as usize].x as usize * points[j as usize].x as usize
}

#[cfg(test)]
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

#[test]
fn part1_examples() {
    assert_eq!(solve(EXAMPLE, 10), 40);
}

#[test]
fn part2_examples() {
    assert_eq!(solve_part2(EXAMPLE), 25272);
}
