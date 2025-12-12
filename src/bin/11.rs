use std::collections::HashMap;
use std::env;
use std::fs;

type Count = u128;

fn parse_input(text: &str) -> (Vec<Vec<usize>>, HashMap<String, usize>) {
    // first collect all names
    let mut names: HashMap<String, usize> = HashMap::new();
    let mut edges_tmp: Vec<(String, Vec<String>)> = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(colon) = line.find(':') {
            let name = line[..colon].trim().to_string();
            let rest = line[colon + 1..].trim();
            let outs: Vec<String> = if rest.is_empty() {
                Vec::new()
            } else {
                rest.split_whitespace().map(|s| s.to_string()).collect()
            };
            edges_tmp.push((name.clone(), outs));
            names.entry(name).or_insert(0);
        }
    }
    // also include targets that may not appear as sources
    for (_src, outs) in &edges_tmp {
        for t in outs {
            names.entry(t.clone()).or_insert(0);
        }
    }

    // assign indices
    let mut idx = 0usize;
    for key in names.keys().cloned().collect::<Vec<_>>() {
        names.insert(key, { idx += 1; idx - 1 });
    }

    let n = names.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (s, outs) in edges_tmp {
        let si = names.get(&s).unwrap();
        for t in outs {
            let ti = names.get(&t).unwrap();
            adj[*si].push(*ti);
        }
    }

    (adj, names)
}

// Part 1: count paths from `you` to `out` with memoization and cycle detection
fn count_paths_part1(adj: &Vec<Vec<usize>>, names: &HashMap<String, usize>) -> Result<Count, String> {
    let start = match names.get("you") {
        Some(&i) => i,
        None => return Ok(0),
    };
    let target = match names.get("out") {
        Some(&i) => i,
        None => return Ok(0),
    };

    let n = adj.len();
    let mut memo: Vec<Option<Count>> = vec![None; n];
    let mut visiting: Vec<u8> = vec![0; n];

    fn dfs(u: usize, target: usize, adj: &Vec<Vec<usize>>, memo: &mut [Option<Count>], visiting: &mut [u8]) -> Result<Count, String> {
        if u == target {
            return Ok(1u128);
        }
        if let Some(v) = memo[u] {
            return Ok(v);
        }
        if visiting[u] == 1 {
            return Err(format!("Cycle detected at index {}", u));
        }
        visiting[u] = 1;
        let mut total: Count = 0;
        for &v in &adj[u] {
            let c = dfs(v, target, adj, memo, visiting)?;
            total = total.wrapping_add(c);
        }
        visiting[u] = 0;
        memo[u] = Some(total);
        Ok(total)
    }

    dfs(start, target, adj, &mut memo, &mut visiting)
}

// Part 2: count paths from `svr` to `out` that visit both `dac` and `fft` (any order)
fn count_paths_part2(adj: &Vec<Vec<usize>>, names: &HashMap<String, usize>) -> Result<Count, String> {
    let start = match names.get("svr") {
        Some(&i) => i,
        None => return Ok(0),
    };
    let target = match names.get("out") {
        Some(&i) => i,
        None => return Ok(0),
    };

    // required nodes
    let mut req_map: HashMap<usize, u8> = HashMap::new();
    if let Some(&d) = names.get("dac") {
        req_map.insert(d, 0);
    }
    if let Some(&f) = names.get("fft") {
        // if dac already present, fft gets next bit
        let idx = if req_map.contains_key(&f) { req_map[&f] } else { req_map.len() as u8 };
        req_map.insert(f, idx);
    }
    let req_count = req_map.values().copied().max().map(|m| m as usize + 1).unwrap_or(0);
    let fullmask: u8 = if req_count == 0 { 0 } else { ((1u16 << req_count) - 1) as u8 };

    let n = adj.len();
    // memo table sized n x (1<<req_count)
    let mask_size = 1usize << req_count;
    let mut memo: Vec<Vec<Option<Count>>> = vec![vec![None; mask_size]; n];
    let mut visiting: Vec<Vec<u8>> = vec![vec![0u8; mask_size]; n];

    fn dfs(u: usize, mask: u8, target: usize, adj: &Vec<Vec<usize>>, req_map: &HashMap<usize,u8>, fullmask: u8,
           memo: &mut [Vec<Option<Count>>], visiting: &mut [Vec<u8>]) -> Result<Count, String> {
        if u == target {
            return Ok(if mask == fullmask { 1 } else { 0 });
        }
        let midx = mask as usize;
        if let Some(v) = memo[u][midx] {
            return Ok(v);
        }
        if visiting[u][midx] == 1 {
            return Err(format!("Cycle detected at {} mask {}", u, mask));
        }
        visiting[u][midx] = 1;
        let mut total: Count = 0;
        for &v in &adj[u] {
            let mut m = mask;
            if let Some(&b) = req_map.get(&v) {
                m = mask | (1u8 << b);
            }
            let c = dfs(v, m, target, adj, req_map, fullmask, memo, visiting)?;
            total = total.wrapping_add(c);
        }
        visiting[u][midx] = 0;
        memo[u][midx] = Some(total);
        Ok(total)
    }

    let mut startmask: u8 = 0;
    if let Some(&b) = req_map.get(&start) {
        startmask |= 1u8 << b;
    }
    dfs(start, startmask, target, adj, &req_map, fullmask, &mut memo, &mut visiting)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { args[1].clone() } else { "python/11.in".to_string() };
    let text = fs::read_to_string(&path)?;

    let (adj, names) = parse_input(&text);

    match count_paths_part1(&adj, &names) {
        Ok(v) => println!("Part1 (you->out) paths = {}", v),
        Err(e) => println!("Part1 error: {}", e),
    }

    match count_paths_part2(&adj, &names) {
        Ok(v) => println!("Part2 (svr->out w/ dac & fft) paths = {}", v),
        Err(e) => println!("Part2 error: {}", e),
    }

    Ok(())
}
