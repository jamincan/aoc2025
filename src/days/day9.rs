const INPUT: &str = include_str!("input/day9.txt");

pub fn part1() {
    println!("{}", solution1(INPUT));
}

pub fn part2() {
    println!("{}", solution2(INPUT));
}

fn parse_points<T>(input: &str) -> Vec<[T; 2]>
where
    T: std::str::FromStr<Err: std::fmt::Debug>,
{
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            [x.parse().unwrap(), y.parse().unwrap()]
        })
        .collect()
}

fn solution1(input: &str) -> i64 {
    let points = parse_points::<i64>(input);
    let n = points.len();

    let mut areas = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        let [ax, ay] = points[i];
        for j in i + 1..n {
            let [bx, by] = points[j];
            let dx = (bx - ax).abs() + 1;
            let dy = (by - ay).abs() + 1;
            areas.push(dx * dy);
        }
    }

    *areas.iter().max().unwrap()
}

fn solution2(input: &str) -> i64 {
    let points = parse_points::<i64>(input);

    // find all rectangles to test and sort by area
    let n = points.len();
    let mut rectangles = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        let [ax, ay] = points[i];
        for j in i + 1..n {
            let [bx, by] = points[j];
            let dx = (bx - ax).abs() + 1;
            let dy = (by - ay).abs() + 1;
            rectangles.push((dx * dy, [ax, ay], [bx, by]));
        }
    }
    rectangles.sort_unstable();

    // find the vertical edges of the polygon as with the winding number test we
    // are doing, only the vertical edges matter
    let polygon = get_edges(&points);

    // check rectangles by largest area first and return first that is contained
    rectangles
        .into_iter()
        .rev()
        .find(|&(_, a, b)| check_if_rectangle_is_contained(&polygon, [a, b]))
        .map(|(area, _, _)| area)
        .unwrap()
}

type Point = [i64; 2];
type Edge = [Point; 2];

fn get_edges(points: &[Point]) -> Vec<Edge> {
    let mut edges = Vec::with_capacity(points.len() / 2 + 1);
    for pair in points.windows(2) {
        let a = pair[0];
        let b = pair[1];
        edges.push([a, b])
    }

    // add edge connecting first and last point if it's vertical
    let first = points[0];
    let last = points[points.len() - 1];
    edges.push([last, first]);
    edges
}

fn check_if_rectangle_is_contained(polygon: &[Edge], rect: [Point; 2]) -> bool {
    // first shrink the rectangle by 0.5 inward to help deal with instances where the rectangle coincides with an edge
    let [a, b] = rect;
    let pts = [a, [a[0], b[1]], [b[0], a[1]], b];
    let edges = get_edges(&pts);

    // check if any edges intersect
    if check_for_edge_intersection(polygon, &edges) {
        return false;
    }

    // check that all points are contained
    pts.iter()
        .all(|pt| check_if_point_is_contained(polygon, pt))
}

// check polygon and rectangle edges to see if rectangle edges intersect polygon edges
// edges that overlap are not considered
fn check_for_edge_intersection(polygon_edges: &[Edge], rect_edges: &[Edge]) -> bool {
    for &[p1, p2] in polygon_edges {
        for &[r1, r2] in rect_edges {
            if p1[0] == p2[0] && r1[1] == r2[1] {
                // polygon edge is vertical and rect edge is horizontal
                let min_py = p1[1].min(p2[1]);
                let max_py = p1[1].max(p2[1]);
                let min_rx = r1[0].min(r2[0]);
                let max_rx = r1[0].max(r2[0]);
                if min_py < r1[1] && r1[1] < max_py && min_rx < p1[0] && p1[0] < max_rx {
                    return true;
                }
            } else if p1[1] == p2[1] && r1[0] == r2[0] {
                // polygon edge is horizontal and rect edge is vertical
                let min_px = p1[0].min(p2[0]);
                let max_px = p1[0].max(p2[0]);
                let min_ry = r1[1].min(r2[1]);
                let max_ry = r1[1].max(r2[1]);
                if min_px < r1[0] && r1[0] < max_px && min_ry < p1[1] && p1[1] < max_ry {
                    return true;
                }
            }
        }
    }
    return false;
}

/// check if a point is in the polygon using Dan Sunday's winding number algorithm
fn check_if_point_is_contained(polygon: &[Edge], pt: &Point) -> bool {
    let mut winding_num = 0;
    let mut touching_edges = vec![];

    for &[a, b] in polygon {
        if a[0] == b[0] {
            // vertical polygon edge with point on edge; skip winding num and just check if point is on edge
            if a[0] == pt[0] {
                let min_py = a[1].min(b[1]);
                let max_py = a[1].max(b[1]);
                if pt[1] >= min_py && pt[1] <= max_py {
                    return true;
                }
            }
            // vertical polygon without point on edge - use winding number
            else if a[1] <= pt[1] && b[1] > pt[1] && pt[0] < a[0] {
                // upward crossing to the right
                touching_edges.push([a, b]);
                winding_num += 1;
            } else if a[1] > pt[1] && b[1] <= pt[1] && pt[0] < a[0] {
                // downward crossing to the right
                touching_edges.push([a, b]);
                winding_num -= 1;
            }
        } else if a[1] == pt[1] {
            // horizontal polygon edge with point on edge; skip winding num and just check if point is on edge
            let min_px = a[0].min(b[0]);
            let max_px = a[0].max(b[0]);
            if pt[0] >= min_px && pt[0] <= max_px {
                return true;
            }
        }
    }

    // if the winding number isn't 0, we know that the point is in the polygon
    winding_num != 0
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
        let polygon = get_edges(&points);
        let rectangles = [
            ([[7, 3], [11, 1]], true),
            ([[9, 7], [9, 5]], true),
            ([[9, 5], [2, 3]], true),
            ([[11, 1], [2, 5]], false),
            ([[7, 1], [11, 7]], false),
            ([[7, 1], [9, 7]], false),
            ([[7, 1], [9, 5]], true),
            ([[7, 1], [2, 5]], false),
            ([[7, 1], [2, 3]], false),
        ];
        for (rect, result) in rectangles {
            assert_eq!(check_if_rectangle_is_contained(&polygon, rect), result);
        }
    }

    #[test]
    fn check_horizontal_line() {
        let points = parse_points(INPUT);
        let polygon = get_edges(&points);
        let edge = [[9, 7], [11, 7]];
        assert_eq!(check_if_point_is_contained(&polygon, &edge[0]), true);
        assert_eq!(check_if_point_is_contained(&polygon, &edge[1]), true);
        let edge = [[2, 5], [9, 5]];
        assert_eq!(check_if_point_is_contained(&polygon, &edge[0]), true);
        assert_eq!(check_if_point_is_contained(&polygon, &edge[1]), true);
    }

    #[test]
    fn check_rect_pts_inside_but_edge_crosses() {
        // polygon is like a giant C
        let polygon = get_edges(&[
            [0, 10],
            [10, 10],
            [10, 8],
            [2, 8],
            [2, 2],
            [10, 2],
            [10, 0],
            [0, 0],
        ]);
        let rect = [[1, 1], [9, 9]];
        assert_eq!(check_if_rectangle_is_contained(&polygon, rect), false);
    }

    #[test]
    fn check_edge_intersection() {
        let edges = [
            ([[7, 1], [7, 9]], [[2, 5], [9, 5]], true),
            ([[7, 1], [7, 9]], [[2, 5], [5, 5]], false),
            ([[9, 9], [9, 1]], [[2, 2], [10, 2]], true),
            ([[9, 9], [9, 1]], [[2, 8], [10, 8]], true),
        ];
        for (a, b, result) in edges {
            assert_eq!(check_for_edge_intersection(&[a], &[b]), result);
        }
    }
}
