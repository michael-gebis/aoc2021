use crate::*;
use std::collections::HashMap;
use regex::Regex;

const FILENAME: &str = "src/day18/day18_input.txt";
//const FILENAME: &str = "src/day18/day18_example.txt";

#[allow(dead_code)]
pub fn day18_p1() {
    println!("Day 18 Puzzle 2");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut rules: HashMap<(String, String), String> = HashMap::new();
        let mut polymer:Vec<String> = Vec::new();
        let re_insertion_rules = Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();
        let re_polymer_template = Regex::new(r"^([A-Z]+)$").unwrap();
        let mut pair_counts: HashMap<(String, String), i64> = HashMap::new();

    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}