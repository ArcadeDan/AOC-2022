use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{stdin, BufRead, BufReader},
    str::FromStr,
};

use anyhow::Ok;
use anyhow::Result;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
struct Move(Direction, usize);

impl FromStr for Move {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s[0..1].parse()?, s[2..].parse()?))
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use anyhow::anyhow;
        Ok(match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => Err(anyhow!("failed to parse: {}", s))?,
        })
    }
}
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn adjustment(&self, head: &Self) -> (isize, isize) {
        let (tx, ty) = (self.x, self.y);
        let (hx, hy) = (head.x, head.y);
        (
            match tx.cmp(&hx) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                _ => 1,
            },
            match ty.cmp(&hy) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                _ => 1,
            },
        )
    }
    fn adjust(&mut self, head: &Self) {
        let needs_adjustment = ![1, 0, -1]
            .into_iter()
            .flat_map(|x| [1, 0, -1].into_iter().map(move |y| (x, y)))
            .any(|(dx, dy)| head.x + dx == self.x && head.y + dy == self.y);
        if needs_adjustment {
            let (adjx, adjy) = self.adjustment(head);
            self.x += adjx;
            self.y += adjy;
        }
    }
}

struct Rope {
    head: Point,
    tail: Vec<Point>,
}

impl Rope {
    fn new(tailcount: usize) -> Self {
        Self {
            head: Point { x: 0, y: 0 },
            tail: vec![Point { x: 0, y: 0 }; tailcount],
        }
    }
    fn step(&mut self, direction: &Direction) {
        use Direction::*;
        match direction {
            Left => {
                self.head.x -= 1;
            }
            Right => {
                self.head.x += 1;
            }
            Up => {
                self.head.y -= 1;
            }
            Down => {
                self.head.y += 1;
            }
        }
        self.adjust_tails()
    }
    fn adjust_tails(&mut self) {
        self.tail.iter_mut().fold(&self.head, |head, tail| {
            tail.adjust(head);
            tail
        });
    }
    fn final_tail(&self) -> Option<Point> {
        self.tail.last().copied()
    }
}

fn read(reader: impl BufRead) -> impl Iterator<Item = Result<Move>> {
    use anyhow::anyhow;
    reader.lines().map(|line| {
        line.map_err(Into::into)
            .and_then(|line| line.parse())
            .map_err(|e| anyhow!("failed to parse line: {}", e))
    })
}
fn main() -> Result<()> {
    //let file = File::open("puzzle input.txt")?;
    //let reader = BufReader::new(file);
    let mut rope = Rope::new(1);
    let mut biggerrope = Rope::new(9);
    let mut set = HashSet::from([Point { x: 0, y: 0 }]);
    let mut bigger_set: HashSet<Point> = HashSet::from([Point { x: 0, y: 0 }]);

    for move_cmd in read(BufReader::new(stdin())) {
        let Move(direction, step) = move_cmd?;
        (0..step).for_each(|_| {
            rope.step(&direction);
            rope.final_tail().into_iter().for_each(|tail| {
                set.insert(tail);
            });

            biggerrope.step(&direction);
            biggerrope.final_tail().into_iter().for_each(|tail| {
                bigger_set.insert(tail);
            })
        })
    }
    println!("Rope 1: {}", set.len());
    println!("Rope 2: {}", bigger_set.len());
    Ok(())
}
