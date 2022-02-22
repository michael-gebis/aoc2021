use crate::*;
//use std::fmt;

//const FILENAME: &str = "src/day19/day19_input.txt";
const FILENAME: &str = "src/day19/day19_example.txt";


#[allow(dead_code)]
pub fn day19_p1() {
    println!("Day 19 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        //let mut total:Option<BinTreeNode> = None;
        for line in lines {
            if let Ok(ip) = line {
            }
        }
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}