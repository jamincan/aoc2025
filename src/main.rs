use std::env;

mod day1;

const SOLUTIONS: [[fn(); 2]; 12] = [
    [|| not_implemented(1, 1), || not_implemented(1, 2)],
    [|| not_implemented(2, 1), || not_implemented(2, 2)],
    [|| not_implemented(3, 1), || not_implemented(3, 2)],
    [|| not_implemented(4, 1), || not_implemented(4, 2)],
    [|| not_implemented(5, 1), || not_implemented(5, 2)],
    [|| not_implemented(6, 1), || not_implemented(6, 2)],
    [|| not_implemented(7, 1), || not_implemented(7, 2)],
    [|| not_implemented(8, 1), || not_implemented(8, 2)],
    [|| not_implemented(9, 1), || not_implemented(9, 2)],
    [|| not_implemented(10, 1), || not_implemented(10, 2)],
    [|| not_implemented(11, 1), || not_implemented(11, 2)],
    [|| not_implemented(12, 1), || not_implemented(12, 2)],
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
        let Ok(day @ 1..=12) = day.parse::<usize>() else {
            eprintln!("Invalid input '{day}' for day. Must be between 1 and 12.");
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
            Some(part) => SOLUTIONS[day - 1][part - 1](),
            None => {
                SOLUTIONS[day - 1][0]();
                SOLUTIONS[day - 1][1]();
            }
        }
    } else {
        // No arguments provided, go through all solutions
        for [part1, part2] in SOLUTIONS {
            part1();
            part2();
        }
        return;
    }
}

/** Print out a not implemented message for the specified day and part */
fn not_implemented(day: u8, part: u8) {
    println!("Advent of Code Day {day} Part {part} is not yet implemented.");
}
