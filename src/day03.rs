use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

use itertools::Itertools;

fn char_to_num(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 38
    } else {
        c as u32 - 96
    }
}

fn strings_intersection(strings: Vec<String>) -> String {
    strings[1..].iter().fold(strings[0].clone(), |acc, cur| {
        acc.chars().filter(|c| cur.chars().contains(c)).collect()
    })
}

fn solve1(rucksacks: Vec<String>) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            (
                rucksack[..rucksack.len() / 2].to_string(),
                rucksack[rucksack.len() / 2..].to_string(),
            )
        })
        .map(|rucksack| strings_intersection(vec![rucksack.0, rucksack.1]))
        .map(|v| char_to_num(v.chars().collect::<Vec<char>>()[0]))
        .sum()
}

fn solve2(rucksacks: Vec<String>) -> u32 {
    rucksacks
        .chunks(3)
        .collect_vec()
        .iter()
        .map(|v| strings_intersection(v.to_vec()))
        .map(|s| char_to_num(s.chars().collect::<Vec<char>>()[0]))
        .sum()
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day03.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let rucksacks: Vec<String> = input.trim().split('\n').map(|s| s.to_string()).collect();

    println!("Found {} rucksacks", rucksacks.len());

    println!("Part 1: {}", solve1(rucksacks.clone()));
    println!("Part 2: {}", solve2(rucksacks.clone()));
    Ok(())
}
