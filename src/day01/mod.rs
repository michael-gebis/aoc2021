use std::str::FromStr;
use crate::*;

pub fn day1p1() {
    println!("Day 1 Puzzle 1");

    let mut prev: i32 = 99999;
    let mut count: i32 = 0;

    if let Ok(lines) = util::read_lines("data/day1_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cur: i32 = i32::from_str(&ip).unwrap();
                if cur > prev {
                    count += 1;
                }
                prev = cur;
                // println!("{}", cur);
            }
        }
    }
    println!("count={}", count);
}
pub fn day1p2() {
    println!("Day 1 Puzzle 2");

    let mut p1: i32 = -1;
    let mut p0: i32 = -1;

    let mut prevsum: i32 = 9999999;
    let mut count: i32 = 0;
    if let Ok(lines) = util::read_lines("data/day1_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cur: i32 = i32::from_str(&ip).unwrap();

                if p1 == -1 {
                    p1 = cur;
                } else if p0 == -1 {
                    p0 = cur;
                } else {
                    let sum: i32 = cur + p0 + p1;
                    if sum > prevsum {
                        count += 1;
                    }
                    p1 = p0;
                    p0 = cur;
                    prevsum = sum;
                }

                // println!("p1:{} p0:{} cur:{} prevsum:{}", p1, p0, cur, prevsum);
            }
        }
        println!("count={}", count);
    }
}