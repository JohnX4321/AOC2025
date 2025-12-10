use anyhow::Result;
use std::collections::BTreeMap;
use std::fs;
use std::env;

fn read_points(path: &str) -> Result<Vec<(i64, i64)>> {
    let s = fs::read_to_string(path)?;
    let mut pts = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let mut parts = line.split(',');
        let x: i64 = parts.next().unwrap().trim().parse()?;
        let y: i64 = parts.next().unwrap().trim().parse()?;
        pts.push((x, y));
    }
    Ok(pts)
}

fn max_area(pts: &[(i64, i64)]) -> i64 {
    let n = pts.len();
    let mut best = 0i64;
    for i in 0..n {
        let (xi, yi) = pts[i];
        for j in (i+1)..n {
            let (xj, yj) = pts[j];
            let area = (xi - xj).abs() + 1;
            let area2 = (yi - yj).abs() + 1;
            let a = area * area2;
            if a > best { best = a; }
        }
    }
    best
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { args[1].clone() } else { "AOC2025/input/9.txt".to_string() };

    let pts = read_points(&path)?;
    println!("{}", max_area(&pts));

    // Part 2
    let poly = pts.clone();
    let ys_all: Vec<i64> = poly.iter().map(|&(_, y)| y).collect();
    let miny = *ys_all.iter().min().unwrap();
    let maxy = *ys_all.iter().max().unwrap();

    let eps = 1e-9f64;
    let mut row_intervals: BTreeMap<i64, Vec<(i64,i64)>> = BTreeMap::new();
    let m = poly.len();
    for y in miny..=maxy {
        let mut xs: Vec<f64> = Vec::new();
        for i in 0..m {
            let (x1, y1) = poly[i];
            let (x2, y2) = poly[(i+1)%m];
            if y1 == y2 { continue; }
            if (y1 > y) != (y2 > y) {
                let xi = x1 as f64 + (y as f64 - y1 as f64) * (x2 as f64 - x1 as f64) / (y2 as f64 - y1 as f64);
                xs.push(xi);
            }
        }
        xs.sort_by(|a,b| a.partial_cmp(b).unwrap());
        let mut ivals: Vec<(i64,i64)> = Vec::new();
        let mut t = 0usize;
        while t + 1 < xs.len() {
            let xl = xs[t];
            let xr = xs[t+1];
            let l = (xl - eps).ceil() as i64;
            let r = (xr + eps).floor() as i64;
            if l <= r { ivals.push((l,r)); }
            t += 2;
        }

        // horizontal edges exactly on this row
        for i in 0..m {
            let (x1, y1) = poly[i];
            let (x2, y2) = poly[(i+1)%m];
            if y1 == y2 && y1 == y {
                let l = x1.min(x2);
                let r = x1.max(x2);
                ivals.push((l,r));
            }
        }

        if ivals.is_empty() { continue; }
        ivals.sort();
        // merge
        let mut merged = vec![ivals[0]];
        for &(l,r) in ivals.iter().skip(1) {
            let (ml, mr) = merged.last().cloned().unwrap();
            if l <= mr + 1 {
                let newr = mr.max(r);
                *merged.last_mut().unwrap() = (ml, newr);
            } else {
                merged.push((l,r));
            }
        }
        row_intervals.insert(y, merged);
    }

    let n = pts.len();
    let mut best2 = 0i64;
    for i in 0..n {
        let (xi, yi) = pts[i];
        for j in (i+1)..n {
            let (xj, yj) = pts[j];
            if xi == xj || yi == yj { continue; }
            let xmin = xi.min(xj);
            let xmax = xi.max(xj);
            let ymin = yi.min(yj);
            let ymax = yi.max(yj);
            let mut ok = true;
            for y in ymin..=ymax {
                let ivals_opt = row_intervals.get(&y);
                if ivals_opt.is_none() { ok = false; break; }
                let mut covered = false;
                for &(l,r) in ivals_opt.unwrap() {
                    if l <= xmin && r >= xmax { covered = true; break; }
                }
                if !covered { ok = false; break; }
            }
            if ok {
                let area = (xmax - xmin + 1) * (ymax - ymin + 1);
                if area > best2 { best2 = area; }
            }
        }
    }
    println!("{}", best2);

    Ok(())
}
