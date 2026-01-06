use std::ops::RangeInclusive;

const INPUT: &str = include_str!("input/day5.txt");

pub fn part1() {
    println!("{}", solution1(INPUT));
}

pub fn part2() {
    println!("{}", solution2(INPUT));
}

fn parse_ranges<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<RangeInclusive<u64>> {
    lines
        .map(|line| {
            let (start, end) = line.split_once('-').expect("invalid range format");
            let start = start.parse().expect("invalid start to range");
            let end = end.parse().expect("invalid end to range");
            start..=end
        })
        .collect()
}

fn parse_ids<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<u64> {
    lines
        .map(|line| line.parse().expect("invalid id"))
        .collect()
}

fn solution1(input: &str) -> usize {
    let mut lines = input.lines();
    let range_section = lines.by_ref().take_while(|line| !line.is_empty());
    let ranges = parse_ranges(range_section);
    let ids = parse_ids(lines);

    ids.iter()
        .filter(|id| ranges.iter().any(|r| r.contains(*id)))
        .count()
}

fn solution2(input: &str) -> u64 {
    let mut lines = input.lines();
    let range_section = lines.by_ref().take_while(|line| !line.is_empty());
    let mut ranges = parse_ranges(range_section);

    ranges.sort_by_key(|range| *range.start());

    // check each range to see if it overlaps and combine them if they do
    // since they are sorted by the start value, one pass is sufficient to
    // remove all overlaps
    let mut non_overlapping_ranges: Vec<RangeInclusive<u64>> = Vec::new();
    
    'outer: for range in ranges.into_iter() {
        // check to see if the range overlaps preceding ranges
        for existing in non_overlapping_ranges.iter_mut() {
            if existing.contains(range.start()) {
                if range.end() > existing.end() {
                    *existing = *existing.start()..=*range.end();
                }
                continue 'outer;
            } else if existing.contains(range.end()) {
                if range.start() < existing.start() {
                    *existing = *range.start()..=*existing.end();
                }
                continue 'outer;
            }
        }
        // no overlap, so add it to the list
        non_overlapping_ranges.push(range);
    }

    // sum the size of each range
    non_overlapping_ranges
        .into_iter()
        .map(|r| r.end() - r.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input/example5.txt");

    #[test]
    fn example1() {
        assert_eq!(solution1(INPUT), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 14);
    }
}
