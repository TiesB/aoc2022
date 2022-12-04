use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

fn parse_elf(elf: &str) -> (u32, u32) {
    let split: Vec<&str> = elf.split('-').collect();
    (
        split[0].parse::<u32>().unwrap(),
        split[1].parse::<u32>().unwrap(),
    )
}

fn overlaps_entirely(pair: &((u32, u32), (u32, u32))) -> bool {
    (pair.0 .0 <= pair.1 .0 && pair.0 .1 >= pair.1 .1)
        || (pair.1 .0 <= pair.0 .0 && pair.1 .1 >= pair.0 .1)
}

fn overlaps_partly(pair: &((u32, u32), (u32, u32))) -> bool {
    (pair.1 .0 <= pair.0 .1 && pair.1 .0 >= pair.0 .0)
        || (pair.0 .0 <= pair.1 .1 && pair.0 .0 >= pair.1 .0)
}

fn solve1(pairs: Vec<((u32, u32), (u32, u32))>) -> usize {
    pairs
        .iter()
        .filter(|pair| overlaps_entirely(pair))
        .map(|pair| pair.to_owned())
        .collect::<Vec<((u32, u32), (u32, u32))>>()
        .len()
}

fn solve2(pairs: Vec<((u32, u32), (u32, u32))>) -> usize {
    pairs
        .iter()
        .filter(|pair| overlaps_partly(pair))
        .map(|pair| pair.to_owned())
        .collect::<Vec<((u32, u32), (u32, u32))>>()
        .len()
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day04.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let pairs: Vec<((u32, u32), (u32, u32))> = input
        .trim()
        .split('\n')
        .map(|line| {
            let split: Vec<&str> = line.split(',').collect();
            (parse_elf(split[0]), parse_elf(split[1]))
        })
        .collect();

    println!("Found {} pairs", pairs.len());

    println!("Part 1: {}", solve1(pairs.clone()));
    println!("Part 2: {}", solve2(pairs.clone()));
    Ok(())
}
