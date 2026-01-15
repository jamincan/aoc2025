const INPUT: &str = include_str!("input/day9.txt");

pub fn part1() {
    println!("{}", solution1(INPUT));
}

pub fn part2() {
    println!("{}", solution2(INPUT));
}

fn parse_points(input: &str) -> Vec<[i64; 2]> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect()
}

fn solution1(input: &str) -> i64 {
    let points = parse_points(input);
    let n = points.len();

    let mut areas = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        let [ax, ay] = points[i];
        for j in i + 1..n {
            let [bx, by] = points[j];
            // have to add one to the length of the sides because we're dealing with tile coordinates and not the
            // actual corners
            let dx = (bx - ax).abs() + 1;
            let dy = (by - ay).abs() + 1;
            areas.push(dx * dy);
        }
    }

    *areas.iter().max().unwrap()
}

fn solution2(input: &str) -> i64 {
    let points = parse_points(input);

    // find all rectangles to test and sort by area
    let n = points.len();
    let mut rectangles = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        let [ax, ay] = points[i];
        for j in i + 1..n {
            let [bx, by] = points[j];
            let dx = (bx - ax).abs() + 1;
            let dy = (by - ay).abs() + 1;
            rectangles.push((dx * dy, Rect::new(ax, ay, bx, by)));
        }
    }
    rectangles.sort_unstable_by_key(|&(area, _)| area);

    // find the vertical edges of the polygon separately as with the winding number test we
    // are doing, only the vertical edges matter
    let (vertical_edges, horizontal_edges) = get_separated_edges(&points);

    // check rectangles by largest area first and return first that is contained
    rectangles
        .into_iter()
        .rev()
        .find(|&(_, rect)| rect.check(&vertical_edges, &horizontal_edges))
        .map(|(area, _)| area)
        .unwrap()
}

#[derive(Clone, Copy, Debug)]
struct VerticalEdge {
    x: i64,
    min_y: i64,
    max_y: i64,
    up: bool,
}

#[derive(Clone, Copy, Debug)]
struct HorizontalEdge {
    y: i64,
    min_x: i64,
    max_x: i64,
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Rect {
    fn new(ax: i64, ay: i64, bx: i64, by: i64) -> Self {
        Self {
            min_x: ax.min(bx),
            max_x: ax.max(bx),
            min_y: ay.min(by),
            max_y: ay.max(by),
        }
    }

    fn corners(&self) -> impl Iterator<Item = [i64; 2]> {
        [
            [self.min_x, self.min_y],
            [self.min_x, self.max_y],
            [self.max_x, self.min_y],
            [self.max_x, self.max_y],
        ]
        .into_iter()
    }

    fn check(&self, vertical_edges: &[VerticalEdge], horizontal_edges: &[HorizontalEdge]) -> bool {
        !has_intersection(vertical_edges, horizontal_edges, *self)
            && self
                .corners()
                .all(|[x, y]| is_corner_contained(vertical_edges, horizontal_edges, x, y))
    }
}

fn get_separated_edges(points: &[[i64; 2]]) -> (Vec<VerticalEdge>, Vec<HorizontalEdge>) {
    let mut vert = vec![];
    let mut horiz = vec![];

    for window in points.windows(2) {
        let [x1, y1] = window[0];
        let [x2, y2] = window[1];
        if x1 == x2 {
            // vertical
            vert.push(VerticalEdge {
                x: x1,
                min_y: y1.min(y2),
                max_y: y1.max(y2),
                up: y2 > y1,
            });
        } else {
            //horizontal
            horiz.push(HorizontalEdge {
                y: y1,
                min_x: x1.min(x2),
                max_x: x1.max(x2),
            });
        }
    }

    // close the loop
    let [fx, fy] = points[0];
    let [lx, ly] = *points.last().unwrap();
    if fx == lx {
        vert.push(VerticalEdge {
            x: fx,
            min_y: fy.min(ly),
            max_y: fy.max(ly),
            up: fy > ly,
        });
    } else {
        horiz.push(HorizontalEdge {
            y: fy,
            min_x: fx.min(lx),
            max_x: fx.max(lx),
        });
    }

    (vert, horiz)
}

/// check a rectangle against both vertical and horizontal edges of polygon for intersections
/// and return true if any are found
fn has_intersection(
    vertical_edges: &[VerticalEdge],
    horizontal_edges: &[HorizontalEdge],
    rect: Rect,
) -> bool {
    for &VerticalEdge {
        x, min_y, max_y, ..
    } in vertical_edges
    {
        if rect.min_x < x && x < rect.max_x {
            // vertical edge is between rectangles x bounds
            if min_y < rect.min_y && rect.min_y < max_y {
                return true; // crosses bottom edge of rect
            }
            if min_y < rect.max_y && rect.max_y < max_y {
                return true; // crosses top edge of rect
            }
        }
    }
    for &HorizontalEdge { y, min_x, max_x } in horizontal_edges {
        if rect.min_y < y && y < rect.max_y {
            // horiz edge is between rectangles x bounds
            if min_x < rect.min_x && rect.min_x < max_x {
                return true; // crosses left edge of rect
            }
            if min_x < rect.max_x && rect.max_x < max_x {
                return true; // crosses right edge of rect
            }
        }
    }
    // no intersections
    false
}

fn is_corner_contained(
    vertical_edges: &[VerticalEdge],
    horizontal_edges: &[HorizontalEdge],
    px: i64,
    py: i64,
) -> bool {
    // check if point is on horizontal edges
    for &HorizontalEdge { y, min_x, max_x } in horizontal_edges {
        if y == py && px >= min_x && px <= max_x {
            return true;
        }
    }

    // check vertical edges
    let mut winding = 0;
    for &VerticalEdge {
        x,
        min_y,
        max_y,
        up,
    } in vertical_edges
    {
        // check if point is on vertical edge
        if x == px && py >= min_y && py <= max_y {
            return true;
        }

        // point is left of edge, update the winding number
        if px < x && min_y <= py && py < max_y {
            if up {
                winding += 1;
            } else {
                winding -= 1;
            }
        }
    }
    winding != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input/example9.txt");

    #[test]
    fn example1() {
        assert_eq!(solution1(INPUT), 50);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 24);
    }

    #[test]
    fn correctly_check_rectangles() {
        let points = parse_points(INPUT);
        let (vert, horiz) = get_separated_edges(&points);
        let rectangles = [
            (Rect::new(7, 3, 11, 1), true),
            (Rect::new(9, 7, 9, 5), true),
            (Rect::new(9, 5, 2, 3), true),
            (Rect::new(11, 1, 2, 5), false),
            (Rect::new(7, 1, 11, 7), false),
            (Rect::new(7, 1, 9, 7), false),
            (Rect::new(7, 1, 9, 5), true),
            (Rect::new(7, 1, 2, 5), false),
            (Rect::new(7, 1, 2, 3), false),
        ];
        for (rect, result) in rectangles {
            println!("Testing {rect:?}");
            assert_eq!(rect.check(&vert, &horiz), result);
        }
    }
}
