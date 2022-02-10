use crate::*;
use itertools::Itertools;
use std::collections::HashMap;

const FILENAME: &str = "src/day13/day13_input.txt";
//const FILENAME: &str = "src/day13/day13_example.txt";

pub fn day13_p1() {
    println!("Day 13 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut neighbors: HashMap<String, Vec<String>> = HashMap::new();
        let mut count = 0;
        for line in lines {
            //let mut mystack: Vec<&str> = Vec::new();
            if let Ok(ip) = line {
                //let (left, right) = ip.trim().split("-").collect_tuple().unwrap();
            }
        }
    }
}