use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

use take_until::TakeUntilExt;

type Row = Vec<u32>;
type Grid = Vec<Row>;
type Input = Grid;

type Output = u64;

fn parse_input(input: &str) -> Input {
    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect::<Row>()
        })
        .collect()
}

fn is_visible(input: &Input, r: usize, c: usize) -> bool {
    let size = input[r][c];
    input[r][0..c].iter().all(|&s| s < size)
        || input[r][c + 1..input[r].len()].iter().all(|&s| s < size)
        || input[0..r].iter().all(|row| row[c] < size)
        || input[r + 1..input.len()].iter().all(|row| row[c] < size)
}

fn scenic_score(input: &Input, r: usize, c: usize) -> u64 {
    let size = input[r][c];
    let left = input[r][0..c]
        .iter()
        .rev()
        .take_until(|&&s| s >= size)
        .collect::<Vec<&u32>>()
        .len();
    let right = input[r][c + 1..input[r].len()]
        .iter()
        .take_until(|&&s| s >= size)
        .collect::<Vec<&u32>>()
        .len();
    let up = input[0..r]
        .iter()
        .rev()
        .take_until(|row| row[c] >= size)
        .collect::<Vec<&Row>>()
        .len();
    let down = input[r + 1..input.len()]
        .iter()
        .take_until(|row| row[c] >= size)
        .collect::<Vec<&Row>>()
        .len();
    let score = (left * right * up * down).try_into().unwrap();
    score
}

fn solve1(input: &Input) -> Output {
    let mut visible = 0;
    for i in 1..input.len() - 1 {
        let row = &input[i];
        for j in 1..row.len() - 1 {
            if is_visible(input, i, j) {
                visible += 1;
            }
        }
    }
    (visible + 2 * input.len() + 2 * (input.get(0).unwrap().len() - 2))
        .try_into()
        .unwrap()
}

fn solve2(input: &Input) -> Output {
    let mut highest = 0;
    for i in 0..input.len() {
        let row = &input[i];
        for j in 0..row.len() {
            let score = scenic_score(input, i, j);
            if score > highest {
                highest = score;
            }
        }
    }
    highest
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day08.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("Starting parsing");
    let p_start = Instant::now();
    let dirs = parse_input(&input);
    println!("Parsing took {:.2?}", p_start.elapsed());

    println!("Starting part 1");
    let s1start = Instant::now();
    let s1 = solve1(&dirs);
    let s1elapsed = s1start.elapsed();
    println!("Starting part 2");
    let s2start = Instant::now();
    let s2 = solve2(&dirs);
    let s2elapsed = s2start.elapsed();
    println!("Part 1({:.2?}): {}", s1elapsed, s1);
    println!("Part 2({:.2?}): {}", s2elapsed, s2);
    Ok(())
}
