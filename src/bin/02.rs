use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124\
";

fn parse_ranges_from_reader<R: Read>(mut rdr: R) -> Result<Vec<(i128, i128)>> {
    let mut s = String::new();
    rdr.read_to_string(&mut s)?;
    if s.starts_with("```") {
        let parts: Vec<&str> = s.lines().collect();
        let mut mid = parts.clone();
        if !mid.is_empty() {
            if mid[0].starts_with("```") {
                mid = mid[1..].to_vec();
            }
            if !mid.is_empty() && mid[mid.len() - 1].starts_with("```") {
                mid = mid[..mid.len() - 1].to_vec();
            }
        }
        s = mid.join("");
    }
    s = s.replace('\n', "");
    let toks: Vec<&str> = s.split(',').map(|t| t.trim()).filter(|t| !t.is_empty()).collect();
    let mut ranges = Vec::new();
    for t in toks {
        if let Some(idx) = t.find('-') {
            let a = &t[..idx];
            let b = &t[idx + 1..];
            let ai: i128 = a.parse()?;
            let bi: i128 = b.parse()?;
            ranges.push((ai, bi));
        }
    }
    Ok(ranges)
}

fn sum_exact_double(a: i128, b: i128) -> i128 {
    let mut total: i128 = 0;
    let mut k: u32 = 1;
    loop {
        let ten_k = i128::pow(10, k);
        let denom = ten_k + 1;
        let m_min = i128::pow(10, k - 1);
        if m_min * denom > b {
            break;
        }
        let m_low = std::cmp::max(m_min, (a + denom - 1) / denom);
        let m_high = std::cmp::min(ten_k - 1, b / denom);
        if m_low <= m_high {
            let n = m_high - m_low + 1;
            let sum_m = n * (m_low + m_high) / 2;
            total += denom * sum_m;
        }
        k += 1;
    }
    total
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let ranges = parse_ranges_from_reader(reader)?;
    let mut total: i128 = 0;
    for (a, b) in ranges {
        total += sum_exact_double(a, b);
    }
    Ok(total as usize)
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let ranges = parse_ranges_from_reader(reader)?;
    let max_b = ranges.iter().map(|(_, b)| *b).max().unwrap_or(0);
    let max_len = max_b.to_string().len() as u32;

    use std::collections::HashSet;
    let mut nums: HashSet<i128> = HashSet::new();
    for k in 1..=max_len {
        let ten_k = i128::pow(10, k);
        let m_min_digit = i128::pow(10, k - 1);
        let mut rcount: u32 = 2;
        while k * rcount <= max_len {
            let pow_kr = i128::pow(10, k * rcount);
            let denom = (pow_kr - 1) / (ten_k - 1);

            if m_min_digit.checked_mul(denom).map_or(true, |v| v > max_b) {
                break;
            }

            for (a, b) in &ranges {
                let m_low = std::cmp::max(m_min_digit, (a + denom - 1) / denom);
                let m_high = std::cmp::min(ten_k - 1, b / denom);
                if m_low <= m_high {
                    for m in m_low..=m_high {
                        nums.insert(m * denom);
                    }
                }
            }

            rcount += 1;
        }
    }

    let mut sum: i128 = 0;
    for v in nums {
        sum += v;
    }
    Ok(sum as usize)
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");
    assert_eq!(1227775554usize, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    assert_eq!(4174379265usize, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
