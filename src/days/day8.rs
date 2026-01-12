use std::{cmp::Reverse, collections::BinaryHeap};

const INPUT: &str = include_str!("input/day8.txt");

pub fn part1() {
    println!("{}", solution1::<1000>(INPUT));
}

pub fn part2() {
    println!("{}", solution2(INPUT));
}

fn parse_points(input: &str) -> Vec<[i64; 3]> {
    input
        .lines()
        .map(|line| {
            let (x, rest) = line.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()]
        })
        .collect()
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
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

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    fn get_sizes(&self) -> impl Iterator<Item = usize> {
        (0..self.parent.len())
            .filter(|&i| self.parent[i] == i)
            .map(|root| self.size[root])
    }
}

fn solution1<const N: usize>(input: &str) -> usize {
    let points = parse_points(input);
    let n = points.len();

    // find the distance squared between each pair of points and add their indices
    // to a max heap sorted in reverse by the distance squared
    let mut closest_pairs = BinaryHeap::with_capacity(n * (n - 1) / 2);
    for i in 0..n - 1 {
        let [cx, cy, cz] = points[i];
        for j in i + 1..n {
            let [x, y, z] = points[j];
            let dist_sq = (cx - x).pow(2) + (cy - y).pow(2) + (cz - z).pow(2);
            closest_pairs.push((Reverse(dist_sq), i, j));
        }
    }

    let mut uf = UnionFind::new(n);

    // process only the N closest pairs
    for _ in 0..N {
        if let Some((_, i, j)) = closest_pairs.pop() {
            uf.union(i, j);
        }
    }

    let mut sizes: Vec<_> = uf.get_sizes().map(Reverse).collect();
    sizes.sort_unstable();
    sizes[0].0 * sizes[1].0 * sizes[2].0
}

fn solution2(input: &str) -> i64 {
    let points = parse_points(input);
    let n = points.len();

    // find the distance squared between each point and then sort with max distance
    // squared last
    let mut pairs = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n - 1 {
        let [cx, cy, cz] = points[i];
        for j in i + 1..n {
            let [x, y, z] = points[j];
            let dist_sq = (cx - x).pow(2) + (cy - y).pow(2) + (cz - z).pow(2);
            pairs.push((dist_sq, i, j));
        }
    }
    pairs.sort_unstable();

    let mut uf = UnionFind::new(n);
    let mut last_pair = None;

    for (_, i, j) in pairs {
        if uf.union(i, j) {
            last_pair = Some((i, j));
        }
    }

    let (i, j) = last_pair.unwrap();
    points[i][0] * points[j][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input/example8.txt");

    #[test]
    fn example1() {
        assert_eq!(solution1::<10>(INPUT), 40);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 25272);
    }
}
