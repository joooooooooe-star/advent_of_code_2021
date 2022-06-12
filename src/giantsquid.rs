use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
struct BingoBoard {
    rows: Vec<HashSet<String>>,
    cols: Vec<HashSet<String>>,
    all_nums: HashSet<String>,
}

impl BingoBoard {
    pub fn new(text_block: &Vec<String>) -> Self {
        let mut rows: Vec<HashSet<String>> = vec![HashSet::new(); 5];
        let mut cols: Vec<HashSet<String>> = vec![HashSet::new(); 5];
        let mut all_nums: HashSet<String> = HashSet::new();
        let mut row_idx = 0;

        for line in text_block {
            let nums = line.split_whitespace();
            let mut col_idx = 0;
            for num in nums {
                cols[col_idx].insert(num.to_string());
                rows[row_idx].insert(num.to_string());
                all_nums.insert(num.to_string());
                col_idx += 1;
            }
            row_idx += 1;
        }
        Self {
            rows,
            cols,
            all_nums,
        }
    }

    pub fn score(&self, winning_num: i32, pulled_nums: &HashSet<String>) -> i32 {
        let num_iter = self.all_nums.difference(&pulled_nums);
        let score_iter = num_iter.map(|x| x.parse::<i32>().expect("not a number"));
        let prescore: i32 = score_iter.sum();
        winning_num * prescore
    }

    pub fn check_for_win(&self, pulled_nums: &HashSet<String>) -> bool {
        for row in &self.rows {
            if row.is_subset(pulled_nums) {
                return true;
            }
        }
        for col in &self.cols {
            if col.is_subset(pulled_nums) {
                return true;
            }
        }
        false
    }
}

pub fn play_bingo(filename: &str) {
    let file_path = Path::new(filename);
    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);
    let mut conv_line = buf_reader
        .lines()
        .map(|line| line.expect("Something went wrong"));

    let bingo_num_string: String = conv_line.next().expect("not bingo nums");

    let bingo_nums: Vec<String> = bingo_num_string
        .split(',')
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    conv_line.next();

    let mut bingoboards: Vec<BingoBoard> = Vec::new();
    let mut buf_vec: Vec<String> = Vec::new();
    for line in conv_line {
        if line == "" {
            let new_board = BingoBoard::new(&buf_vec);
            bingoboards.push(new_board);
            buf_vec.clear();
        } else {
            buf_vec.push(line)
        }
    }

    let mut pulled_nums = HashSet::new();
    let mut last_winning_score: i32 = 0;
    let mut idx_to_score: HashSet<usize> = HashSet::new();

    for num in bingo_nums.iter() {
        pulled_nums.insert(num.to_string());
        for board in bingoboards.iter().enumerate() {
            if !idx_to_score.contains(&board.0) {
                if board.1.check_for_win(&pulled_nums) {
                    last_winning_score = board.1.score(num.parse::<i32>().unwrap(), &pulled_nums);
                    idx_to_score.insert(board.0);
                }
            }
        }
    }
    println!("winning score is {}", last_winning_score);
}
