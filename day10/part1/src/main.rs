use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

/// represent the lights in binary
#[derive(Debug, Default, PartialEq, Eq)]
struct Indicators(u16);

/// represent a switch in binary
#[derive(Debug, Default, PartialEq, Eq)]
struct Switch(u16);

impl Indicators {
    fn neighbors(self, switches: &Vec<Switch>) -> Vec<Self> {
        let mut neighbors = vec![];
        for switch in switches {
            neighbors.push(Indicators(self.0 ^ switch.0));
        }
        neighbors
    }
}

impl From<&str> for Indicators {
    fn from(value: &str) -> Self {
        let inner = value.trim();
        let inner = inner.strip_prefix('[').unwrap_or(inner);
        let inner = inner.strip_suffix(']').unwrap_or(inner);

        let mut val = 0;
        let mut pow = 1;
        for c in inner.chars() {
            if c == '#' {
                val += pow;
            }
            pow *= 2;
        }

        Indicators(val)
    }
}

impl From<&str> for Switch {
    fn from(value: &str) -> Self {
        let inner = value.trim();
        let inner = inner.strip_prefix('(').unwrap_or(inner);
        let inner = inner.strip_suffix(')').unwrap_or(inner);

        let mut val: u16 = 0;

        for s in inner.split(',').filter(|p| !p.is_empty()) {
            let bit: u8 = s.trim().parse().unwrap();
            if bit < 16 {
                val |= 1u16 << bit;
            }
        }

        Switch(val)
    }
}

/// Explore until the target is discovered by pressing the buttons
fn explore(switches: &Vec<Switch>, target: Indicators) -> u64 {
    let mut visited = vec![false; 65536];

    let mut queue = VecDeque::new();
    queue.push_back((Indicators::default(), 0));
    visited[0] = true;

    while queue.len() > 0 {
        let (s, d) = queue.pop_front().unwrap();
        // println!("{:b} {d}", s.0);
        if s == target {
            return d;
        }
        let neighbors = s.neighbors(switches);
        for t in neighbors {
            let index = t.0 as usize;
            if !visited[index] {
                queue.push_back((t, d + 1));
                visited[index] = true;
            }
        }
    }

    // should reach the target first
    0
}

fn main() {
    let stdin = io::stdin();

    let mut count = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut blocks = line.split(' ');
        let target = Indicators::from(blocks.next().unwrap());

        let switches: Vec<Switch> = blocks.rev().skip(1).map(Switch::from).collect();

        count += explore(&switches, target);
    }

    println!("press: {count}");
}
