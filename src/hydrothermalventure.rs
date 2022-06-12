use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(pt_str: &str) -> Self {
        let nums: Vec<i32> = pt_str
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        Self {
            x: nums[0],
            y: nums[1],
        }
    }

    fn get_line(&self, other: &Point) -> Vec<(i32, i32)> {
        let mut ret = Vec::new();
        if self.x == other.x {
            let (r1, r2) = (cmp::min(self.y, other.y), cmp::max(self.y, other.y));
            for n in r1..=r2 {
                let linepoint = (self.x, n);
                ret.push(linepoint);
            }
        } else if self.y == other.y {
            let (r1, r2) = (cmp::min(self.x, other.x), cmp::max(self.x, other.x));
            for n in r1..=r2 {
                let linepoint = (n, self.y);
                ret.push(linepoint);
            }
        } else if ((self.y - other.y) / (self.x - other.x)).abs() == 1 {
            let diff = (self.x - other.x).abs();
            let x_slope = if self.x > other.x { -1 } else { 1 };
            let y_slope = if self.y > other.y { -1 } else { 1 };
            for n in 0..=diff {
                let linepoint = (self.x + x_slope * n, self.y + y_slope * n);
                ret.push(linepoint)
            }
        }
        ret
    }
}

pub fn find_overlap(filename: &str) {
    let file_path = Path::new(filename);
    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut pointmap = HashMap::new();
    for line in buf_reader.lines() {
        let pstring = line.expect("not a point");
        let mut pstring_split = pstring.split(" -> ");
        let p1 = Point::new(pstring_split.next().unwrap());
        let p2 = Point::new(pstring_split.next().unwrap());

        let line = p1.get_line(&p2);
        for point in line {
            if let Some(n) = pointmap.get_mut(&point) {
                *n += 1;
            } else {
                pointmap.insert(point, 1);
            }
        }
    }
    let mut overlap = 0;
    for val in pointmap.values() {
        if val > &1 {
            overlap += 1;
        }
    }
    println!("There are {} overlaps", overlap);
}
