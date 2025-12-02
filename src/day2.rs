const INPUT: &str = include_str!("input/day2.txt");

use regex::Regex;
use std::sync::LazyLock;

pub fn part1() {
    println!("{}", solution1(INPUT));
}

pub fn part2() {
    println!("{}", solution2(INPUT));
}

fn solution1(input: &str) -> u64 {
    parse_ranges(input).flat_map(find_doubles).sum()
}

fn solution2(input: &str) -> u64 {
    parse_ranges(input).flat_map(find_repeats).sum()
}

fn parse_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    static RANGES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([0-9]+)-([0-9]+)").unwrap());

    RANGES.captures_iter(input).map(|caps| {
        let start = caps[1].parse::<u64>().unwrap();
        let end = caps[2].parse::<u64>().unwrap();
        (start, end)
    })
}

fn find_doubles((start, end): (u64, u64)) -> impl Iterator<Item = u64> {
    use ilog::IntLog;

    (start..=end).filter_map(|num| {
        let digits = (num.log10() + 1) as u32;
        // Skip odd numbers
        if digits % 2 > 0 {
            return None;
        };

        // Collect the right half using mod and compare with the left half
        let right = num % 10u64.pow(digits / 2);
        let left = (num - right) / 10u64.pow(digits / 2);
        if right == left {
            return Some(num);
        }

        None
    })
}

fn find_repeats((start, end): (u64, u64)) -> impl Iterator<Item = u64> {
    use ilog::IntLog;

    (start..=end).filter_map(|num| {
        let digits = (num.log10() + 1) as u32;

        // Check all possible pattern lengths
        'outer: for length in 1..=(digits / 2) {
            // Skip if can't divide evenly
            if digits % length > 0 {
                continue;
            }

            // Truncate off the right part and compare it to the pattern and repeat until all have been compared
            let pattern = num % 10u64.pow(length);
            let mut remainder = (num - pattern) / 10u64.pow(length);
            while remainder > 0 {
                //println!("{num}/{pattern}: {remainder}");
                let right = remainder % 10u64.pow(length);
                if right != pattern {
                    continue 'outer;
                }
                remainder = (remainder - right) / 10u64.pow(length);
            }
            return Some(num);
        }
        None
    })
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn parsing() {
        let mut ranges = super::parse_ranges(INPUT);
        assert_eq!(ranges.next(), Some((11, 22)));
        assert_eq!(ranges.next(), Some((95, 115)));
        assert_eq!(ranges.next(), Some((998, 1012)));
        assert_eq!(ranges.next(), Some((1188511880, 1188511890)));
        assert_eq!(ranges.next(), Some((222220, 222224)));
        assert_eq!(ranges.next(), Some((1698522, 1698528)));
        assert_eq!(ranges.next(), Some((446443, 446449)));
        assert_eq!(ranges.next(), Some((38593856, 38593862)));
        assert_eq!(ranges.next(), Some((565653, 565659)));
        assert_eq!(ranges.next(), Some((824824821, 824824827)));
        assert_eq!(ranges.next(), Some((2121212118, 2121212124)));
        assert_eq!(ranges.next(), None);
    }

    #[test]
    fn part1() {
        assert_eq!(super::solution1(INPUT), 1227775554);
    }

    #[test]
    fn part2() {
        assert_eq!(super::solution2(INPUT), 4174379265);
    }
}
