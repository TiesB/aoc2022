#![feature(map_many_mut)]
#![feature(get_many_mut)]

use std::time::Instant;
#[macro_use]
extern crate scan_fmt;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() {
    let s = Instant::now();

    println!("Day 1:");
    day01::main().unwrap();
    println!("Day 2:");
    day02::main().unwrap();
    println!("Day 3:");
    day03::main().unwrap();
    println!("Day 4:");
    day04::main().unwrap();
    println!("Day 5:");
    day05::main().unwrap();
    println!("Day 6:");
    day06::main().unwrap();
    println!("Day 7:");
    day07::main().unwrap();
    println!("Day 8:");
    day08::main().unwrap();
    println!("Day 9:");
    day09::main().unwrap();
    println!("Day 10:");
    day10::main().unwrap();
    println!("Day 11:");
    day11::main().unwrap();
    println!("Day 12:");
    day12::main().unwrap();
    println!("Day 13:");
    day13::main().unwrap();

    println!("Total runtime: {:.2?}", s.elapsed());
}
