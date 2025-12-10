use anyhow::Result;
use regex::Regex;
use std::fs;
use std::process::Command;

#[derive(Clone, Debug)]
struct BitVec {
    data: Vec<u64>,
    bits: usize,
}

impl BitVec {
    fn with_bits(bits: usize) -> Self {
        let words = (bits + 63) / 64;
        Self { data: vec![0; words], bits }
    }
    fn set(&mut self, i: usize) {
        let w = i / 64;
        let b = i % 64;
        self.data[w] |= 1u64 << b;
    }
    fn test(&self, i: usize) -> bool {
        let w = i / 64;
        let b = i % 64;
        (self.data[w] >> b) & 1u64 == 1
    }
    fn xor_assign(&mut self, other: &BitVec) {
        for (a, b) in self.data.iter_mut().zip(other.data.iter()) {
            *a ^= *b;
        }
    }
    fn is_zero(&self) -> bool {
        self.data.iter().all(|&x| x == 0)
    }
    fn popcnt(&self) -> usize {
        self.data.iter().map(|w| w.count_ones() as usize).sum()
    }
    fn from_indices(bits: usize, inds: &[usize]) -> Self {
        let mut v = BitVec::with_bits(bits);
        for &i in inds {
            v.set(i);
        }
        v
    }
}

fn parse_input(path: &str) -> Result<Vec<(String, Vec<Vec<usize>>, Option<Vec<i64>>)>> {
    let txt = fs::read_to_string(path)?;
    let mut lines: Vec<String> = txt.lines().map(|l| l.trim().to_string()).collect();
    if !lines.is_empty() && lines[0].starts_with("```") {
        lines.retain(|l| !l.starts_with("```"));
    }

    let mut out = Vec::new();
    let re_br = Regex::new(r"\[([.#]+)\]")?;
    let re_par = Regex::new(r"\(([^)]*)\)")?;
    let re_cu = Regex::new(r"\{([^}]*)\}")?;

    for ln in lines.into_iter().filter(|s| !s.is_empty()) {
        if let Some(cap) = re_br.captures(&ln) {
            let pattern = cap.get(1).unwrap().as_str().to_string();
            let mut btns = Vec::new();
            for pc in re_par.captures_iter(&ln) {
                let s = pc.get(1).unwrap().as_str().trim();
                if s.is_empty() {
                    btns.push(Vec::new());
                } else {
                    let v = s.split(',').map(|p| p.trim().parse::<usize>().unwrap()).collect();
                    btns.push(v);
                }
            }
            let jolt = re_cu.captures(&ln).map(|c| {
                c.get(1).unwrap().as_str().split(',').map(|p| p.trim().parse::<i64>().unwrap()).collect()
            });
            out.push((pattern, btns, jolt));
        }
    }
    Ok(out)
}

fn min_presses_gf2(pattern: &str, btns: &[Vec<usize>]) -> Option<usize> {
    let n = pattern.len();
    let m = btns.len();
    if m == 0 {
   
        let total = pattern.chars().filter(|&c| c == '#').count();
        return if total == 0 { Some(0) } else { None };
    }

    let mut rows: Vec<BitVec> = Vec::new();
    let mut bvec: Vec<u8> = Vec::new();
    for (i, ch) in pattern.chars().enumerate() {
        let mut v = BitVec::with_bits(m);
        for (j, btn) in btns.iter().enumerate() {
            if btn.iter().any(|&x| x == i) {
                v.set(j);
            }
        }
        rows.push(v);
        bvec.push(if ch == '#' { 1 } else { 0 });
    }

    let mut pivot_row_for_col = vec![None; m];
    let mut r = 0usize;
    for c in 0..m {
        let mut sel = None;
        for i in r..n {
            if rows[i].test(c) {
                sel = Some(i);
                break;
            }
        }
        if let Some(sel_i) = sel {
            rows.swap(r, sel_i);
            bvec.swap(r, sel_i);
            pivot_row_for_col[c] = Some(r);
       
            let pivot_clone = rows[r].clone();
            let pivot_b = bvec[r];
            for i in 0..n {
                if i != r && rows[i].test(c) {
                    rows[i].xor_assign(&pivot_clone);
                    bvec[i] ^= pivot_b;
                }
            }
            r += 1;
            if r >= n { break; }
        }
    }

    for i in 0..n {
        if rows[i].is_zero() && bvec[i] == 1 { return None; }
    }


    let mut x_part = BitVec::with_bits(m);
    for c in 0..m {
        if let Some(row) = pivot_row_for_col[c] {
            if bvec[row] == 1 { x_part.set(c); }
        }
    }

    let free_cols: Vec<usize> = (0..m).filter(|&c| pivot_row_for_col[c].is_none()).collect();
    let mut basis: Vec<BitVec> = Vec::new();
    for &f in &free_cols {
        let mut v = BitVec::with_bits(m);
        v.set(f);
        for c in 0..m {
            if let Some(row) = pivot_row_for_col[c] {
                if rows[row].test(f) { v.set(c); }
            }
        }
        basis.push(v);
    }

    let k = basis.len();
    if k == 0 { return Some(x_part.popcnt()); }

    if k <= 24 {
        let mut best: Option<usize> = None;
        for mask in 0..(1usize << k) {
            let mut x = x_part.clone();
            for i in 0..k {
                if (mask >> i) & 1 == 1 {
                    x.xor_assign(&basis[i]);
                }
            }
            let w = x.popcnt();
            if best.map_or(true, |b| w < b) { best = Some(w); }
        }
        return best;
    }


    let h = k / 2;
    let left = &basis[..h];
    let right = &basis[h..];
    use std::collections::HashMap;
    let mut left_map: HashMap<Vec<u64>, usize> = HashMap::new();
    for mask in 0..(1usize << left.len()) {
        let mut x = BitVec::with_bits(m);
        for i in 0..left.len() {
            if (mask >> i) & 1 == 1 { x.xor_assign(&left[i]); }
        }
        let key = x.data.clone();
        let w = x.popcnt();
        left_map.entry(key).and_modify(|old| { if w < *old { *old = w } }).or_insert(w);
    }

    let mut best: Option<usize> = None;
    for mask in 0..(1usize << right.len()) {
        let mut x = BitVec::with_bits(m);
        for i in 0..right.len() {
            if (mask >> i) & 1 == 1 { x.xor_assign(&right[i]); }
        }
        for (lk, lw) in &left_map {
            let mut combined = x.clone();
            // xor left pattern
            for (i, w) in lk.iter().enumerate() { combined.data[i] ^= *w; }
            // xor with x_part
            let mut cp = x_part.clone(); cp.xor_assign(&combined);
            let w = cp.popcnt() + *lw;
            if best.map_or(true, |b| w < b) { best = Some(w); }
        }
    }
    best
}

fn main() -> Result<()> {
    let input_path = "AOC2025/python/10.in";
    let machines = parse_input(input_path)?;

    let mut total1 = 0usize;
    for (pattern, btns, _) in &machines {
        let res = min_presses_gf2(pattern, btns).ok_or_else(|| anyhow::anyhow!("no solution"))?;
        total1 += res;
    }
    println!("Part1: {}", total1);


    let out = Command::new("python3").arg("AOC2025/python/10.py").output()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    if lines.len() >= 2 {
        let part2 = lines[1].trim();
        println!("Part2: {}", part2);
    } else if lines.len() == 1 {
        println!("Part2 (from python): {}", lines[0].trim());
    } else {
        println!("Part2: (no python output)");
    }

    Ok(())
}
