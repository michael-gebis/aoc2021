use crate::*;
use regex::Regex;

const FILENAME: &str = "src/day21/day21_input.txt";
//const FILENAME: &str = "src/day21/day21_example.txt";

#[allow(dead_code)]
pub fn day21_p1() {
    println!("Day 21 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut rules: Vec<i32> = Vec::new();
        let mut board: Vec<Vec<i32>> = Vec::new();
        //let re_rules = Regex::new(r"^([\.#]{512})$").unwrap();
        let re_image_row = Regex::new(r"^([\.#]+)$").unwrap();

        for line in lines {
            if let Ok(ip) = line {
                //if let Some(cap) = re_rules.captures(&ip) {
            }
        }
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
