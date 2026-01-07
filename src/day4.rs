use std::collections::HashSet;

const INPUT: &str = include_str!("input/day4.txt");

pub fn part1() {
    println!("{}", solution1(INPUT));
}

pub fn part2() {
    println!("{}", solution2(INPUT));
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.width + x]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y * self.width + x]
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        [
            (y > 0 && x > 0).then(|| (x - 1, y - 1)),
            (y > 0).then(|| (x, y - 1)),
            (y > 0 && x < self.width - 1).then(|| (x + 1, y - 1)),
            (x > 0).then(|| (x - 1, y)),
            (x < self.width - 1).then(|| (x + 1, y)),
            (y < self.height - 1 && x > 0).then(|| (x - 1, y + 1)),
            (y < self.height - 1).then(|| (x, y + 1)),
            (y < self.height - 1 && x < self.width - 1).then(|| (x + 1, y + 1)),
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Paper,
}

fn parse_grid(input: &str) -> Grid {
    let mut width = 0;
    let mut height = 0;
    let mut cells = Vec::with_capacity(input.len());

    for line in input.lines() {
        assert!(!line.is_empty(), "empty line in input");

        height += 1;

        if width == 0 {
            width = line.len()
        } else {
            assert_eq!(width, line.len(), "mismatched line widths in grid");
        }

        cells.extend(line.as_bytes().iter().map(|c| match c {
            b'.' => Cell::Empty,
            b'@' => Cell::Paper,
            _ => panic!("invalid character input '{}'", c),
        }));
    }
    Grid {
        width,
        height,
        cells,
    }
}

fn solution1(input: &str) -> usize {
    let grid = parse_grid(input);
    let paper = get_paper_to_move(&grid);
    paper.len()
}

fn solution2(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut neighbor_counts = vec![0u8; grid.cells.len()];
    let mut removed_paper = 0;
    let mut to_check = Vec::new();

    // cache the number of adjacent cells that are paper for each cell
    // and if the count is less than four, add it to the cells to check
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.get(x, y) == Cell::Paper {
                let count = grid
                    .neighbors(x, y)
                    .filter(|(nx, ny)| grid.get(*nx, *ny) == Cell::Paper)
                    .count();
                neighbor_counts[y * grid.width + x] = count as u8;
                if count < 4 {
                    to_check.push((x, y))
                };
            }
        }
    }

    while !to_check.is_empty() {
        let mut next_to_check = HashSet::new();

        for &(x, y) in &to_check {
            let idx = y * grid.width + x;

            if grid.get(x, y) == Cell::Empty {
                continue;
            }

            if neighbor_counts[idx] < 4 {
                *grid.get_mut(x, y) = Cell::Empty;
                removed_paper += 1;

                // fix neighbor count for adjacent cells and add neighbours in the next cells to check
                for (nx, ny) in grid.neighbors(x, y) {
                    let n_idx = ny * grid.width + nx;
                    if grid.get(nx, ny) == Cell::Paper {
                        neighbor_counts[n_idx] -= 1;
                        next_to_check.insert((nx, ny));
                    }
                }
            }
        }

        to_check.clear();
        to_check.extend(next_to_check.iter());
    }

    removed_paper
}

fn get_paper_to_move(grid: &Grid) -> HashSet<(usize, usize)> {
    let mut paper_to_move = HashSet::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            let current = grid.get(x, y);
            if current == Cell::Empty {
                continue;
            }

            let adjacent_paper = grid
                .neighbors(x, y)
                .map(|(x, y)| grid.get(x, y))
                .filter(|cell| *cell == Cell::Paper)
                .count();

            if adjacent_paper < 4 {
                paper_to_move.insert((x, y));
            }
        }
    }

    paper_to_move
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input/example4.txt");

    #[test]
    fn example1() {
        assert_eq!(solution1(INPUT), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 43);
    }
}
