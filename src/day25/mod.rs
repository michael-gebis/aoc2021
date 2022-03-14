use crate::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
//use std::mem;

const FILENAME: &str = "src/day25/day25_example.txt";
//const FILENAME: &str = "src/day25/day25_input.txt";
//const FILENAME: &str = "src/day24/day24_mini1.txt";


pub fn day25_p1() {
    println!("Day 25 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let re_comment = Regex::new(r"^#.*").unwrap();
        let re_opinp = Regex::new(r"^(inp) ([wxyz])$").unwrap();
        let re_opregval = Regex::new(r"^(...) ([wxyz]) (-?\d+)$").unwrap();
        let re_opregreg = Regex::new(r"^(...) ([wxyz]) ([wxyz])$").unwrap();

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_opinp.captures(&ip) {
                    //prog.push_back(Op::new_1op(&cap[1], &cap[2]));
                }
            }
        }
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}