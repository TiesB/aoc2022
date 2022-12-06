use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;

fn solve_for_size(line: &Vec<char>, size: usize) -> usize {
    line.windows(size)
        .find_position(|w| w.iter().all_unique())
        .unwrap()
        .0
        + size
}

fn solve1(line: &Vec<char>) -> usize {
    solve_for_size(line, 4)
}

fn solve2(line: &Vec<char>) -> usize {
    solve_for_size(line, 14)
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day06.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let line = input.trim().chars().collect::<Vec<char>>();

    println!("Starting part 1");
    let s1start = Instant::now();
    let s1 = solve1(&line);
    let s1elapsed = s1start.elapsed();
    println!("Starting part 2");
    let s2start = Instant::now();
    let s2 = solve2(&line);
    let s2elapsed = s2start.elapsed();
    println!("Part 1: {}. Took: {:.2?}", s1, s1elapsed);
    println!("Part 2: {}. Took: {:.2?}", s2, s2elapsed);
    Ok(())
}
