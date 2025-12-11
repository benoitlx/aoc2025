use std::io;
use std::io::prelude::*;

struct Point {
    x: i128,
    y: i128,
}

impl From<String> for Point {
    fn from(s: String) -> Self {
        let splited = s.split(',');
        let mut iter_int = splited.map(|x| x.parse().unwrap());
        Point {
            x: iter_int.next().unwrap(),
            y: iter_int.next().unwrap(),
        }
    }
}

fn area(p1: &Point, p2: &Point) -> i128 {
    (p1.x - p2.x + 1).abs() * (p1.y - p2.y + 1).abs()
}

fn main() {
    let stdin = io::stdin();

    let mut points = vec![];
    for line in stdin.lock().lines() {
        points.push(Point::from(line.unwrap()));
    }

    let mut max_area = 0;
    for p1 in &points {
        for p2 in &points {
            let current_area = area(p1, p2);
            if current_area > max_area {
                max_area = current_area;
            }
        }
    }
    println!("{max_area}");
}
