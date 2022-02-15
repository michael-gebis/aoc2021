use crate::*;
use std::collections::HashMap;
use regex::Regex;

//const FILENAME: &str = "src/day16/day16_input.txt";
const FILENAME: &str = "src/day16/day16_example.txt";

#[allow(dead_code)]
pub fn day16_p1() {
    println!("Day 16 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut rules: HashMap<(String, String), String> = HashMap::new();
        let mut polymer:Vec<String> = Vec::new();
        let re_insertion_rules = Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();
        let re_polymer_template = Regex::new(r"^([A-Z]+)$").unwrap();

        for line in lines {
            if let Ok(ip) = line {
            }
        }
    }
    else { 
        panic!("Couldn't open {}", FILENAME);
    }
}