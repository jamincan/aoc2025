const INPUT: &str = include_str!("input/day7.txt");

pub fn part1() {
    println!("{}", solution1(INPUT));
}

pub fn part2() {
    println!("{}", solution2(INPUT));
}

fn parse_manifold(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.char_indices()
                .filter_map(|(idx, ch)| matches!(ch, 'S' | '^').then_some(idx))
                .collect()
        })
        .collect()
}

fn solution1(input: &str) -> u64 {
    let manifold = parse_manifold(input);

    let mut splits = 0;
    let mut current_beams = std::collections::HashSet::from([manifold[0][0]]);

    for row in &manifold[1..] {
        for split in row {
            if current_beams.remove(split) {
                current_beams.insert(split + 1);
                current_beams.insert(split - 1);
                splits += 1;
            }
        }
    }

    splits
}

fn solution2(input: &str) -> u64 {
    let manifold = parse_manifold(input);

    let mut current_beams = std::collections::HashMap::from([(manifold[0][0], 1)]);

    for row in &manifold[1..] {
        for split in row {
            if let Some(beam_count) = current_beams.remove(split) {
                *current_beams.entry(split - 1).or_default() += beam_count;
                *current_beams.entry(split + 1).or_default() += beam_count;
            }
        }
    }

    current_beams.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input/example7.txt");

    #[test]
    fn example1() {
        assert_eq!(solution1(INPUT), 21);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 40);
    }
}
