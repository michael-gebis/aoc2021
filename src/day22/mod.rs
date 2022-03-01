use crate::*;
use regex::Regex;
use std::collections::HashMap;

//const FILENAME: &str = "src/day22/day22_input.txt";
const FILENAME: &str = "src/day22/day22_exampleA.txt";
//const FILENAME: &str = "src/day22/day22_exampleB.txt";

#[allow(dead_code)]
pub fn day22_p1() {
    println!("Day 22 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {

        let re_player1 = Regex::new(r"^Player 1 starting position: (\d+)$").unwrap();
        let re_player2 = Regex::new(r"^Player 2 starting position: (\d+)$").unwrap();

        for line in lines {
            if let Ok(ip) = line {
                /*
                if let Some(cap) = re_player1.captures(&ip) {
                    p1_pos = cap[1].parse().unwrap();
                }
                */
                println!("Ignoring line '{}'", ip);
            }
        }
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}