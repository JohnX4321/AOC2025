use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut pos: i64 = 50;
        let mut count: usize = 0;
        for line in reader.lines().flatten() {
            let s = line.trim();
            if s.is_empty() {
                continue;
            }
            let dir = s.chars().next().ok_or_else(|| anyhow!("empty line"))?;
            let dist: i64 = s[1..].parse()?;
            match dir {
                'R' => {
                    pos = (pos + dist) % 100;
                }
                'L' => {
                    pos = ((pos - dist) % 100 + 100) % 100;
                }
                _ => return Err(anyhow!("Unknown direction: {}", dir)),
            }
            if pos == 0 {
                count += 1;
            }
        }
        Ok(count)
    }

    // example expects 3 for part1
    assert_eq!(3usize, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut pos: i64 = 50;
        let mut total: usize = 0;
        for line in reader.lines().flatten() {
            let s = line.trim();
            if s.is_empty() {
                continue;
            }
            let dir = s.chars().next().ok_or_else(|| anyhow!("empty line"))?;
            let dist: i64 = s[1..].parse()?;
            match dir {
                'R' => {
                    let mut k0 = (100 - pos) % 100;
                    if k0 == 0 {
                        k0 = 100;
                    }
                    if k0 <= dist {
                        total += ((dist - k0) / 100 + 1) as usize;
                    }
                    pos = (pos + dist) % 100;
                }
                'L' => {
                    let mut k0 = pos % 100;
                    if k0 == 0 {
                        k0 = 100;
                    }
                    if k0 <= dist {
                        total += ((dist - k0) / 100 + 1) as usize;
                    }
                    pos = ((pos - dist) % 100 + 100) % 100;
                }
                _ => return Err(anyhow!("Unknown direction: {}", dir)),
            }
        }
        Ok(total)
    }

    // example expects 6 for part2
    assert_eq!(6usize, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
