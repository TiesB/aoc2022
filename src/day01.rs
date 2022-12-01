use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

fn solve1(elves: Vec<u32>) -> Result<u32, Error> {
    Ok(elves.iter().max().unwrap().to_owned())
}

fn solve2(elves: &mut Vec<u32>) -> Result<u32, Error> {
    elves.sort();
    elves.reverse();
    Ok(elves[0..3].iter().sum())
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day01.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let lines = input.trim().split('\n');
    let mut groups: Vec<Vec<String>> = Vec::new();
    groups.push(vec![]);
    for line in lines.into_iter() {
        if (line.len() > 0) {
            let mut group = groups.pop().unwrap();
            group.push(line.to_string());
            groups.push(group);
        } else {
            groups.push(vec![]);
        }
    }

    let elves = groups
        .iter()
        .map(|group| group.iter().map(|v| v.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();

    println!("Part 1: {}", solve1(elves.clone()).unwrap());
    println!("Part 2: {}", solve2(elves.clone().as_mut()).unwrap());
    Ok(())
}
