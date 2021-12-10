use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct DayOne {
    values: Vec<i32>,
}

impl DayOne {
    pub fn new() -> DayOne {
        let mut values: Vec<i32> = Vec::new();

        let file = File::open("data/d01.txt").unwrap();
        let mut buf_reader = BufReader::new(file);

        loop {
            let mut value = String::new();
            match buf_reader.read_line(&mut value) {
                Ok(_) => {}
                Err(_) => { break; }
            };

            match value.trim().parse() {
                Ok(t) => values.push(t),
                Err(_) => { break; },
            };
        }

        DayOne {
            values
        }
    }

    pub fn one(&self) -> i32 {
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

    pub fn two(&self) -> i32 {
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