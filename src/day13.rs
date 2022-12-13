use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::ops::Index;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;
use serde_json::Number;
use serde_json::Value;

type Pair = (Value, Value);
type Input = Vec<Pair>;

type Output1 = usize;
type Output2 = Output1;

fn parse_value(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn parse_input(input: &str) -> Input {
    input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let values = pair.split("\n").collect::<Vec<&str>>();
            (parse_value(values[0]), parse_value(values[1]))
        })
        .collect()
}

#[derive(Debug, PartialEq)]
enum CompareResult {
    Done(bool),
    NotDone,
}

fn check(pair: Pair) -> CompareResult {
    match &pair.0 {
        Value::Number(l) => match &pair.1 {
            Value::Number(r) => {
                let d = l.as_i64().unwrap() - r.as_i64().unwrap();
                if d < 0 {
                    CompareResult::Done(true)
                } else if d == 0 {
                    CompareResult::NotDone
                } else {
                    CompareResult::Done(false)
                }
            }
            Value::Array(_) => check((Value::Array(vec![pair.0]), pair.1)),
            _ => panic!(),
        },
        Value::Array(al) => match pair.1 {
            Value::Number(_) => check((pair.0, Value::Array(vec![pair.1]))),
            Value::Array(ar) => {
                if al.is_empty() && !ar.is_empty() {
                    return CompareResult::Done(true);
                }
                for i in 0..al.len() {
                    if ar.len() == i {
                        return CompareResult::Done(false);
                    }

                    let il = &al[i];
                    let ir = &ar[i];
                    let c = check((il.clone(), ir.clone()));
                    if c == CompareResult::NotDone {
                        if al.len() - 1 == i && ar.len() - 1 > i {
                            return CompareResult::Done(true);
                        }
                        continue;
                    }
                    return c;
                }

                CompareResult::NotDone
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn solve1(input: &Input) -> Output1 {
    let mut sum = 0;
    for i in 0..input.len() {
        let c = check(input[i].clone());
        if c == CompareResult::NotDone {
            panic!()
        }
        if c == CompareResult::Done(true) {
            sum += i + 1;
        }
    }
    sum
}

fn solve2(input: &Input) -> Output2 {
    let dec0 = Value::Array(vec![Value::Array(vec![Value::from(2)])]);
    let dec1 = Value::Array(vec![Value::Array(vec![Value::from(6)])]);
    let mut all: Vec<Value> = vec![dec0.clone(), dec1.clone()];
    for pair in input {
        all.push(pair.0.clone());
        all.push(pair.1.clone());
    }
    all.sort_by(|a, b| {
        let c = check((a.clone(), b.clone()));
        match c {
            CompareResult::Done(true) => Ordering::Less,
            CompareResult::Done(false) => Ordering::Greater,
            _ => panic!(),
        }
    });
    all.iter()
        .positions(|v| *v == dec0 || *v == dec1)
        .fold(1, |acc, cur| acc * cur)
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day13.txt"))?;
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
