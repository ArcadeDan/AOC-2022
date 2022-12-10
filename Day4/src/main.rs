
use std::{
    fs::File,
    io::{BufRead, BufReader},
    
};

fn main() {
    let file = std::fs::read_to_string("puzzle input.txt").unwrap();
    file.lines().filter(|line| {
        let line = line.as_bytes();
        let (lhs, rhs) = line.split_at(line.find(','));

        lhs.iter()
            .find(predicate)
    })
        
        
    println!("Hello, world!");
}
