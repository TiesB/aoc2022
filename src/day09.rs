use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Steps = (Direction, u32);
type Input = Vec<Steps>;

type Output1 = usize;
type Output2 = Output1;

type Position = (i64, i64);

fn parse_input(input: &str) -> Input {
    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    lines
        .iter()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let dir = parts[0];
            let steps = parts[1].parse::<u32>().unwrap();
            (
                match dir {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!(),
                },
                steps,
            )
        })
        .collect()
}

fn solve_for_n(input: &Input, n: usize) -> Output1 {
    let mut knots: Vec<Position> = vec![(0, 0); n];
    let mut visited_positions: HashSet<Position> = HashSet::new();
    for (direction, steps) in input {
        for _ in 0..*steps {
            knots[0] = match direction {
                Direction::Up => (knots[0].0, knots[0].1 + 1),
                Direction::Down => (knots[0].0, knots[0].1 - 1),
                Direction::Left => (knots[0].0 - 1, knots[0].1),
                Direction::Right => (knots[0].0 + 1, knots[0].1),
            };

            for i in 1..knots.len() {
                let prev = knots[i - 1];
                let cur = knots[i];
                let diff: Position = (prev.0 - cur.0, prev.1 - cur.1);
                if diff.0.abs() >= 2 || diff.1.abs() >= 2 {
                    knots[i] = (knots[i].0 + diff.0.signum(), knots[i].1 + diff.1.signum());
                }
            }

            visited_positions.insert(*knots.last().unwrap());
        }
    }
    visited_positions.len()
}

fn solve1(input: &Input) -> Output1 {
    solve_for_n(input, 2)
}

fn solve2(input: &Input) -> Output2 {
    solve_for_n(input, 10)
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day09.txt"))?;
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
