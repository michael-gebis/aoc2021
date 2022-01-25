use std::str::FromStr;
use crate::*;

pub fn day08_p1() {
    println!("day08_p1!");

    if let Ok(lines) = util::read_lines("src/day08/day08_example.txt") {
        for line in lines {
            if let Ok(ip) = dbg!(line) {
                let mut vals: Vec<usize> = ip.split(",").map(|x| x.parse().unwrap()).collect();
            }
        }
    }
}