use std::collections::{HashMap, VecDeque};
use std::io;
use std::io::prelude::*;

// Liste d'adjacences
struct Graph {
    map: HashMap<String, u16>,
    adj: Vec<Vec<u16>>,
    deg_in: Vec<u16>,
}

impl Graph {
    fn build<B: io::BufRead>(lines: std::io::Lines<B>) -> Self {
        let mut map: HashMap<String, u16> = HashMap::new();
        let mut adj: Vec<Vec<u16>> = Vec::new();
        let mut deg_in: Vec<u16> = Vec::new();

        let mut to_s: Vec<Vec<String>> = Vec::new();

        for (i, line_res) in lines.enumerate() {
            let line = line_res.unwrap();
            let line = line.replace(":", "");
            let mut parts = line.split_whitespace();

            let opt_from = parts.next();
            if let Some(from) = opt_from {
                map.insert(from.to_string(), i as u16);
            }

            let targets: Vec<String> = parts.map(|s| s.to_string()).collect();
            deg_in.push(0);
            to_s.push(targets);
        }
        map.insert("out".to_string(), to_s.len() as u16);
        deg_in.push(0);

        for desc in to_s {
            let row: Vec<u16> = desc
                .iter()
                .map(|x| {
                    let ind = *map.get(x).unwrap();
                    deg_in[ind as usize] += 1;
                    ind
                })
                .collect();
            adj.push(row);
        }

        Graph { map, adj, deg_in }
    }

    fn ordre(&self) -> Vec<u16> {
        let mut queue = VecDeque::new();
        let mut res: Vec<u16> = Vec::new();
        let mut deg_in = self.deg_in.clone();
        let n = self.adj.len();

        for i in 0..n {
            if self.deg_in[i] == 0 {
                queue.push_back(i as u16);
            }
        }

        while !queue.is_empty() {
            let top = queue.pop_front().unwrap();
            res.push(top);

            for next in self.adj[top as usize].clone() {
                if (next as usize) == n {
                    continue;
                }
                deg_in[next as usize] -= 1;
                if deg_in[next as usize] == 0 {
                    queue.push_back(next);
                }
            }
        }
        res.push(n as u16);

        res
    }

    fn denombre(&self, src: String) -> usize {
        let target = self.adj.len();
        let source = *self.map.get(&src).unwrap();

        let mut d = vec![0; target + 1];
        d[source as usize] = 1;

        let ordre = self.ordre();

        for u in ordre {
            let u = u as usize;
            if d[u] == 0 || u == target {
                continue;
            }
            for v in self.adj[u].clone() {
                let v = v as usize;
                d[v] += d[u];
            }
        }
        println!("{:?}", d);

        d[target]
    }
}

fn main() {
    let stdin = io::stdin();

    let graph = Graph::build(stdin.lock().lines());
    // println!("{:?}\n{:?}\n{:?}", graph.map, graph.adj, graph.deg_in);

    // println!("{:?}", graph.ordre());
    println!("{}", graph.denombre("svr".into()));
}
