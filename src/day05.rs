use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;
use std::time::Instant;
use std::vec;

use itertools::Itertools;

fn parse_lines(lines: Vec<&str>) -> (HashMap<usize, Vec<char>>, Vec<(usize, usize, usize)>) {
    let lines_chars = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let num_cols = (lines_chars[0].len() + 1) / 4;
    let mut cols: HashMap<usize, Vec<char>> = HashMap::new();
    for i in 0..num_cols {
        cols.insert(i, vec![]);
    }

    let mut moves: Vec<(usize, usize, usize)> = vec![];

    let mut setup_done = false;
    let mut i = 0;
    while i < lines_chars.len() {
        let line = &lines_chars[i];
        if line.len() == 0 || (!setup_done && lines_chars[i + 1].len() == 0) {
            setup_done = true;
            i += 1;
            continue;
        }

        if !setup_done {
            for j in 0..num_cols {
                let index = 1 + j * 4;
                let char = line[index];
                if char != ' ' {
                    cols.get_mut(&j).unwrap().insert(0, char)
                }
            }
        } else {
            if let Ok((cnt, src, dst)) = scan_fmt!(
                &line.into_iter().collect::<String>(),
                "move {d} from {d} to {d}",
                usize,
                usize,
                usize
            ) {
                moves.push((cnt, src - 1, dst - 1));
            }
        }
        i += 1;
    }
    (cols, moves)
}

fn solve1<'a>(
    (cols, moves): &'a mut (HashMap<usize, Vec<char>>, Vec<(usize, usize, usize)>),
) -> String {
    for (cnt, src, dst) in moves {
        let [src_col, dst_col] = cols.get_many_mut([src, dst]).unwrap();
        let x = src_col.len() - *cnt;
        let c = &mut src_col.split_off(x);
        c.reverse();
        dst_col.extend_from_slice(c);
    }
    cols.keys()
        .sorted()
        .map(|k| cols.get(k).unwrap().last().unwrap())
        .collect()
}

fn solve2<'a>(
    (cols, moves): &'a mut (HashMap<usize, Vec<char>>, Vec<(usize, usize, usize)>),
) -> String {
    for (cnt, src, dst) in moves {
        let [src_col, dst_col] = cols.get_many_mut([src, dst]).unwrap();
        let x = src_col.len() - *cnt;
        dst_col.extend_from_slice(&mut src_col.split_off(x));
    }
    cols.keys()
        .sorted()
        .map(|k| cols.get(k).unwrap().last().unwrap())
        .collect()
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day05.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let lines = input.split('\n').collect::<Vec<&str>>();
    println!("Starting parsing");
    let pstart = Instant::now();
    let data = parse_lines(lines);
    println!("Parsing took: {:.2?}", pstart.elapsed());

    println!("Starting part 1");
    let s1start = Instant::now();
    let s1 = solve1(data.clone().borrow_mut());
    let s1elapsed = s1start.elapsed();
    println!("Starting part 2");
    let s2start = Instant::now();
    let s2 = solve2(data.clone().borrow_mut());
    let s2elapsed = s2start.elapsed();
    println!("Part 1: {}. Took: {:.2?}", s1, s1elapsed);
    println!("Part 2: {}. Took: {:.2?}", s2, s2elapsed);
    Ok(())
}
