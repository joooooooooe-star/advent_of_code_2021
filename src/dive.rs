use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

struct Position {
    pub position: i32,
    pub depth: i32,
    aim: i32,
}

pub fn dive(filename: &str) -> i32 {
    let file_path = Path::new(filename);
    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut pos = Position {
        position: 0,
        depth: 0,
        aim: 0,
    };

    for line in buf_reader.lines() {
        let line_str = line.unwrap();
        let command: Vec<&str> = line_str.split(' ').collect();
        let val = command[1].parse::<i32>().unwrap();
        match command[0] {
            "forward" => {
                pos.position += val;
                pos.depth += pos.aim * val
            }
            "down" => pos.aim += val,
            "up" => pos.aim -= val,
            _ => (),
        };
    }
    pos.position * pos.depth
}
