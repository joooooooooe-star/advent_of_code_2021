use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

enum Sensor {
    CO2,
    O2,
}
pub fn binarydiagnostic(filename: &str, binsize: usize) {
    let file_path = Path::new(filename);
    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut num_poll = vec![0; binsize];
    let conv_line: Vec<String> = buf_reader
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();
    for line in conv_line.iter() {
        let char_iter = line.graphemes(true);
        let mut idx: usize = 0;
        for chr in char_iter {
            let val = if chr == "1" { 1 } else { -1 };
            num_poll[idx] += val;
            idx += 1;
        }
    }
    let num = gamma_eps_calc(&num_poll);
    println!("Product of epsilon and gamma is {}", num);
    let life = life_support_calc(&conv_line);
    println!("Product of o2 and co2 is {}", life);
}

fn gamma_eps_calc(num_poll: &Vec<i32>) -> i32 {
    let mut gamma_string = String::new();
    let mut eps_string = String::new();
    for elem in num_poll {
        if elem > &0 {
            gamma_string.push('1');
            eps_string.push('0');
        } else if elem < &0 {
            gamma_string.push('0');
            eps_string.push('1');
        }
    }
    let gamma_intval = i32::from_str_radix(gamma_string.as_str(), 2).unwrap();
    let eps_intval = i32::from_str_radix(eps_string.as_str(), 2).unwrap();

    gamma_intval * eps_intval
}

fn life_support_calc(conv_line: &Vec<String>) -> i32 {
    let o2_val = life_calc(conv_line, Sensor::O2);
    let co2_val = life_calc(conv_line, Sensor::CO2);
    let o2_intval = i32::from_str_radix(o2_val.as_str(), 2).unwrap();
    let co2_intval = i32::from_str_radix(co2_val.as_str(), 2).unwrap();

    o2_intval * co2_intval
}

fn life_calc(conv_line: &Vec<String>, sensor_type: Sensor) -> String {
    let mut idx = 0;
    let mut res: Vec<String> = conv_line.clone();
    loop {
        let (bin_zero, bin_one): (Vec<String>, Vec<String>) = res
            .into_iter()
            .partition(|n| n.graphemes(true).nth(idx).unwrap() == "0");

        match sensor_type {
            Sensor::O2 => {
                if bin_one.len() >= bin_zero.len() {
                    res = bin_one.clone();
                } else {
                    res = bin_zero.clone();
                }
                if res.len() == 1 {
                    break;
                }
            }
            Sensor::CO2 => {
                if bin_one.len() < bin_zero.len() {
                    res = bin_one.clone();
                } else {
                    res = bin_zero.clone();
                }
                if res.len() == 1 {
                    break;
                }
            }
        }

        if res.len() < 1 {
            panic!("life is less than zero");
        }
        if res.len() == 0 {
            break;
        }
        idx += 1;
    }
    res[0].clone()
}
