use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

fn read_grid_from_reader<R: Read>(mut rdr: R) -> Result<Vec<String>> {
    let mut s = String::new();
    rdr.read_to_string(&mut s)?;
    // remove fencing if present
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
        s = mid.join("\n");
    }
    let lines: Vec<String> = s
        .lines()
        .map(|l| l.trim_end().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    Ok(lines)
}

fn count_accessible_from_lines(lines: &[String]) -> usize {
    let h = lines.len();
    if h == 0 {
        return 0;
    }
    let w = lines[0].len();
    let grid: Vec<Vec<u8>> = lines.iter().map(|l| l.as_bytes().to_vec()).collect();
    let dirs: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1),
    ];
    let mut total = 0usize;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != b'@' {
                continue;
            }
            let mut cnt = 0usize;
            for (di, dj) in &dirs {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if ni >= 0 && nj >= 0 && (ni as usize) < h && (nj as usize) < w {
                    if grid[ni as usize][nj as usize] == b'@' {
                        cnt += 1;
                    }
                }
            }
            if cnt < 4 {
                total += 1;
            }
        }
    }
    total
}

fn simulate_removal_from_lines(lines: &[String]) -> usize {
    let mut grid: Vec<Vec<u8>> = lines.iter().map(|l| l.as_bytes().to_vec()).collect();
    let h = grid.len();
    if h == 0 {
        return 0;
    }
    let w = grid[0].len();
    let dirs: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1),
    ];
    let mut removed_total = 0usize;
    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        for i in 0..h {
            for j in 0..w {
                if grid[i][j] != b'@' {
                    continue;
                }
                let mut cnt = 0usize;
                for (di, dj) in &dirs {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0 && nj >= 0 && (ni as usize) < h && (nj as usize) < w {
                        if grid[ni as usize][nj as usize] == b'@' {
                            cnt += 1;
                        }
                    }
                }
                if cnt < 4 {
                    to_remove.push((i, j));
                }
            }
        }
        if to_remove.is_empty() {
            break;
        }
        for (i, j) in to_remove.iter() {
            grid[*i][*j] = b'.';
        }
        removed_total += to_remove.len();
    }
    removed_total
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let mut s = String::new();
    reader.take(10_000_000).read_to_string(&mut s)?;
    let lines = read_grid_from_reader(s.as_bytes())?;
    Ok(count_accessible_from_lines(&lines))
}

fn part2<R: BufRead>(reader: R) -> Result<usize> {
    let mut s = String::new();
    reader.take(10_000_000).read_to_string(&mut s)?;
    let lines = read_grid_from_reader(s.as_bytes())?;
    Ok(simulate_removal_from_lines(&lines))
}

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");
    assert_eq!(13usize, part1(BufReader::new(TEST.as_bytes()))?);
    let input_file = File::open(INPUT_FILE).or_else(|_| File::open("python/4.in"))?;
    let result = time_snippet!(part1(BufReader::new(input_file))?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");
    assert_eq!(43usize, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = File::open(INPUT_FILE).or_else(|_| File::open("python/4.in"))?;
    let result = time_snippet!(part2(BufReader::new(input_file))?);
    println!("Result = {}", result);

    Ok(())
}
