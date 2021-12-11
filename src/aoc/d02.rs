use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::aoc::common::AoC;

enum Direction {
    Forward,
    Down,
    Up,
}

pub struct DayTwo {
    values: Vec<(Direction, i32)>,
}

impl DayTwo {
    pub fn new() -> Box<dyn AoC> {
        let f = File::open("data/d02.txt").unwrap();
        let lines = BufReader::new(f).lines();

        let mut values: Vec<(Direction, i32)> = Vec::new();

        for line in lines {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };
            let s: Vec<&str> = line.trim().split(" ").collect();
            if s.len() != 2 {
                continue;
            }
            let direction: Direction = match s.get(0).unwrap() {
                &"forward" => Direction::Forward,
                &"down" => Direction::Down,
                &"up" => Direction::Up,
                _ => continue,
            };

            let speed: i32 = match s.get(1).unwrap().parse() {
                Ok(i) => i,
                Err(_) => continue,
            };

            values.push((direction, speed));
        }
        Box::new(DayTwo {
            values
        })
    }
}

impl AoC for DayTwo {
    fn one(&self) -> i32 {
        let mut pos = (0, 0);
        for value in &self.values {
            match value.0 {
                Direction::Forward => pos.0 += value.1,
                Direction::Up => pos.1 -= value.1,
                Direction::Down => pos.1 += value.1,
            }
        }

        pos.0 * pos.1
    }

    fn two(&self) -> i32 {
        let mut pos = (0, 0, 0);
        for value in &self.values {
            match value.0 {
                Direction::Forward => {
                    pos.0 += value.1;
                    pos.1 += pos.2 * value.1;
                },
                Direction::Up => pos.2 -= value.1,
                Direction::Down => pos.2 += value.1,
            }
        }
        return pos.0 * pos.1;
    }
}