use std::cmp::max;
use std::fs;
use std::path::Path;

fn parse_input(s: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let parts: Vec<&str> = s.trim().splitn(2, "\n\n").collect();
    let range_part = parts.get(0).unwrap_or(&"");
    let mut ranges = Vec::new();
    for line in range_part.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut it = line.split('-');
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();
        ranges.push((a, b));
    }

    let mut ids = Vec::new();
    if parts.len() > 1 {
        for line in parts[1].lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            ids.push(line.parse::<i64>().unwrap());
        }
    }

    (ranges, ids)
}

fn merge_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return Vec::new();
    }
    ranges.sort_unstable();
    let mut out = Vec::new();
    let mut cur = ranges[0];
    for (lo, hi) in ranges.into_iter().skip(1) {
        if lo > cur.1 + 1 {
            out.push(cur);
            cur = (lo, hi);
        } else {
            cur.1 = max(cur.1, hi);
        }
    }
    out.push(cur);
    out
}

fn count_available_fresh(merged: &[(i64, i64)], ids: &[i64]) -> usize {
    if merged.is_empty() || ids.is_empty() {
        return 0;
    }
    let starts: Vec<i64> = merged.iter().map(|iv| iv.0).collect();
    let mut cnt = 0usize;
    for &x in ids {
        match starts.binary_search(&x) {
            Ok(k) => {
                if x <= merged[k].1 {
                    cnt += 1;
                }
            }
            Err(k) => {
                if k > 0 {
                    let idx = k - 1;
                    if x <= merged[idx].1 {
                        cnt += 1;
                    }
                }
            }
        }
    }
    cnt
}

fn total_fresh_ids(merged: &[(i64, i64)]) -> i128 {
    let mut sum: i128 = 0;
    for &(lo, hi) in merged {
        sum += (hi as i128) - (lo as i128) + 1;
    }
    sum
}

fn main() {
    // Look for input at `input/05.txt` then fallback to `python/5.in`
    let path1 = Path::new("input/05.txt");
    let path2 = Path::new("python/5.in");
    let input_path = if path1.exists() { path1 } else { path2 };
    let data = fs::read_to_string(input_path).expect("failed to read input file");

    let (ranges, ids) = parse_input(&data);
    let merged = merge_ranges(ranges);

    let part1 = count_available_fresh(&merged, &ids);
    let part2 = total_fresh_ids(&merged);

    println!("{}", part1);
    println!("{}", part2);
}
