use rand::Rng;
use std::cmp::{max, min};
use std::io;
use std::io::prelude::*;

#[derive(Clone, Copy, Debug)]
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

fn no_other_points_in(p1: &Point, p2: &Point, points: &Vec<Point>) -> bool {
    for p in points {
        if min(p1.x, p2.x) < p.x
            && p.x < max(p1.x, p2.x)
            && min(p1.y, p2.y) < p.y
            && p.y < max(p1.y, p2.y)
        {
            return false;
        }
    }

    true
}

fn is_inside(p: &Point, points: &Vec<Point>) -> bool {
    let mut count = 0;
    let n = points.len();
    for i in 0..(n - 1) {
        let (p1, p2) = (points[i], points[i + 1]);

        // edges
        if p.y == p1.y && p.y == p2.y && (min(p1.x, p2.x) <= p.x) && (max(p1.x, p2.x) >= p.x) {
            return true;
        }
        if p.x == p1.x && p.x == p2.x && (min(p1.y, p2.y) <= p.y) && (max(p1.y, p2.y) >= p.y) {
            return true;
        }

        // inner area
        if p1.y == p2.y && p1.y < p.y {
            // vertical ray
            if min(p1.x, p2.x) < p.x && p.x <= max(p1.x, p2.x) {
                count += 1;
            }
        }
    }

    (count % 2) == 1
}

fn good(x: i128, x_min: i128, x_max: i128) -> bool {
    let dx = x_max - x_min;
    let x1 = x_min + dx / 4 + dx / 100;
    let x2 = x_min + dx / 4 - dx / 100;
    let x3 = x_min + dx / 4 + dx / 4 + dx / 4 + dx / 100;
    let x4 = x_min + dx / 4 + dx / 4 + dx / 4 - dx / 100;

    if (x1 < x && x < x2) || (x3 < x && x < x4) {
        return true;
    }
    false
}

fn main() {
    let stdin = io::stdin();

    let mut rng = rand::rng();

    let mut points = vec![];
    let (mut x_min, mut x_max) = (i128::MAX, i128::MIN);
    let (mut y_min, mut y_max) = (i128::MAX, i128::MIN);
    for line in stdin.lock().lines() {
        let p = Point::from(line.unwrap());
        if p.x < x_min {
            x_min = p.x;
        }
        if p.y < y_min {
            y_min = p.y;
        }
        if p.x > x_max {
            x_max = p.x;
        }
        if p.y > y_max {
            y_max = p.y;
        }
        points.push(p);
    }
    println!("{:?}\n {x_min} {x_max} {y_min} {y_max}", points);
    let mut new_points = vec![];
    for p in points {
        if good(p.x, x_min, x_max) && good(p.y, y_min, y_max) {
            new_points.push(p);
        }
    }

    points = new_points;
    points.push(points[0]);
    let n = points.len();

    for i in 0..15 {
        let mut line = vec!['.'; 16];
        for j in 0..15 {
            if is_inside(&Point { x: j, y: i }, &points) {
                line[j as usize] = 'X';
            }
        }
        let s: String = line.into_iter().collect();
        println!("{}", s);
    }

    let mut max_area = 0;
    let mut ind = 0;
    for p1 in &points {
        for p2 in &points {
            let current_area = area(p1, p2);
            if current_area > max_area {
                let mut flag = true;
                let (min_x, max_x) = (min(p1.x, p2.x), max(p1.x, p2.x));
                let (min_y, max_y) = (min(p1.y, p2.y), max(p1.y, p2.y));
                for i in min_x..max_x {
                    if rng.random_range(0..14) != 0 {
                        continue;
                    }
                    if !is_inside(&Point { x: i, y: min_y }, &points) {
                        flag = false;
                        break;
                    }
                }
                for i in min_x..max_x {
                    if rng.random_range(0..14) != 0 {
                        continue;
                    }
                    if !is_inside(&Point { x: i, y: max_y }, &points) {
                        flag = false;
                        break;
                    }
                }
                for j in min_y..max_y {
                    if rng.random_range(0..14) != 0 {
                        continue;
                    }
                    if !is_inside(&Point { x: min_x, y: j }, &points) {
                        flag = false;
                        break;
                    }
                }
                for j in min_y..max_y {
                    if rng.random_range(0..14) != 0 {
                        continue;
                    }
                    if !is_inside(&Point { x: max_x, y: j }, &points) {
                        flag = false;
                        break;
                    }
                }

                if flag {
                    max_area = current_area;
                }
            }
            ind += 1;
            println!("{ind}/{}", n * n);
        }
    }
    println!("{max_area}");
}
