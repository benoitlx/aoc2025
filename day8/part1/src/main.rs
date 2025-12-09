use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl From<String> for Point {
    fn from(s: String) -> Self {
        let splited = s.split(',');
        let mut iter_int = splited.map(|x| x.parse().unwrap());
        Point {
            x: iter_int.next().unwrap(),
            y: iter_int.next().unwrap(),
            z: iter_int.next().unwrap(),
        }
    }
}

fn distance(p1: &Point, p2: &Point) -> u128 {
    let deltax = p1.x as i128 - p2.x as i128;
    let deltay = p1.y as i128 - p2.y as i128;
    let deltaz = p1.z as i128 - p2.z as i128;
    (deltax * deltax + deltay * deltay + deltaz * deltaz) as u128
}

#[derive(Debug)]
struct Minimum {
    best_dist: u128,
    best_i: usize,
    best_j: usize,
    found: bool,
}

impl Minimum {
    fn new() -> Self {
        Self {
            best_dist: u128::MAX,
            best_i: 0,
            best_j: 0,
            found: false,
        }
    }

    fn update(&mut self, i: usize, j: usize, d2: u128) {
        if d2 < self.best_dist {
            self.best_dist = d2;
            self.best_i = i;
            self.best_j = j;
            self.found = true;
        }
    }

    fn best(&self) -> Option<(usize, usize, u128)> {
        if self.found {
            Some((self.best_i, self.best_j, self.best_dist))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let r = self.find(self.parent[x]);
            self.parent[x] = r;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

fn main() {
    let stdin = io::stdin();

    let mut points = vec![];
    for line in stdin.lock().lines() {
        points.push(Point::from(line.unwrap()));
    }
    let n = points.len();
    let mut dsu = DSU::new(n);

    let k: usize = 1000;

    let mut selected: HashSet<(usize, usize)> = HashSet::new();

    for _ in 0..k {
        let mut minimum = Minimum::new();

        for i in 0..n {
            for j in (i + 1)..n {
                if selected.contains(&(i, j)) {
                    continue; // déjà choisie à une itération précédente
                }
                let d2 = distance(&points[i], &points[j]);
                minimum.update(i, j, d2);
            }
        }

        let Some((bi, bj, _)) = minimum.best() else {
            break;
        };

        selected.insert((bi, bj));

        let _ = dsu.union(bi, bj);
    }

    // Calcul final
    let mut comp_sizes: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let r = dsu.find(i);
        *comp_sizes.entry(r).or_insert(0) += 1;
    }

    // Produit des trois plus grandes composantes
    let mut sizes: Vec<usize> = comp_sizes.values().copied().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // décroissant

    let mut product: u128 = 1;
    for s in sizes.iter().take(3) {
        product *= *s as u128;
    }

    println!("{:?}", dsu);
    println!("{}", product);
}
