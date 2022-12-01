#![feature(str_split_as_str)]

use std::fs;
use std::iter::Map;
use std::str::Split;

fn main() {
    let data = read_file();
    let split_data: Vec<String> = data.split("\n\n").map(|s| s.to_string()).collect();
    let mut biggest= 0;
    for block in split_data {
        let sub_blocks: Vec<String> = block.split("\n").map(|s| s.to_string()).collect();
        let mut total = 0;
        for sub in sub_blocks {
            let integer = sub.parse::<i32>().unwrap();
            total += integer
        }

        if total > biggest {
            biggest = total
        }
    }

    println!("biggest: {biggest}")



}

fn read_file() -> String {
    fs::read_to_string("input.txt").expect("Unable to find / read file")
}
