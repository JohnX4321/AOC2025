use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

fn read_grid<R: Read>(mut rdr: R) -> Result<Vec<String>> {
    let mut s = String::new();
    rdr.read_to_string(&mut s)?;
    let mut lines: Vec<String> = s.lines().map(|l| l.trim_end().to_string()).collect();
    if lines.is_empty() {
        return Ok(lines);
    }
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    for line in &mut lines {
        if line.len() < width {
            line.push_str(&" ".repeat(width - line.len()));
        }
    }
    Ok(lines)
}

fn is_space_col(grid: &Vec<String>, col: usize) -> bool {
    for row in grid {
        if row.as_bytes()[col] != b' ' {
            return false;
        }
    }
    true
}

fn solve_rowwise(grid: &Vec<String>) -> Result<i128> {
    let height = grid.len();
    if height == 0 {
        return Ok(0);
    }
    let width = grid[0].len();
    let mut col = 0usize;
    let mut grand: i128 = 0;
    while col < width {
        if is_space_col(grid, col) {
            col += 1;
            continue;
        }
        // collect block columns
        let mut block: Vec<usize> = Vec::new();
        while col < width && !is_space_col(grid, col) {
            block.push(col);
            col += 1;
        }
        // find operator in bottom row
        let mut operator: Option<char> = None;
        let bottom = &grid[height - 1];
        for &c in &block {
            let ch = bottom.as_bytes()[c] as char;
            if ch == '+' || ch == '*' {
                operator = Some(ch);
                break;
            }
        }
        let op = operator.ok_or_else(|| anyhow!("operator not found"))?;
        // extract numbers by rows (exclude bottom row)
        let mut numbers: Vec<i128> = Vec::new();
        for r in 0..(height - 1) {
            let mut s = String::new();
            for &c in &block {
                s.push(grid[r].as_bytes()[c] as char);
            }
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                let v: i128 = trimmed.parse()?;
                numbers.push(v);
            }
        }
        let result: i128 = match op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => unreachable!(),
        };
        grand += result;
    }
    Ok(grand)
}

fn solve_columnwise(grid: &Vec<String>) -> Result<i128> {
    let height = grid.len();
    if height == 0 {
        return Ok(0);
    }
    let width = grid[0].len();
    let mut col = 0usize;
    let mut grand: i128 = 0;
    while col < width {
        if is_space_col(grid, col) {
            col += 1;
            continue;
        }
        let mut block: Vec<usize> = Vec::new();
        while col < width && !is_space_col(grid, col) {
            block.push(col);
            col += 1;
        }
        // find operator in bottom row
        let mut operator: Option<char> = None;
        let bottom = &grid[height - 1];
        for &c in &block {
            let ch = bottom.as_bytes()[c] as char;
            if ch == '+' || ch == '*' {
                operator = Some(ch);
                break;
            }
        }
        let op = operator.ok_or_else(|| anyhow!("operator not found"))?;
        // read numbers column-wise right-to-left, exclude bottom row
        let mut numbers: Vec<i128> = Vec::new();
        for &c in block.iter().rev() {
            let mut s = String::new();
            for r in 0..(height - 1) {
                let ch = grid[r].as_bytes()[c] as char;
                if ch != ' ' {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                let v: i128 = s.parse()?;
                numbers.push(v);
            }
        }
        let result: i128 = match op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => unreachable!(),
        };
        grand += result;
    }
    Ok(grand)
}

fn main() -> Result<()> {
    // read input
    let path1 = Path::new("input/06.txt");
    let path2 = Path::new("python/6.in");
    let file = if path1.exists() { File::open(path1)? } else { File::open(path2)? };
    let grid = read_grid(BufReader::new(file))?;

    let part1 = solve_rowwise(&grid)?;
    let part2 = solve_columnwise(&grid)?;

    println!("{}", part1);
    println!("{}", part2);
    Ok(())
}
