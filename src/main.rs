#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn line2d(Point { x: x0, y: y0 }: Point, Point { x: x1, y: y1 }: Point) -> Vec<Point> {
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

fn main() {
    let start = Point { x: 1, y: 2 };
    let end = Point { x: 10, y: 5 };

    let line = line2d(start, end);
    println!("{:?}", line);
}
