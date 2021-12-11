use crate::aoc::common::AoC;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct DayOne {
    values: Vec<i32>,
}

impl DayOne {
    pub fn new() -> Box<dyn AoC> {
        let mut values: Vec<i32> = Vec::new();

        let file = File::open("data/d01.txt").unwrap();
        let lines = BufReader::new(file).lines();

        for line in lines{
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };
            match line.trim().parse() {
                Ok(t) => values.push(t),
                Err(_) => continue,
            };
        }

        Box::new(DayOne {
            values
        })
    }
}

impl AoC for DayOne {
    fn one(&self) -> i32 {
        let mut old_value: i32 = 0;
        let mut count: i32 = -1;

        for value in &self.values {
            if *value > old_value {
                count += 1;
            }
            old_value = *value;
        }

        count
    }

    fn two(&self) -> i32 {
        let mut cursor: Vec<i32> = Vec::new();

        let mut old_sum: i32 = 0;
        let mut new_sum: i32 = 0;

        let mut count: i32 = 0;

        for value in &self.values[..3] {
            new_sum += value;
            old_sum += value;

            cursor.push(*value);
        }

        for value in &self.values[3..]{

            cursor.push(*value);
            new_sum += value;
            new_sum -= cursor.get(0).unwrap();
            cursor.remove(0);

            if new_sum > old_sum {
                count += 1;
            }
            old_sum = new_sum;
        }

        count
    }
}