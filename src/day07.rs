use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

fn parse_input(lines: Vec<&str>) -> HashMap<Vec<&str>, u64> {
    let mut dirs: HashMap<Vec<&str>, u64> = HashMap::new();
    let mut pwd: Vec<&str> = vec![];
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match parts[0] {
            "$" => {
                if parts[1] == "cd" {
                    match parts[2] {
                        ".." => {
                            pwd.pop();
                        }
                        _ => {
                            pwd.push(parts[2]);
                        }
                    }
                }
            }
            "dir" => (),
            _ => {
                let size = parts[0].parse::<u64>().unwrap();
                let mut v: Vec<&str> = vec![];
                for dir in pwd.as_slice() {
                    v.push(dir);
                    *dirs.entry(v.clone()).or_insert(0) += size;
                }
            }
        }
    }
    dirs
}

fn solve1(dirs: &HashMap<Vec<&str>, u64>) -> u64 {
    dirs.values().filter(|&&size| size <= 100_000).sum()
}

fn solve2(dirs: &HashMap<Vec<&str>, u64>) -> u64 {
    let needed = 70_000_000 - dirs.iter().find(|(dir, _)| dir.len() == 1).unwrap().1;
    *dirs
        .values()
        .filter(|&&size| size + needed > 30_000_000)
        .min()
        .unwrap()
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day07.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let lines = input.trim().split('\n').collect::<Vec<&str>>();

    println!("Starting parsing");
    let p_start = Instant::now();
    let dirs = parse_input(lines);
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
