#![feature(map_many_mut)]
#[macro_use]
extern crate scan_fmt;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
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
}
