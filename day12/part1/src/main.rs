// Crazy Backtracking !

use std::collections::{BinaryHeap, HashSet};
use std::io;
use std::io::prelude::*;

/// Region is directly parsed from input
#[derive(Debug)]
struct Region {
    /// width of the region
    w: u8,

    /// height of the region
    h: u8,

    /// The number of each shapes to put in the region
    /// In our case the size of counts is always 6
    counts: Vec<u8>,

    /// Partially filled Region
    cells: Vec<Vec<bool>>,

    /// Pieces to be placed
    shapes: Vec<Shape>,
}

struct Move {
    x: u8,
    y: u8,

    /// The shape to apply
    shape_index: usize,

    /// The variant of the shape to apply
    variant_index: usize,
}

#[derive(PartialEq, Eq)]
enum Direction {
    Backward,
    Forward,
}

/// A Variant is a specific rotation of a Shape
#[derive(Debug, Clone)]
struct Variant {
    cells: Vec<(u8, u8)>,
}

/// Represent a Shape that can be placed in a Region
#[derive(Debug, Clone)]
struct Shape {
    /// Number of '#' cells
    area: u8,

    /// List of Variant: the shape rotated and flipped
    variants: Vec<Variant>,
}

impl From<&[&str; 3]> for Shape {
    fn from(rows: &[&str; 3]) -> Self {
        let grid: Vec<Vec<bool>> = rows
            .iter()
            .map(|r| r.chars().map(|c| c == '#').collect())
            .collect();
        let trimmed = trim_grid(grid);
        let area: usize = trimmed
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum();
        let variants = gen_variants(trimmed);

        Shape {
            area: (area as u8),
            variants,
        }
    }
}

fn rotate90(g: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let h = g.len();
    let w = g[0].len();
    let mut out = vec![vec![false; h]; w];
    for y in 0..h {
        for x in 0..w {
            out[x][h - 1 - y] = g[y][x];
        }
    }
    out
}

fn flip_h(g: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let h = g.len();
    let w = g[0].len();
    let mut out = vec![vec![false; w]; h];
    for y in 0..h {
        for x in 0..w {
            out[y][w - 1 - x] = g[y][x];
        }
    }
    out
}

fn trim_grid(g: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    if g.is_empty() || g[0].is_empty() {
        return g;
    }
    let h = g.len();
    let w = g[0].len();

    let mut min_y = h;
    let mut max_y = 0;
    let mut min_x = w;
    let mut max_x = 0;

    for y in 0..h {
        for x in 0..w {
            if g[y][x] {
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
                if x < min_x {
                    min_x = x;
                }
                if x > max_x {
                    max_x = x;
                }
            }
        }
    }

    if min_y > max_y || min_x > max_x {
        return vec![];
    }

    let new_h = max_y - min_y + 1;
    let new_w = max_x - min_x + 1;
    let mut out = vec![vec![false; new_w]; new_h];
    for y in 0..new_h {
        for x in 0..new_w {
            out[y][x] = g[min_y + y][min_x + x];
        }
    }
    out
}

impl From<&Vec<Vec<bool>>> for Variant {
    fn from(grid: &Vec<Vec<bool>>) -> Self {
        let mut cells = Vec::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, row) in grid.iter().enumerate() {
            for (x, &b) in row.iter().enumerate() {
                if b {
                    cells.push((x as u8, y as u8));
                    if x as u8 > max_x {
                        max_x = x as u8;
                    }
                    if y as u8 > max_y {
                        max_y = y as u8;
                    }
                }
            }
        }
        Variant { cells }
    }
}

impl Variant {
    fn signature(&self) -> String {
        let mut c = self.cells.clone();
        c.sort_unstable();
        c.into_iter()
            .map(|(x, y)| format!("{},{};", x, y))
            .collect()
    }
}

fn gen_variants(original: Vec<Vec<bool>>) -> Vec<Variant> {
    let mut seen = HashSet::<String>::new();
    let mut variants = Vec::new();

    let mut m = trim_grid(original.clone());
    for _ in 0..4 {
        let trimmed = trim_grid(m.clone());
        let v = Variant::from(&trimmed);
        let sig = v.signature();
        if seen.insert(sig) {
            variants.push(v);
        }
        m = rotate90(&m);
    }

    let mut mf = trim_grid(flip_h(&original));
    for _ in 0..4 {
        let trimmed = trim_grid(mf.clone());
        let v = Variant::from(&trimmed);
        let sig = v.signature();
        if seen.insert(sig) {
            variants.push(v);
        }
        mf = rotate90(&mf);
    }

    variants
}

/// Render a variant to ASCII lines using '#' for filled cells and '.' for empty.
fn render_variant(v: &Variant, width: usize, height: usize) -> Vec<String> {
    let mut grid = vec![vec!['.'; width]; height];
    for (dx, dy) in &v.cells {
        let x = *dx as usize;
        let y = *dy as usize;
        grid[y][x] = '#';
    }
    grid.into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
}

/// Print a single variant with a title/header.
fn print_variant(v: &Variant, width: usize, height: usize) {
    for line in render_variant(v, width, height) {
        println!("{}", line);
    }
    println!();
}

/// Print all variants for a given shape.
fn print_variants_for_shape(shape_index: usize, shape: &Shape) {
    println!("==============================");
    println!(
        "Shape {}: area={}, variants={}",
        shape_index,
        shape.area,
        shape.variants.len()
    );
    println!("==============================");
    for v in shape.variants.iter() {
        print_variant(v, 3, 3);
    }
}

/// Print variants for all shapes.
fn print_all_shapes_variants(shapes: &[Shape; 6]) {
    for (si, shape) in shapes.iter().enumerate() {
        print_variants_for_shape(si, shape);
    }
}

impl From<(String, &[Shape; 6])> for Region {
    fn from((s, shapes): (String, &[Shape; 6])) -> Self {
        let line = s.trim();
        let colon_pos = line.find(':').expect("Region line must contain ':'");
        let (left, right) = (&line[..colon_pos], &line[colon_pos + 1..]);

        let left = left.trim();
        let x_pos = left
            .find('x')
            .expect("Region dimensions must be in 'WxH' format");
        let w: u8 = left[..x_pos].trim().parse().expect("Invalid width");
        let h: u8 = left[x_pos + 1..].trim().parse().expect("Invalid height");

        let counts: Vec<u8> = right
            .trim()
            .split_whitespace()
            .map(|t| t.parse::<u8>().expect("Invalid count"))
            .collect();

        assert!(
            counts.len() == 6,
            "Region must list exactly 6 counts (for shapes 0..5)"
        );

        let cells = vec![vec![false; w as usize]; h as usize];

        Region {
            w,
            h,
            counts,
            cells,
            shapes: shapes.to_vec(),
        }
    }
}

impl Move {
    fn is_legal(&self, region: &Region) -> bool {
        let x0 = self.x;
        let y0 = self.y;

        let variant = &region.shapes[self.shape_index].variants[self.variant_index];

        for (dx, dy) in &variant.cells {
            let x = x0 + *dx;
            let y = y0 + *dy;
            if x >= region.w || y >= region.h {
                return false;
            }
            if region.cells[y as usize][x as usize] {
                return false;
            }
        }
        true
    }

    /// Apply a Move to a region
    /// We consider the move is legal or was legal if the direction is backward
    fn apply(&self, region: &mut Region, dir: Direction) {
        let x0 = self.x;
        let y0 = self.y;

        let variant = &region.shapes[self.shape_index].variants[self.variant_index];

        for (dx, dy) in &variant.cells {
            let x = x0 + *dx;
            let y = y0 + *dy;
            match dir {
                Direction::Forward => region.cells[y as usize][x as usize] = true,
                Direction::Backward => region.cells[y as usize][x as usize] = false,
            }
        }
    }
}

impl Region {
    fn display(&self) {
        for rows in &self.cells {
            println!(
                "{}",
                rows.iter()
                    .map(|x| if *x { '#' } else { '.' })
                    .collect::<String>()
            )
        }
        println!()
    }

    fn total_required_area(&self) -> usize {
        self.counts
            .iter()
            .enumerate()
            .map(|(i, &c)| (c as usize) * (self.shapes[i].area as usize))
            .sum()
    }

    fn next_empty(&self) -> Option<(u8, u8)> {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, &b) in row.iter().enumerate() {
                if !b {
                    return Some((x as u8, y as u8));
                }
            }
        }
        None
    }

    fn is_solvable(&mut self, depth: usize) -> bool {
        let region_area = (self.w as usize) * (self.w as usize);
        if self.total_required_area() > region_area {
            return false;
        }

        let Some((anchor_x, anchor_y)) = self.next_empty() else {
            return true;
        };

        // println!("{depth} {anchor_x} {anchor_y}");
        // self.display();

        let mut flag = true;
        for si in 0..self.shapes.len() {
            if self.counts[si] != 0 {
                flag = false;
            }
        }

        if flag {
            return true;
        }

        for si in 0..self.shapes.len() {
            if self.counts[si] == 0 {
                continue;
            }
            let shape = self.shapes[si].clone();

            for (vi, variant) in shape.variants.iter().enumerate() {
                for &(dx, dy) in &variant.cells {
                    let x0 = anchor_x + dx;
                    let y0 = anchor_y + dy;

                    let m = Move {
                        x: x0,
                        y: y0,
                        shape_index: si,
                        variant_index: vi,
                    };
                    if !m.is_legal(self) {
                        // println!("illegal move");
                        continue;
                    }

                    m.apply(self, Direction::Forward);
                    self.counts[si] -= 1;

                    if self.is_solvable(depth + 1) {
                        return true;
                    }

                    m.apply(self, Direction::Backward);
                    self.counts[si] += 1;
                }
            }
        }

        false
    }
}

fn main() {
    let stdin = io::stdin();

    let shapes: [Shape; 6] = [
        Shape::from(&["#..", "##.", "###"]),
        Shape::from(&["###", ".#.", "###"]),
        Shape::from(&["..#", "###", "###"]),
        Shape::from(&["#.#", "###", "##."]),
        Shape::from(&["#..", "##.", ".##"]),
        Shape::from(&["###", "#.#", "#.#"]),
    ];

    let shapes_test: [Shape; 6] = [
        Shape::from(&["###", "##.", "##."]),
        Shape::from(&["###", "##.", ".##"]),
        Shape::from(&[".##", "###", "##."]),
        Shape::from(&["##.", "###", "##."]),
        Shape::from(&["###", "#..", "###"]),
        Shape::from(&["###", ".#.", "###"]),
    ];
    let shapes = shapes_test;

    print_all_shapes_variants(&shapes);

    let mut results: Vec<bool> = Vec::new();
    for line in stdin.lock().lines() {
        let mut region = Region::from((line.unwrap(), &shapes));
        // println!("{:?}", region);
        results.push(region.is_solvable(0));
        // region.display();

        // let movement = Move {
        //     x: 0,
        //     y: 0,
        //     shape_index: 0,
        //     variant_index: 0,
        // };
        // movement.apply(&mut region, Direction::Forward);
        // region.display();
        // movement.apply(&mut region, Direction::Backward);
        // region.display();
    }

    println!("{:?}", results);
}
