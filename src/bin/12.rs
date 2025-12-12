use std::collections::{HashSet, HashMap};
use std::fs;

fn parse_input(path: &str) -> (Vec<Vec<String>>, Vec<(usize, usize, Vec<usize>)>) {
    let data = fs::read_to_string(path).expect("failed to read input");
    let lines: Vec<String> = data.lines().map(|s| s.to_string()).collect();

    let mut shapes: Vec<Vec<String>> = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }
        if line.contains(':') && line.contains('x') {
            break;
        }
        if line.ends_with(':') {
            i += 1;
            let mut grid: Vec<String> = Vec::new();
            while i < lines.len() && !lines[i].trim().is_empty() {
                grid.push(lines[i].clone());
                i += 1;
            }
            shapes.push(grid);
        } else {
            i += 1;
        }
    }

    let mut regions = Vec::new();
    while i < lines.len() {
        let line = lines[i].trim().to_string();
        i += 1;
        if line.is_empty() { continue; }
        if !line.contains(':') { continue; }
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        let size = parts[0];
        let rest = parts[1];
        let wh: Vec<&str> = size.split('x').collect();
        let w = wh[0].parse::<usize>().unwrap();
        let h = wh[1].parse::<usize>().unwrap();
        let counts: Vec<usize> = rest.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        regions.push((w, h, counts));
    }

    (shapes, regions)
}

fn shape_cells(grid: &Vec<String>) -> Vec<(i32,i32)> {
    let mut cells = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '#' {
                cells.push((x as i32, y as i32));
            }
        }
    }
    cells
}

fn normalize(mut cells: Vec<(i32,i32)>) -> Vec<(i32,i32)> {
    let minx = cells.iter().map(|(x,_)| *x).min().unwrap_or(0);
    let miny = cells.iter().map(|(_,y)| *y).min().unwrap_or(0);
    for p in cells.iter_mut() {
        p.0 -= minx;
        p.1 -= miny;
    }
    cells.sort();
    cells
}

fn transforms(cells: &Vec<(i32,i32)>) -> Vec<Vec<(i32,i32)>> {
    let mut out: HashSet<Vec<(i32,i32)>> = HashSet::new();
    for &flipx in &[1, -1] {
        for &flipy in &[1, -1] {
            for rot in 0..4 {
                let mut pts = cells.clone();
                for p in pts.iter_mut() {
                    p.0 *= flipx;
                    p.1 *= flipy;
                }
                for _ in 0..rot {
                    pts = pts.into_iter().map(|(x,y)| (-y, x)).collect();
                }
                let norm = normalize(pts);
                out.insert(norm);
            }
        }
    }
    let mut v: Vec<Vec<(i32,i32)>> = out.into_iter().collect();
    v.sort_by_key(|a| (a.len(), a.clone()));
    v
}

fn greedy_pack(w: usize, h: usize, counts: &Vec<usize>, shape_orients: &Vec<Vec<Vec<(i32,i32)>>>) -> bool {
    // build pieces list
    let mut pieces: Vec<usize> = Vec::new();
    for (si, &cnt) in counts.iter().enumerate() {
        for _ in 0..cnt { pieces.push(si); }
    }
    if pieces.is_empty() { return true; }
    // compute areas for shapes
    let mut areas: Vec<usize> = vec![0; shape_orients.len()];
    for (i, orients) in shape_orients.iter().enumerate() {
        if let Some(o) = orients.get(0) {
            areas[i] = o.len();
        }
    }
    // sort pieces by area descending
    pieces.sort_by_key(|&s| std::cmp::Reverse(areas[s]));

    let mut grid = vec![false; w*h];
    for si in pieces {
        let mut placed = false;
        let orients = &shape_orients[si];
        // sort orientations by bbox area small -> large
        let mut or_sorted = orients.clone();
        or_sorted.sort_by_key(|o| {
            let maxx = o.iter().map(|(x,_)| *x).max().unwrap_or(0);
            let maxy = o.iter().map(|(_,y)| *y).max().unwrap_or(0);
            ((maxx+1) * (maxy+1)) as usize
        });
        for orient in or_sorted.iter() {
            let maxx = orient.iter().map(|(x,_)| *x).max().unwrap_or(0) as usize;
            let maxy = orient.iter().map(|(_,y)| *y).max().unwrap_or(0) as usize;
            if maxx + 1 > w || maxy + 1 > h { continue; }
            'oy: for oy in 0..=(h - (maxy+1)) {
                for ox in 0..=(w - (maxx+1)) {
                    let mut ok = true;
                    let mut cells = Vec::new();
                    for (x,y) in orient.iter() {
                        let gx = ox as i32 + *x;
                        let gy = oy as i32 + *y;
                        let idx = (gy as usize) * w + (gx as usize);
                        if grid[idx] { ok = false; break; }
                        cells.push(idx);
                    }
                    if ok {
                        for idx in cells { grid[idx] = true; }
                        placed = true;
                        break 'oy;
                    }
                }
            }
            if placed { break; }
        }
        if !placed { return false; }
    }
    true
}

fn main() {
    let (shapes, regions) = parse_input("python/12.in");
    let shapes_cells: Vec<Vec<(i32,i32)>> = shapes.iter().map(|g| shape_cells(g)).collect();
    let shape_orients: Vec<Vec<Vec<(i32,i32)>>> = shapes_cells.iter().map(|c| transforms(c)).collect();

    let mut ok = 0usize;
    let mut total = 0usize;
    let regions_len = regions.len();
    for (w, h, mut counts) in regions {
        total += 1;
        if counts.len() < shapes_cells.len() {
            counts.resize(shapes_cells.len(), 0);
        }
        let can = if greedy_pack(w, h, &counts, &shape_orients) {
            true
        } else if w != h && greedy_pack(h, w, &counts, &shape_orients) {
            true
        } else {
            false
        };
        if can { ok += 1; }
        if total % 50 == 0 { eprintln!("Checked {}/{}... ok={}", total, regions_len, ok); }
    }

    println!("{}", ok);
}
