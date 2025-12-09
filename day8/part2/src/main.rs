use std::io::{self, Read};

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    d: u128, // (au carré)
    u: usize,
    v: usize,
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    comps: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        DSU {
            parent,
            size,
            comps: n,
        }
    }
    fn find(&mut self, a: usize) -> usize {
        if self.parent[a] != a {
            let p = self.parent[a];
            self.parent[a] = self.find(p);
        }
        self.parent[a]
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
        self.comps -= 1;
        true
    }
}

fn parse_point(line: &str) -> Option<Point> {
    let s = line.trim();
    if s.is_empty() {
        return None;
    }
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let x = parts[0].trim().parse::<i64>().ok()?;
    let y = parts[1].trim().parse::<i64>().ok()?;
    let z = parts[2].trim().parse::<i64>().ok()?;
    Some(Point { x, y, z })
}

fn dist2(a: &Point, b: &Point) -> u128 {
    let dx = (a.x - b.x).abs() as i128;
    let dy = (a.y - b.y).abs() as i128;
    let dz = (a.z - b.z).abs() as i128;
    ((dx * dx) + (dy * dy) + (dz * dz)) as u128
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut points: Vec<Point> = Vec::new();
    for line in input.lines() {
        if let Some(p) = parse_point(line) {
            points.push(p);
        }
    }

    let n = points.len();
    if n <= 1 {
        eprintln!(
            "Entrée contient {} point(s). Pas de connexion nécessaire.",
            n
        );
        println!("0");
        return;
    }

    // génération des arrêtes
    let mut edges: Vec<Edge> = Vec::new();
    edges.reserve(n.saturating_mul(n.saturating_sub(1)) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let d = dist2(&points[i], &points[j]);
            edges.push(Edge { d, u: i, v: j });
        }
    }

    // tri par distance puis indices
    edges.sort_unstable_by(|a, b| a.d.cmp(&b.d).then(a.u.cmp(&b.u)).then(a.v.cmp(&b.v)));

    // Kruskal !!
    let mut dsu = DSU::new(n);
    let mut last_edge: Option<Edge> = None;

    for e in edges.iter().copied() {
        if dsu.union(e.u, e.v) {
            last_edge = Some(e);
            if dsu.comps == 1 {
                break;
            }
        }
    }

    let e = last_edge.unwrap();
    let x_prod = (points[e.u].x as i128) * (points[e.v].x as i128);

    println!("{}", x_prod);
}
