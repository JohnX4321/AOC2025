use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct DSU {
    p: Vec<usize>,
    sz: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU { p: (0..n).collect(), sz: vec![1; n] }
    }

    fn find(&mut self, mut a: usize) -> usize {
        while self.p[a] != a {
            let pa = self.p[a];
            self.p[a] = self.p[pa];
            a = pa;
        }
        a
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb { return false; }
        if self.sz[ra] < self.sz[rb] {
            self.p[ra] = rb;
            self.sz[rb] += self.sz[ra];
        } else {
            self.p[rb] = ra;
            self.sz[ra] += self.sz[rb];
        }
        true
    }
}

fn read_points<P: AsRef<Path>>(path: P) -> io::Result<Vec<(i64,i64,i64)>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut pts = Vec::new();
    for line in reader.lines() {
        let l = line?;
        let l = l.trim();
        if l.is_empty() { continue; }
        let parts: Vec<&str> = l.split(',').collect();
        if parts.len() != 3 { continue; }
        let x = parts[0].parse::<i64>().unwrap();
        let y = parts[1].parse::<i64>().unwrap();
        let z = parts[2].parse::<i64>().unwrap();
        pts.push((x,y,z));
    }
    Ok(pts)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { args[1].clone() } else { "input/08.txt".to_string() };

    let pts = read_points(path)?;
    let n = pts.len();

    let mut pairs: Vec<(u64, usize, usize)> = Vec::new();
    pairs.reserve(n*(n.saturating_sub(1))/2);
    for i in 0..n {
        for j in (i+1)..n {
            let dx = pts[i].0 - pts[j].0;
            let dy = pts[i].1 - pts[j].1;
            let dz = pts[i].2 - pts[j].2;
            let d2 = (dx*dx + dy*dy + dz*dz) as u64;
            pairs.push((d2, i, j));
        }
    }

    pairs.sort_unstable_by_key(|k| k.0);

    // Part 1: union first 1000 pairs
    let take = 1000.min(pairs.len());
    let mut dsu1 = DSU::new(n);
    for k in 0..take {
        let (_, i, j) = pairs[k];
        dsu1.union(i,j);
    }

    // compute component sizes
    let mut comp = std::collections::HashMap::new();
    for i in 0..n {
        let mut dsu_temp = &mut dsu1;
        let r = dsu_temp.find(i);
        *comp.entry(r).or_insert(0usize) += 1;
    }
    let mut sizes: Vec<usize> = comp.values().copied().collect();
    sizes.sort_unstable_by(|a,b| b.cmp(a));
    while sizes.len() < 3 { sizes.push(1); }
    let part1 = (sizes[0] as u128) * (sizes[1] as u128) * (sizes[2] as u128);

    // Part 2: union until single component
    let mut dsu2 = DSU::new(n);
    let mut components = n;
    let mut last_pair: Option<(usize,usize)> = None;
    for &( _d2, i, j) in &pairs {
        if dsu2.union(i,j) {
            components -= 1;
            last_pair = Some((i,j));
            if components == 1 { break; }
        }
    }

    let part2 = match last_pair {
        None => 0u128,
        Some((i,j)) => {
            let xi = pts[i].0 as i128;
            let xj = pts[j].0 as i128;
            (xi * xj) as u128
        }
    };

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
