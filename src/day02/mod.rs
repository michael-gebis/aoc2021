use crate::*;
use std::str::FromStr;

#[allow(dead_code)]
pub fn day02_p1() {
    println!("Day 2 Puzzle 1");

    let mut hpos: i32 = 0;
    let mut depth: i32 = 0;

    if let Ok(lines) = read_lines("src/day02/day02_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(" ").collect();
                let dir: &str = v[0];
                let val: i32 = i32::from_str(v[1]).unwrap();
                // println!("dir:{} val:{}", dir, val);

                match dir {
                    "forward" => hpos += val,
                    "up" => depth -= val,
                    "down" => depth += val,
                    _ => println!("bad input"),
                }
            }
        }
    }
    println!("hpos:{} depth:{} answer:{}", hpos, depth, hpos * depth);
}

#[allow(dead_code)]
pub fn day02_p2() {
    println!("Day 2 Puzzle 2");

    let mut hpos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    if let Ok(lines) = read_lines("src/day02/day02_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(" ").collect();
                let dir: &str = v[0];
                let val: i32 = i32::from_str(v[1]).unwrap();
                // println!("dir:{} val:{}", dir, val);

                match dir {
                    "forward" => {
                        hpos += val;
                        depth += aim * val;
                    }
                    "up" => aim -= val,
                    "down" => aim += val,
                    _ => println!("bad input"),
                }

                // println!("hpos:{} depth:{} aim:{}", hpos, depth, aim);
            }
        }
    }
    println!(
        "hpos:{} depth:{} aim:{} answer:{}",
        hpos,
        depth,
        aim,
        hpos * depth
    );
}
