use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn load_input() -> Vec<String> {
    // try manifest dir at compile time (absolute path to the crate root)
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mpath = manifest_dir.join("input/07.txt");
    if mpath.exists() {
        let s = fs::read_to_string(&mpath).expect("failed to read input file");
        return s.lines().map(|l| l.to_string()).collect();
    }

    // fall back to a set of relative candidate paths (useful when running binary from different cwd)
    let candidates = ["input/07.txt", "./input/07.txt", "../input/07.txt", "../../input/07.txt", "python/7.in", "7.in"];
    for cand in &candidates {
        let p = Path::new(cand);
        if p.exists() {
            let s = fs::read_to_string(p).expect("failed to read input file");
            return s.lines().map(|l| l.to_string()).collect();
        }
    }
    panic!("input file not found (tried input/07.txt, python/7.in, 7.in)");
}

fn count_splits(lines: &Vec<String>) -> usize {
    if lines.is_empty() {
        return 0;
    }
    let h = lines.len();
    let w = lines[0].chars().count();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    // find S
    let mut start_r: Option<usize> = None;
    let mut start_c: Option<usize> = None;
    'outer: for r in 0..h {
        for (c, ch) in grid[r].iter().enumerate() {
            if *ch == 'S' {
                start_r = Some(r);
                start_c = Some(c);
                break 'outer;
            }
        }
    }
    let start_r = start_r.expect("No start 'S' found");
    let start_c = start_c.expect("No start 'S' found");

    let mut active: HashSet<usize> = HashSet::new();
    if start_r + 1 < h {
        active.insert(start_c);
    }

    let mut splits: usize = 0;

    for r in (start_r + 1)..h {
        let mut current: HashSet<usize> = active.iter().cloned().filter(|&c| c < w).collect();
        let mut handled: HashSet<usize> = HashSet::new();
        loop {
            let split_cols: Vec<usize> = current
                .iter()
                .cloned()
                .filter(|&c| grid[r][c] == '^' && !handled.contains(&c))
                .collect();
            if split_cols.is_empty() {
                break;
            }
            for c in &split_cols {
                handled.insert(*c);
            }
            for c in &split_cols {
                current.remove(c);
            }
            splits += split_cols.len();
            for c in split_cols {
                if c > 0 {
                    let left = c - 1;
                    if left < w && !handled.contains(&left) {
                        current.insert(left);
                    }
                }
                let right = c + 1;
                if right < w && !handled.contains(&right) {
                    current.insert(right);
                }
            }
        }

        active = current.into_iter().filter(|&c| c < w).collect();
    }

    splits
}

fn count_timelines(lines: &Vec<String>) -> u128 {
    if lines.is_empty() {
        return 0;
    }
    let h = lines.len();
    let w = lines[0].chars().count();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    // find S
    let mut sr: Option<usize> = None;
    let mut sc: Option<usize> = None;
    'outer2: for r in 0..h {
        for (c, ch) in grid[r].iter().enumerate() {
            if *ch == 'S' {
                sr = Some(r);
                sc = Some(c);
                break 'outer2;
            }
        }
    }
    let sr = sr.expect("Start 'S' not found");
    let sc = sc.expect("Start 'S' not found");

    let mut counts: HashMap<usize, u128> = HashMap::new();
    if sr + 1 < h {
        counts.insert(sc, 1u128);
    }

    for r in (sr + 1)..h {
        let mut curr: HashMap<usize, u128> = counts
            .iter()
            .filter(|(&c, &n)| c < w && n > 0)
            .map(|(&c, &n)| (c, n))
            .collect();

        loop {
            let split_cols: Vec<usize> = curr
                .iter()
                .filter(|(&c, &n)| n > 0 && grid[r][c] == '^')
                .map(|(&c, _)| c)
                .collect();
            if split_cols.is_empty() {
                break;
            }
            let mut new_curr = curr.clone();
            for c in split_cols {
                let n = new_curr.remove(&c).unwrap_or(0);
                if n == 0 {
                    continue;
                }
                if c > 0 {
                    let left = c - 1;
                    if left < w {
                        *new_curr.entry(left).or_insert(0) += n;
                    }
                }
                let right = c + 1;
                if right < w {
                    *new_curr.entry(right).or_insert(0) += n;
                }
            }
            curr = new_curr;
        }

        let mut new_counts: HashMap<usize, u128> = HashMap::new();
        for (c, n) in curr.into_iter() {
            if n > 0 {
                new_counts.insert(c, n);
            }
        }
        counts = new_counts;
    }

    counts.values().copied().sum()
}

fn main() {
    let lines = load_input();
    let part1 = count_splits(&lines);
    let part2 = count_timelines(&lines);
    println!("{}", part1);
    println!("{}", part2);
}
