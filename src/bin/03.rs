use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn max_subseq_value(s: &str, k: usize) -> i128 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }
    let digits: Vec<i128> = s.chars().filter_map(|c| c.to_digit(10).map(|d| d as i128)).collect();
    let n = digits.len();
    if k == 0 {
        return 0;
    }
    if k >= n {
        let mut v: i128 = 0;
        for &d in &digits {
            v = v * 10 + d;
        }
        return v;
    }
    let mut res: Vec<i128> = Vec::with_capacity(k);
    let mut start = 0usize;
    for remaining in (1..=k).rev() {
        let end = n - remaining;
        let mut best_d: i128 = -1;
        let mut best_pos = start;
        for i in start..=end {
            let d = digits[i];
            if d > best_d {
                best_d = d;
                best_pos = i;
                if best_d == 9 {
                    break; 
                }
            }
        }
        res.push(best_d);
        start = best_pos + 1;
    }
    let mut val: i128 = 0;
    for d in res {
        val = val * 10 + d;
    }
    val
}

fn part1<R: BufRead>(reader: R) -> Result<i128> {
    let mut total: i128 = 0;
    for line in reader.lines().flatten() {
        total += max_subseq_value(&line, 2);
    }
    Ok(total)
}

fn part2<R: BufRead>(reader: R) -> Result<i128> {
    let mut total: i128 = 0;
    for line in reader.lines().flatten() {
        total += max_subseq_value(&line, 12);
    }
    Ok(total)
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");
    assert_eq!(357i128, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = File::open(INPUT_FILE)?;
    let result = time_snippet!(part1(BufReader::new(input_file))?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    assert_eq!(3121910778619i128, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = File::open(INPUT_FILE)?;
    let result = time_snippet!(part2(BufReader::new(input_file))?);
    println!("Result = {}", result);

    Ok(())
}
