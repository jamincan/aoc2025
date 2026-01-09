const INPUT: &str = include_str!("input/day6.txt");

pub fn part1() {
    println!("{}", solution1(INPUT));
}

pub fn part2() {
    println!("{}", solution2(INPUT));
}

fn parse_ltr_numbers<'a>(input: impl Iterator<Item = &'a str>) -> (usize, Vec<u64>) {
    let mut lines: Vec<_> = input.map(|line| line.split_ascii_whitespace()).collect();
    let problem_length = lines.len();
    let mut numbers = Vec::new();
    'outer: loop {
        numbers.reserve(problem_length);
        for line in lines.iter_mut() {
            if let Some(n) = line.next() {
                let number = n.parse().expect("invalid number");
                numbers.push(number)
            } else {
                break 'outer;
            }
        }
    }
    (problem_length, numbers)
}

fn parse_ttb_numbers<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Vec<u64>> {
    let grid: Vec<_> = input.map(|row| row.as_bytes()).collect();

    let max_width = grid.iter().map(|row| row.len()).max().unwrap();

    let mut number_sets = vec![vec![]];

    // traverse column first, then row
    for col in 0..max_width {
        let mut num = Vec::with_capacity(grid.len());

        for row in &grid {
            match row.get(col) {
                Some(b' ') | None if num.is_empty() => continue,
                Some(b' ') | None => break,
                Some(digit @ b'0'..=b'9') => num.push(*digit),
                _ => panic!("invalid number"),
            }
        }

        // column is empty, so we need a new set of numbers
        if num.is_empty() && !number_sets.last().unwrap().is_empty() {
            number_sets.push(vec![]);
        } else {
            let number_set = number_sets.last_mut().unwrap();

            // SAFETY: can only contain digits between 0 and 9
            let num_str = unsafe { std::str::from_utf8_unchecked(&num) };
            let num = num_str.parse().expect("contains only valid digits");

            number_set.push(num);
        }
    }

    // if there is an empty set in the last position, remove it
    if number_sets
        .last()
        .is_some_and(|number_set| number_set.is_empty())
    {
        number_sets.pop();
    }

    number_sets
}

fn parse_ops(input: &str) -> Vec<fn(u64, u64) -> u64> {
    input
        .split_ascii_whitespace()
        .map(|op| match op {
            "+" => u64::saturating_add,
            "*" => u64::saturating_mul,
            _ => panic!("invalid operation"),
        })
        .collect()
}

fn solution1(input: &str) -> u64 {
    let mut lines = input.lines();

    // pull the last line to get the ops
    let ops = parse_ops(lines.next_back().unwrap());
    let (problem_length, numbers) = parse_ltr_numbers(lines);

    let problems = numbers.chunks_exact(problem_length);
    problems
        .zip(ops.into_iter())
        .map(|(numbers, op)| numbers.iter().copied().reduce(op).unwrap())
        .sum()
}

fn solution2(input: &str) -> u64 {
    let mut lines = input.lines();

    let ops = parse_ops(lines.next_back().unwrap());

    let numbers = parse_ttb_numbers(lines);

    numbers
        .iter()
        .zip(ops.into_iter())
        .map(|(numbers, op)| numbers.iter().copied().reduce(op).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input/example6.txt");

    #[test]
    fn example1() {
        assert_eq!(solution1(INPUT), 4277556);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 3263827);
    }
}
