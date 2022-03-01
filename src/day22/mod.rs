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
        // off x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        let re_line =
            Regex::new(r"(o.+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)");

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
