use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

type Position = (i32, i32);
type Line = Vec<Position>;
type Lines = Vec<Line>;
type Input = Lines;

#[derive(Debug)]
enum Item {
    Sand,
    Rock,
}

type Map = HashMap<Position, Item>;
type DepthMap = HashMap<i32, i32>;

type Output1 = usize;
type Output2 = Output1;

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let parts = point.split(",").collect::<Vec<&str>>();
                    (
                        parts[0].parse::<i32>().unwrap(),
                        parts[1].parse::<i32>().unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

fn create_map(input: &Input) -> Map {
    let mut map = HashMap::new();
    for line in input {
        for i in 1..line.len() {
            let prev = line[i - 1];
            let cur = line[i];

            for x in if cur.0 >= prev.0 {
                prev.0..=cur.0
            } else {
                cur.0..=prev.0
            } {
                for y in if cur.1 >= prev.1 {
                    prev.1..=cur.1
                } else {
                    cur.1..=prev.1
                } {
                    map.insert((x, y), Item::Rock);
                }
            }
        }
    }
    map
}

fn create_depth_map(map: &Map) -> DepthMap {
    let mut res: DepthMap = HashMap::new();
    for point in map {
        let cur = res.get(&point.0 .0);
        if cur == None {
            res.insert(point.0 .0, point.0 .1);
            continue;
        }
        if *cur.unwrap() < point.0 .1 {
            res.insert(point.0 .0, point.0 .1);
        }
    }
    res
}

fn solve1(input: &Input) -> Output1 {
    let mut map: Map = create_map(input);
    let depth_map = create_depth_map(&map);

    let mut resting = 0;
    let mut abyss = false;
    while !abyss {
        let mut pos: Position = (500, 0);
        loop {
            let (x, y) = pos;

            if !depth_map.contains_key(&x) || *depth_map.get(&x).unwrap() < y {
                abyss = true;
                break;
            }

            if !map.contains_key(&(x, y + 1)) {
                pos = (x, y + 1);
                continue;
            }

            if !map.contains_key(&(x - 1, y + 1)) {
                pos = (x - 1, y + 1);
                continue;
            }

            if !map.contains_key(&(x + 1, y + 1)) {
                pos = (x + 1, y + 1);
                continue;
            }

            map.insert((x, y), Item::Sand);
            resting += 1;
            break;
        }
    }
    resting
}

fn solve2(input: &Input) -> Output2 {
    let mut map: Map = create_map(input);
    let depth_map = create_depth_map(&map);
    let floor_depth = depth_map.values().max().unwrap() + 2;

    let mut resting = 0;
    let mut filled = false;
    while !filled {
        let mut pos: Position = (500, 0);
        loop {
            let (x, y) = pos;

            if y + 1 < floor_depth {
                if !map.contains_key(&(x, y + 1)) {
                    pos = (x, y + 1);
                    continue;
                }

                if !map.contains_key(&(x - 1, y + 1)) {
                    pos = (x - 1, y + 1);
                    continue;
                }

                if !map.contains_key(&(x + 1, y + 1)) {
                    pos = (x + 1, y + 1);
                    continue;
                }
            }

            map.insert((x, y), Item::Sand);
            resting += 1;
            if (x, y) == (500, 0) {
                filled = true;
            }
            break;
        }
    }
    resting
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day14.txt"))?;
    let mut input_s = String::new();
    file.read_to_string(&mut input_s)?;

    println!("Starting parsing");
    let p_start = Instant::now();
    let input = parse_input(&input_s);
    let p_elapsed = p_start.elapsed();

    println!("Starting part 1");
    let s1_start = Instant::now();
    let s1 = solve1(&input);
    let s1_elapsed = s1_start.elapsed();

    println!("Starting part 2");
    let s2_start = Instant::now();
    let s2 = solve2(&input);
    let s2_elapsed = s2_start.elapsed();

    println!("Parsing took {:.2?}", p_elapsed);
    println!("Part 1({:.2?}): {}", s1_elapsed, s1);
    println!("Part 2({:.2?}): {}", s2_elapsed, s2);
    Ok(())
}
