use crate::*;
use std::collections::HashSet;
use regex::Regex;

const FILENAME: &str = "src/day14/day14_input.txt";
//const FILENAME: &str = "src/day14/day14_example.txt";

pub fn day14_p1() {
    println!("Day 14 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut pairs: HashSet<(usize,usize)> = HashSet::new();
        let mut xsize:usize = 0;
        let mut ysize:usize = 0;

        for line in lines {
            if let Ok(ip) = line {
            }
        }
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}