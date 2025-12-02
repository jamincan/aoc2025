const INPUT: &str = include_str!("input/day1.txt");

pub fn part1() {
    let result = solution1(INPUT);
    println!("{result}");
}

pub fn part2() {
    let result = solution2(INPUT);
    println!("{result}");
}

fn solution1(input: &str) -> i64 {
    let mut position = 50;
    let mut zero_count = 0;

    for delta in input.lines().map(parse_line) {
        position = (position + delta).rem_euclid(100);

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn solution2(input: &str) -> i64 {
    let mut position = 50;
    let mut zero_count = 0;

    for delta in input.lines().map(parse_line) {
        let (new_pos, zeroes) = apply_rotation(position, delta);
        position = new_pos;
        zero_count += zeroes;
    }

    zero_count
}

fn apply_rotation(start_pos: i64, delta: i64) -> (i64, i64) {
    let new_pos = (start_pos + delta).rem_euclid(100);
    let mut zeroes = (start_pos + delta).div_euclid(100).abs();

    // Only count landing on zero when going left as going right, the div_euclid picks it up
    if new_pos == 0 && delta < 0 {
        zeroes += 1;
    }

    // Remove the count picked up by div_euclid when moving left off zeroes
    if start_pos == 0 && delta < 0 {
        zeroes -= 1;
    }

    return (new_pos, zeroes);
}

fn parse_line(input: &str) -> i64 {
    // Parse direction and value
    let (dir, dist) = input.split_at(1);
    let dist = dist.parse::<i64>().expect("puzzle input guarantees an int");
    dist * match dir {
        "L" => -1,
        "R" => 1,
        _ => panic!("invalid puzzle input {input}"),
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn part1() {
        assert_eq!(super::solution1(INPUT), 3);
    }

    #[test]
    fn part2() {
        assert_eq!(super::solution2(INPUT), 6);
    }

    #[test]
    fn examples() {
        assert_eq!(super::apply_rotation(50, -68), (82, 1));
        assert_eq!(super::apply_rotation(82, -30), (52, 0));
        assert_eq!(super::apply_rotation(52, 48), (0, 1));
        assert_eq!(super::apply_rotation(0, -5), (95, 0));
        assert_eq!(super::apply_rotation(95, 60), (55, 1));
        assert_eq!(super::apply_rotation(55, -55), (0, 1));
        assert_eq!(super::apply_rotation(0, -1), (99, 0));
        assert_eq!(super::apply_rotation(99, -99), (0, 1));
        assert_eq!(super::apply_rotation(0, 14), (14, 0));
        assert_eq!(super::apply_rotation(14, -82), (32, 1));
    }

    #[test]
    fn large_turns() {
        assert_eq!(super::apply_rotation(50, -200), (50, 2));
        assert_eq!(super::apply_rotation(50, 200), (50, 2));
        assert_eq!(super::apply_rotation(0, 200), (0, 2));
        assert_eq!(super::apply_rotation(0, -200), (0, 2));
    }
}
