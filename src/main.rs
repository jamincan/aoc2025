use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

const LAST_DAY: usize = SOLUTIONS.len();

const SOLUTIONS: [[fn(); 2]; 5] = [
    [day1::part1, day1::part2],
    [day2::part1, day2::part2],
    [day3::part1, day3::part2],
    [day4::part1, day4::part2],
    [day5::part1, day5::part2],
];

/**
 * Usage: aoc2025 [DAY] [PART]
 * Run the solutions for Advent of Code 2025.
 *
 * [DAY]   Run provided solutions for the specified day
 * [PART]  Run only the specified part
 */
fn main() {
    // Get the command line arguments, skipping the program name
    let mut arguments = env::args().skip(1);
    let day = arguments.next();
    let part = arguments.next();

    if let Some(day) = day {
        // Parse and validate inputs
        let Ok(day @ 1..=LAST_DAY) = day.parse::<usize>() else {
            eprintln!(
                "Invalid input '{day}' for day. Must be between 1 and {}.",
                LAST_DAY
            );
            return;
        };
        let part = part.and_then(|part| {
            let Ok(part @ 1..=2) = part.parse::<usize>() else {
                eprintln!("Invalid input '{part}' for part. Must be between 1 and 2.");
                return None;
            };
            Some(part)
        });

        // Run specified solution
        match part {
            Some(part) => run(SOLUTIONS[day - 1][part - 1], day, part),
            None => {
                run(SOLUTIONS[day - 1][0], day, 1);
                run(SOLUTIONS[day - 1][1], day, 2);
            }
        }
    } else {
        // No arguments provided, go through all solutions
        for (n, [part1, part2]) in SOLUTIONS.iter().enumerate() {
            let day = n + 1;
            run(*part1, day, 1);
            run(*part2, day, 2);
        }
        return;
    }
}

fn run(solution: fn(), day: usize, part: usize) {
    use std::time::Instant;

    println!("Day {day}, Part {part}:");
    let now = Instant::now();
    solution();
    let elapsed = now.elapsed().as_millis();
    println!("{elapsed}ms elapsed");
}
