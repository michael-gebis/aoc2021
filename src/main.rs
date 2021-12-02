use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn day1p1() {
    println!("Day 1 Puzzle 1");

    let mut prev:i32 = 99999;
    let mut count:i32 = 0;

    if let Ok(lines) = read_lines("data/day1_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cur:i32 = i32::from_str(&ip).unwrap();
                if cur > prev {
                    count += 1;
                }
                prev=cur;
                // println!("{}", cur);
            }
        }
    }
    println!("count={}",count);
}

fn day1p2() {
    println!("Day 1 Puzzle 2");
    
    let mut p1:i32 = -1;
    let mut p0:i32 = -1;

    let mut prevsum:i32 = 9999999;
    let mut count:i32 = 0;

    if let Ok(lines) = read_lines("data/day1_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cur:i32 = i32::from_str(&ip).unwrap();

                if p1 == -1 {
                    p1 = cur;
                }
                else if p0 == -1 {
                    p0 = cur;
                }
                else {
                    let sum:i32 = cur + p0 + p1;
                    
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
        println!("count={}",count);
    }
}

fn day2p1() {
    println!("Day 2 Puzzle 1");

    let mut hpos:i32 = 0;
    let mut depth:i32 = 0;

    if let Ok(lines) = read_lines("data/day2_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(" ").collect();
                let dir:&str = v[0];
                let val:i32 = i32::from_str(v[1]).unwrap();
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
    println!("hpos:{} depth:{} answer:{}", hpos, depth, hpos*depth);
}

fn day2p2() {
    println!("Day 2 Puzzle 2");

    let mut hpos:i32 = 0;
    let mut depth:i32 = 0;
    let mut aim:i32 = 0;

    if let Ok(lines) = read_lines("data/day2_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(" ").collect();
                let dir:&str = v[0];
                let val:i32 = i32::from_str(v[1]).unwrap();
                // println!("dir:{} val:{}", dir, val);

                match dir {
                    "forward" => { hpos += val; depth += aim * val;},
                    "up" => aim -= val,
                    "down" => aim += val,
                    _ => println!("bad input"),
                }

                // println!("hpos:{} depth:{} aim:{}", hpos, depth, aim);
            }
        }
    }
    println!("hpos:{} depth:{} aim:{} answer:{}", hpos, depth, aim, hpos*depth);
}

fn main() {
    println!("Hello, world!");
    day1p1();
    day1p2();

    day2p1();
    day2p2();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}