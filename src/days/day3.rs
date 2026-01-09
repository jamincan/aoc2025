const INPUT: &str = include_str!("input/day3.txt");

pub fn part1() {
    println!("{}", solution1(INPUT));
}

pub fn part2() {
    println!("{}", solution2(INPUT));
}

fn solution1(input: &str) -> u64 {
    input.lines().map(|l| max_n_digit_subsequence(l, 2)).sum()
}

fn solution2(input: &str) -> u64 {
    input.lines().map(|l| max_n_digit_subsequence(l, 12)).sum()
}

fn max_n_digit_subsequence(line: &str, n: usize) -> u64 {
    let bytes = line.as_bytes();
    let mut result = Vec::with_capacity(n);

    let mut start = 0;
    let remaining = n;

    for pos in 0..remaining {
        // we must leave enough characters to finish the number
        let end = bytes.len() - (remaining - pos);

        // find the maximum digit we can take at this position
        // search from the right as max_by_key gives the last max and we
        // want the first
        let (idx, &digit) = bytes[start..=end]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, d)| d)
            .unwrap();

        result.push(digit);
        start += idx + 1;
    }

    // SAFETY: result derived from valid utf8 string
    let s = unsafe { std::str::from_utf8_unchecked(&result) };
    s.parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("input/example3.txt");

    #[test]
    fn part1() {
        assert_eq!(super::solution1(INPUT), 357);
    }

    #[test]
    fn part2() {
        assert_eq!(super::solution2(INPUT), 3121910778619);
    }

    #[test]
    fn pt2_examples() {
        assert_eq!(super::solution2("987654321111111"), 987654321111);
        assert_eq!(super::solution2("811111111111119"), 811111111119);
        assert_eq!(super::solution2("234234234234278"), 434234234278);
        assert_eq!(super::solution2("818181911112111"), 888911112111);
    }
}
