use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn line_2d(Point { x: x0, y: y0 }: Point, Point { x: x1, y: y1 }: Point) -> Vec<Point> {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut out = Vec::new();
    let mut x0 = x0;
    let mut y0 = y0;
    loop {
        out.push(Point { x: x0, y: y0 });
        if x0 == x1 && y0 == y1 {
            return out;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn print_points(points: Vec<Point>) {
    let point_set: HashSet<Point> = HashSet::from_iter(points);
    let helper = |start: &Point, end: &Point| {
        for y in start.y..=end.y {
            for x in start.x..=end.x {
                if point_set.contains(&Point { x, y }) {
                    print!("x");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    };
    match point_set
        .iter()
        .fold(None, |acc, Point { x: xval, y: yval }| match acc {
            None => {
                let point = Point { x: *xval, y: *yval };
                Some((point, point))
            }
            Some((Point { x: xmin, y: ymin }, Point { x: xmax, y: ymax })) => {
                // This is sad. I think this can be avoided by implementing a custom comparator
                // function, but I'd be even sadder.
                let xmin = std::cmp::min(xmin, *xval);
                let ymin = std::cmp::min(ymin, *yval);
                let xmax = std::cmp::max(xmax, *xval);
                let ymax = std::cmp::max(ymax, *yval);
                let min = Point { x: xmin, y: ymin };
                let max = Point { x: xmax, y: ymax };
                Some((min, max))
            }
        }) {
        None => (),
        Some((min, max)) => helper(&min, &max),
    }
}

fn main() {
    let width = 23;
    let height = 5;
    let test = |a, b| {
        let (x0, y0, x1, y1) = match (a, b) {
            (true, true) => (1, 1, width, height),
            (false, false) => (width, height, 1, 1),
            (false, true) => (1, height, width, 1),
            (true, false) => (width, 1, 1, height),
        };
        let start = Point { x: x0, y: y0 };
        let end = Point { x: x1, y: y1 };
        let line = line_2d(start, end);
        print_points(line);
    };

    test(false, false);
    test(true, false);
    test(true, true);
    test(false, true);
}
