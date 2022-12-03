use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::path::Path;

fn play_to_score(play: char) -> u32 {
    // let d = play.to_digit(10).unwrap();
    let d = play as u32;
    if d > 80 {
        return d - 87;
    } else {
        return d - 64;
    }
}

fn solve1(games: Vec<(char, char)>) -> u32 {
    games
        .iter()
        .map(|game| {
            let e_score = play_to_score(game.0);
            let my_score = play_to_score(game.1);
            if my_score == e_score {
                return my_score + 3;
            } else if (my_score == 1 && e_score == 3) // I'm not proud of this
            || (my_score == 2 && e_score == 1)
            || (my_score == 3 && e_score == 2)
            {
                return my_score + 6;
            } else {
                return my_score;
            }
        })
        .sum()
}

fn solve2(games: Vec<(char, char)>) -> u32 {
    games
        .iter()
        .map(|game| {
            let e_score = play_to_score(game.0);
            let obj = play_to_score(game.1);

            if obj == 1 {
                if e_score == 1 {
                    return 3;
                } else if e_score == 2 {
                    return 1;
                } else {
                    return 2;
                }
            } else if obj == 2 {
                return 3 + e_score;
            } else {
                if e_score == 1 {
                    return 6 + 2;
                } else if e_score == 2 {
                    return 6 + 3;
                } else {
                    return 6 + 1;
                }
            }
        })
        .sum()
}

pub fn main() -> Result<(), Error> {
    let mut file = File::open(Path::new("inputs/day02.txt"))?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let games: Vec<(char, char)> = input
        .trim()
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|s| s.chars().collect::<Vec<char>>()[0])
                .collect::<Vec<char>>()
        })
        .map(|vals| (vals[0], vals[1]))
        .collect();

    println!("Found {} games", games.len());

    println!("Part 1: {}", solve1(games.clone()));
    println!("Part 2: {}", solve2(games.clone()));
    Ok(())
}
