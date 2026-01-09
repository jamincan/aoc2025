use std::env;

mod days;

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
        let Ok(day @ 1..=days::COUNT) = day.parse::<usize>() else {
            eprintln!(
                "Invalid input '{day}' for day. Must be between 1 and {}.",
                days::COUNT
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
            Some(part) => run(days::SOLUTIONS[day - 1][part - 1], day, part),
            None => {
                run(days::SOLUTIONS[day - 1][0], day, 1);
                run(days::SOLUTIONS[day - 1][1], day, 2);
            }
        }
    } else {
        // No arguments provided, go through all solutions
        for (n, [part1, part2]) in days::SOLUTIONS.iter().enumerate() {
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
