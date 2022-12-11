use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;

use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
enum Operand {
    Itself,
    Num(u128),
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Mul(Operand),
    Plus(Operand),
    Sub(Operand),
    Div(Operand),
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u128>,
    op: Op,
    test: u128,
    count: usize,
}

impl Monkey {
    pub fn new(items: Vec<u128>, op: Op, test: u128) -> Self {
        Monkey {
            items,
            op,
            test,
            count: 0,
        }
    }
}

type MonkeyInput = Vec<(Monkey, Vec<usize>)>;
type Input = (MonkeyInput, u128);

type Output1 = usize;
type Output2 = Output1;

fn parse_input(input: &str) -> Input {
    let re = Regex::new(r"Monkey .:\n  Starting items:((:? \d+,?)*)\n  Operation: new = old ([+*-/]) ((:?\d+)|(:?old))\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)").unwrap();
    let mut i = 0;
    let mut prod = 1;
    let monkeys = re
        .captures_iter(input)
        .map(|cap| {
            // println!("{:?}", cap);
            let si = cap[1]
                .split(",")
                .map(|s| s.trim().parse::<u128>().unwrap())
                .collect::<Vec<u128>>();
            let operand = match &cap[4] {
                "old" => Operand::Itself,
                s => Operand::Num(s.parse::<u128>().unwrap()),
            };
            let op = match &cap[3] {
                "*" => Op::Mul(operand),
                "+" => Op::Plus(operand),
                "-" => Op::Sub(operand),
                "/" => Op::Div(operand),
                _ => panic!(),
            };
            let test = cap[7].parse::<u128>().unwrap();
            prod *= test;
            let then = cap[8].parse::<usize>().unwrap();
            let els = cap[9].parse::<usize>().unwrap();
            let res = (Monkey::new(si, op, test), vec![i, then, els]);
            i += 1;
            res
        })
        .collect::<MonkeyInput>();
    (monkeys, prod)
}

fn get_operand(old: u128, operand: Operand) -> u128 {
    match operand {
        Operand::Itself => old,
        Operand::Num(n) => n,
    }
}

fn calc(old: u128, op: Op) -> u128 {
    match op {
        Op::Mul(operand) => old * get_operand(old, operand),
        Op::Plus(operand) => old + get_operand(old, operand),
        Op::Sub(operand) => old - get_operand(old, operand),
        Op::Div(operand) => old / get_operand(old, operand),
    }
}

fn run(monkeys: &mut MonkeyInput, prod: u128, rounds: u128, div_by_3: bool) -> Output1 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get(i).unwrap().0.clone();
            let refs = monkeys.get(i).unwrap().1.clone();
            let indices: [usize; 3] = refs.clone().try_into().unwrap();
            let mut targets: [&mut (Monkey, Vec<usize>); 3] =
                monkeys.get_many_mut(indices).unwrap();

            let items = &monkey.items;
            targets[0].0.count += items.len();
            for item in items {
                let mut new_wl = calc(*item, monkey.op) % prod;
                if div_by_3 {
                    new_wl /= 3;
                }
                let is = new_wl % monkey.test == 0;

                let target = if is {
                    &mut targets[1].0
                } else {
                    &mut targets[2].0
                };
                target.items.push(new_wl);
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

fn solve1(input: &mut MonkeyInput, prod: u128) -> Output1 {
    run(input, prod, 20, true)
}

fn solve2(input: &mut MonkeyInput, prod: u128) -> Output2 {
    run(input, prod, 10_000, false)
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day11.txt"))?;
    let mut input_s = String::new();
    file.read_to_string(&mut input_s)?;

    println!("Starting parsing");
    let p_start = Instant::now();
    let input = parse_input(&input_s);
    println!("Parsing took {:.2?}", p_start.elapsed());

    println!("Starting part 1");
    let s1start = Instant::now();
    let s1 = solve1(input.0.clone().as_mut(), input.1);
    let s1elapsed = s1start.elapsed();
    println!("Starting part 2");
    let s2start = Instant::now();
    let s2 = solve2(input.0.clone().as_mut(), input.1);
    let s2elapsed = s2start.elapsed();
    println!("Part 1({:.2?}): {}", s1elapsed, s1);
    println!("Part 2({:.2?}): {}", s2elapsed, s2);
    Ok(())
}
