use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec;
fn main() -> std::io::Result<()> {
    

    //  Elfgroup = {}
    //  ElfSet = {}
    //


    let mut file = File::open("puzzle input.txt")?;
    let mut reader = BufReader::new(file);
    let mut elfgroup = Vec::<u32>::new();
    

    let mut groupsum: u32 = 0;

    for line in reader.lines() {
        if line.as_ref().ok().unwrap() == "" {
            println!("{}", groupsum);
            elfgroup.push(groupsum.clone());
            groupsum = 0;
        }
        else {
            groupsum = groupsum + line.unwrap().clone().parse::<u32>().ok().unwrap();
            
        }
    }
    
    println!("{:?}", elfgroup.iter().enumerate().max());
    Ok(())
}