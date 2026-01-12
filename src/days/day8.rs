use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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

fn solution1<const N: usize>(input: &str) -> u64 {
    let points = parse_points(input);
    // find the distance between each pair of points and add to a binary heap
    let mut closest_pairs = BinaryHeap::new();
    for i in 0..points.len() - 1 {
        let (&[cx, cy, cz], remaining) = points[i..].split_first().unwrap();
        for &[x, y, z] in remaining {
            let distance = (cx - x).pow(2) + (cy - y).pow(2) + (cz - z).pow(2);
            closest_pairs.push((Reverse(distance), [cx, cy, cz], [x, y, z]));
        }
    }
    let closest_pairs = std::iter::from_fn(move || closest_pairs.pop()).take(N);

    // use a crude disjoint-set to determine the circuits
    fn find(parent: &mut [usize], idx: usize) -> usize {
        if parent[idx] != idx {
            parent[idx] = find(parent, parent[idx]);
        }
        parent[idx]
    }
    let mut circuit = Vec::from_iter(0..points.len());
    let mut pt_to_circuit_idx = HashMap::with_capacity(points.len());

    for (_, a, b) in closest_pairs {
        let len = pt_to_circuit_idx.len();
        let circuit_idx_a = *pt_to_circuit_idx.entry(a).or_insert(len);
        let len = pt_to_circuit_idx.len();
        let circuit_idx_b = *pt_to_circuit_idx.entry(b).or_insert(len);

        let root_a = find(&mut circuit, circuit_idx_a);
        let root_b = find(&mut circuit, circuit_idx_b);
        if root_a == root_b {
            // they're already in the same circuit
            continue;
        }
        circuit[root_a] = root_b;
    }

    let mut sizes: HashMap<usize, u64> = HashMap::new();
    for idx in 0..circuit.len() {
        let root_idx = find(&mut circuit, idx);
        *sizes.entry(root_idx).or_default() += 1;
    }
    let mut largest = BinaryHeap::from_iter(sizes.values());
    largest.pop().unwrap() * largest.pop().unwrap() * largest.pop().unwrap()
}

fn solution2(input: &str) -> i64 {
    let points = parse_points(input);
    // find the distance between each pair of points and add to a binary heap
    let mut closest_pairs = BinaryHeap::new();
    for i in 0..points.len() - 1 {
        let (&[cx, cy, cz], remaining) = points[i..].split_first().unwrap();
        for &[x, y, z] in remaining {
            let distance = (cx - x).pow(2) + (cy - y).pow(2) + (cz - z).pow(2);
            closest_pairs.push((distance, [cx, cy, cz], [x, y, z]));
        }
    }
    let closest_pairs = closest_pairs.into_sorted_vec();

    // use a crude disjoint-set to determine the circuits
    let mut circuit = Vec::from_iter(0..points.len());
    let mut pt_to_circuit_idx = HashMap::with_capacity(points.len());
    let mut last_pair_to_join = None;
    for (_, a, b) in closest_pairs {
        fn find(parent: &mut [usize], idx: usize) -> usize {
            if parent[idx] != idx {
                parent[idx] = find(parent, parent[idx]);
            }
            parent[idx]
        }
        let len = pt_to_circuit_idx.len();
        let circuit_idx_a = *pt_to_circuit_idx.entry(a).or_insert(len);
        let len = pt_to_circuit_idx.len();
        let circuit_idx_b = *pt_to_circuit_idx.entry(b).or_insert(len);

        let root_a = find(&mut circuit, circuit_idx_a);
        let root_b = find(&mut circuit, circuit_idx_b);
        if root_a == root_b {
            // they're already in the same circuit
            continue;
        }
        circuit[root_a] = root_b;
        last_pair_to_join = Some((a, b));
    }

    let (a, b) = last_pair_to_join.unwrap();
    a[0] * b[0]
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
