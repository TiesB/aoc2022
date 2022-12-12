use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

use pathfinding::prelude::dijkstra;

type Position = (i32, i32);
type Input = (Vec<Vec<char>>, Position, Position);

type Output1 = usize;
type Output2 = Output1;

fn parse_input(input: &str) -> Input {
    let lines = input.trim().split("\n").collect::<Vec<&str>>();
    let mut map: Vec<Vec<char>> = vec![];
    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);
    for y in 0..lines.len() {
        let line = lines[y];
        let mut row: Vec<char> = vec![];
        let chars = line.chars().collect::<Vec<char>>();

        for x in 0..chars.len() {
            let char = chars[x];
            if char == 'S' {
                start = (x.try_into().unwrap(), y.try_into().unwrap());
                row.push('a');
            } else if char == 'E' {
                end = (x.try_into().unwrap(), y.try_into().unwrap());
                row.push('z');
            } else {
                row.push(char);
            }
        }

        map.push(row);
    }
    (map, start, end)
}

fn successors(
    map: &Vec<Vec<char>>,
    invert: bool,
) -> impl Fn(&(i32, i32)) -> Vec<((i32, i32), usize)> + '_ {
    let height = map.len() as i32;
    let width = map[0].len() as i32;
    return move |&(x, y): &Position| -> Vec<(Position, usize)> {
        let mut result = vec![];
        vec![(0, -1), (0, 1), (-1, 0), (1, 0)]
            .iter()
            .for_each(|&(dx, dy)| {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < width && ny >= 0 && ny < height {
                    let cur = map[y as usize][x as usize] as i32;
                    let next = map[ny as usize][nx as usize] as i32;
                    let diff = next - cur;
                    if !invert {
                        if diff <= 1 {
                            result.push(((nx, ny), 1));
                        }
                    } else {
                        if diff >= -1 {
                            result.push(((nx, ny), 1));
                        }
                    }
                }
            });
        result
    };
}

fn solve1(input: &Input) -> Output1 {
    if let Some(d) = dijkstra(&input.1, successors(&input.0, false), |&p| p == input.2) {
        d.1
    } else {
        panic!()
    }
}

fn solve2(input: &Input) -> Output2 {
    if let Some(d) = dijkstra(&input.2, successors(&input.0, true), |&(x, y)| {
        input.0[y as usize][x as usize] == 'a'
    }) {
        d.1
    } else {
        panic!()
    }
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day12.txt"))?;
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
