use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

type Input<'a> = Vec<i64>;

type Output1 = i64;
type Output2 = ();

fn parse_input(input: &str) -> Input {
    let mut r: Vec<i64> = vec![];
    let mut x: i64 = 1;
    input.trim().split('\n').for_each(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        match parts[0] {
            "addx" => {
                r.push(x);
                r.push(x);
                x += parts[1].parse::<i64>().unwrap();
            }
            "noop" => {
                r.push(x);
            }
            _ => panic!(),
        }
    });
    r
}

fn solve1(input: &Input) -> Output1 {
    vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|c| input.get(*c - 1).unwrap() * *c as i64)
        .sum()
}

fn solve2(input: &Input) -> Output2 {
    let mut clock = 0;
    for _ in 0..6 {
        for x in 0..40 {
            let val = input[clock];
            if val - 1 == x || val == x || val + 1 == x {
                print!("#");
            } else {
                print!(".");
            }
            clock += 1;
        }
        println!();
    }
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day10.txt"))?;
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
    solve2(&dirs);
    let s2elapsed = s2start.elapsed();
    println!("Part 1({:.2?}): {}", s1elapsed, s1);
    println!("Part 2({:.2?})", s2elapsed);
    Ok(())
}
