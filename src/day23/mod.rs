use crate::*;
use regex::Regex;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

//const FILENAME: &str = "src/day23/day23_example.txt";
const FILENAME: &str = "src/day23/day23_input.txt";

#[allow(dead_code)]
pub fn day23_p1() {
    println!("Day 23 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        //let re_cube =
            Regex::new(r"(o.+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
                .unwrap();
        //let mut pending_cubes: VecDeque<Cube> = VecDeque::new();

        for line in lines {
            if let Ok(ip) = line {
                println!("Ignoring '{}'", ip);
            }
        }
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
