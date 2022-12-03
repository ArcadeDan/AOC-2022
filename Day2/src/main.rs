use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn eval(self, v: RPS) -> u32 {
        match (self, v) {
            (RPS::Rock, RPS::Rock) => 1 + 3,
            (RPS::Rock, RPS::Paper) => 1 + 0,
            (RPS::Rock, RPS::Scissors) => 1 + 6,
            (RPS::Paper, RPS::Rock) => 2 + 6,
            (RPS::Paper, RPS::Paper) => 2 + 3,
            (RPS::Paper, RPS::Scissors) => 2 + 0,
            (RPS::Scissors, RPS::Rock) => 3 + 0,
            (RPS::Scissors, RPS::Paper) => 3 + 6,
            (RPS::Scissors, RPS::Scissors) => 3 + 3,
        }
    }
}

struct RPST {
    v: RPS,
}

impl RPST {
    fn new(rps: char) -> RPST {
        match rps {
            'A' | 'X' => RPST { v: RPS::Rock },
            'B' | 'Y' => RPST { v: RPS::Paper },
            'C' | 'Z' => RPST { v: RPS::Scissors },
            _ => unreachable!(),
        }
    }
}

fn rps_parse(p1: char, p2: char) -> (RPST, RPST) {
    let p1c = RPST::new(p1);
    let p2c = RPST::new(p2);
    (p1c, p2c)
}
fn main() -> std::io::Result<()> {
    let file = BufReader::new(File::open("puzzle input.txt")?);

    let mut p1score = 0 as u32;
    let mut p2score = 0 as u32;

    for line in file.lines() {
        let p1 = line.as_ref().unwrap().as_bytes()[0] as char;
        let p2 = line.as_ref().unwrap().as_bytes()[2] as char;

        let (player1, player2) = rps_parse(p1, p2);
        p1score = p1score + player1.v.eval(player2.v);
        p2score = p2score + player2.v.eval(player1.v);
    }

    println!("P1: {}   P2:  {}", p1score, p2score);

    Ok(())
}
