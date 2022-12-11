use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;
use num::Integer;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Operand {
    Itself,
    Num(u64),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Mul(Operand),
    Plus(Operand),
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    test: u64,
    count: usize,
}

impl Monkey {
    pub fn new(items: Vec<u64>, op: Op, test: u64) -> Self {
        Monkey {
            items,
            op,
            test,
            count: 0,
        }
    }
}

type MonkeyInput = Vec<(Monkey, [usize; 3])>;
type Input = (MonkeyInput, u64);

type Output1 = usize;
type Output2 = Output1;

fn parse_input(input: &str) -> Input {
    let re = Regex::new(r"Monkey .:\n  Starting items:((:? \d+,?)+)\n  Operation: new = old ([+*-/]) ((:?\d+)|(:?old))\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)").unwrap();
    let mut i = 0;
    let mut lcm = 1;
    let monkeys = re
        .captures_iter(input)
        .map(|cap| {
            // println!("{:?}", cap);
            let si = cap[1]
                .split(",")
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let operand = match &cap[4] {
                "old" => Operand::Itself,
                s => Operand::Num(s.parse::<u64>().unwrap()),
            };
            let op = match &cap[3] {
                "*" => Op::Mul(operand),
                "+" => Op::Plus(operand),
                _ => panic!(),
            };
            let test = cap[7].parse::<u64>().unwrap();
            lcm = lcm.lcm(&test);
            let then = cap[8].parse::<usize>().unwrap();
            let els = cap[9].parse::<usize>().unwrap();
            let res = (Monkey::new(si, op, test), [i, then, els]);
            i += 1;
            res
        })
        .collect::<MonkeyInput>();
    (monkeys, lcm)
}

fn get_operand(old: u64, operand: Operand) -> u64 {
    match operand {
        Operand::Itself => old,
        Operand::Num(n) => n,
    }
}

fn calc(old: u64, op: Op) -> u64 {
    match op {
        Op::Mul(operand) => old * get_operand(old, operand),
        Op::Plus(operand) => old + get_operand(old, operand),
    }
}

fn run(monkeys: &mut MonkeyInput, lcm: u64, rounds: u64, div_by_3: bool) -> Output1 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get(i).unwrap().0.clone();
            let count = monkey.items.len();
            if count == 0 {
                continue;
            }

            let refs = monkeys.get(i).unwrap().1;
            let mut targets: [&mut (Monkey, [usize; 3]); 3] = monkeys.get_many_mut(refs).unwrap();

            targets[0].0.count += count;

            for item in monkey.items {
                let new_wl = calc(item, monkey.op) % lcm;

                if div_by_3 {
                    if new_wl % monkey.test == 0 {
                        targets[1].0.items.push(new_wl / 3);
                    } else {
                        targets[2].0.items.push(new_wl / 3);
                    }
                } else {
                    if new_wl % monkey.test == 0 {
                        targets[1].0.items.push(new_wl);
                    } else {
                        targets[2].0.items.push(new_wl);
                    }
                }
            }
            targets[0].0.items.clear();
        }
    }
    monkeys
        .iter()
        .map(|(m, _)| m.count)
        .sorted()
        .rev()
        .take(2)
        .fold(1, |acc, cur| acc * cur)
}

fn solve1(input: &mut MonkeyInput, lcm: u64) -> Output1 {
    run(input, lcm, 20, true)
}

fn solve2(input: &mut MonkeyInput, lcm: u64) -> Output2 {
    run(input, lcm, 10_000, false)
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day11.txt"))?;
    let mut input_s = String::new();
    file.read_to_string(&mut input_s)?;

    println!("Starting parsing");
    let p_start = Instant::now();
    let input = parse_input(&input_s);
    let p_elapsed = p_start.elapsed();

    println!("Starting part 1");
    let s1_start = Instant::now();
    let s1 = solve1(input.0.clone().as_mut(), input.1);
    let s1_elapsed = s1_start.elapsed();

    println!("Starting part 2");
    let s2_start = Instant::now();
    let s2 = solve2(input.0.clone().as_mut(), input.1);
    let s2_elapsed = s2_start.elapsed();

    println!("Parsing took {:.2?}", p_elapsed);
    println!("Part 1({:.2?}): {}", s1_elapsed, s1);
    println!("Part 2({:.2?}): {}", s2_elapsed, s2);
    Ok(())
}
