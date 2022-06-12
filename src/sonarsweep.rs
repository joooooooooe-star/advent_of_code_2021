use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[allow(dead_code)]
pub fn depth_increase(filepath: &str) -> i32 {
    let file_path = Path::new(filepath);
    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);
    let window_size: usize = 3;

    let mut measurement_window = VecDeque::new();

    let mut increase: i32 = 0;
    for line in buf_reader.lines() {
        let cmp = line.unwrap().parse::<i32>().unwrap();
        if measurement_window.len() < window_size {
            measurement_window.push_back(cmp);
            continue;
        }
        let last_value = measurement_window.iter().sum();
        measurement_window.pop_front();
        measurement_window.push_back(cmp);
        let cmp_value: i32 = measurement_window.iter().sum();

        if cmp_value > last_value {
            increase += 1;
        }
    }
    increase
}
