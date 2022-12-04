use std::{
    fs::File,
    io::{BufRead, BufReader},
    
};

use itertools::Itertools;
/*
fn occurences(mut accumulator: u32, target: char, src: char) -> u32 {
    if target == src {
        return 1;
    } else {
        return 0;
    }
}

// to be used in fold function
fn occurence_parse(s: &str, c: char) -> u32 {
    let mut repeats: u32 = 0;
    return s
        .chars()
        .into_iter()
        .fold(repeats, |acc, x| acc + occurences(acc, c, x));
}
*/
fn part1() -> u32 {
    let file = std::fs::read_to_string("puzzle input.txt").unwrap();
    file.lines()
        // calculate the priority of the single unique item for each line
        .filter_map(|line| {
            let line = line.as_bytes();
            let (left, right) = line.split_at(line.len() / 2);

            left.iter()
                .find(|item| right.contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        })
        .sum()
}

fn part2() -> u32 {
    let file = std::fs::read_to_string("puzzle input.txt").unwrap();
    file
        .lines()
        .map(|line| line.as_bytes())
        .tuples()
        .filter_map(|(sack1, sack2, sack3)| {
            sack1
                .iter()
                .find(|item| sack2.contains(item) && sack3.contains(item))
                .map(|item| match item {
                    b'a'..=b'z' => (item - b'a') + 1,
                    _ => (item - b'A') + 1 + 26,
                } as u32)
        })
        .sum()
}
fn main() -> std::io::Result<()> {
    //println!("Hello, world!");

    println!("{}", part1()); //part1
    println!("{}", part2()); //part2

    //println!("{}", occurence_parse("ffffffffff", 'f'));
    Ok(())
}
