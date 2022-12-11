use std::{
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

struct CRT {
    cpu: CPU,
    video: Video,
}

impl CRT {
    fn new() -> Self {
        Self {
            cpu: CPU::new(),
            video: Video::new(),
        }
    }
}

struct Video {
    videobuffer: Vec<char>,
}

impl Video {
    fn new() -> Self {
        Self {
            videobuffer: Vec::new(),
        }
    }
}

struct CPU {
    x: isize, // x register
    cycles: isize,
    signal_sum: isize,
}

impl CPU {
    fn new() -> Self {
        Self {
            x: 1,
            cycles: 0,
            signal_sum: 0,
        }
    }

    fn incrcycle(&mut self) {
        self.cycles += 1;
        match self.cycles {
            20 => self.signal_sum += self.cycles * self.x,
            60 => self.signal_sum += self.cycles * self.x,
            100 => self.signal_sum += self.cycles * self.x,
            140 => self.signal_sum += self.cycles * self.x,
            180 => self.signal_sum += self.cycles * self.x,
            220 => self.signal_sum += self.cycles * self.x,
            _ => {}
        }
    }

    // 2 cycles
    fn addx(&mut self, v: isize) {
        self.incrcycle();

        self.incrcycle();

        self.x += v;
    }
    // 1 cycle
    fn noop(&mut self) {
        self.incrcycle();
    }

    fn execute(&mut self, op: Option<&str>, operand: Option<&str>) {
        match (op, operand) {
            (None, None) => unreachable!(),
            (None, Some(_)) => unreachable!(),
            (Some(_), None) => self.noop(),
            (Some(_), Some(_)) => self.addx(
                operand
                    .unwrap()
                    .parse::<i32>()
                    .ok()
                    .unwrap()
                    .try_into()
                    .unwrap(),
            ),
        }
    }
}

fn main() {
    let mut Arcade = CRT::new();
    Arcade.video.videobuffer.resize(40, '.');
    let file = std::fs::read_to_string("given.txt").unwrap();

    for line in file.lines() {
        let operation = line.get(0..4);
        if line.len() > 4 {
            let operand = line.get(5..);
            Arcade.cpu.execute(operation, operand);
        } else {
            Arcade.cpu.execute(operation, None::<&str>);
        }
    }
    println!("CPU Signal Sum: {}", Arcade.cpu.signal_sum);
}
