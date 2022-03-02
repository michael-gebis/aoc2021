use crate::*;
use regex::Regex;
use std::collections::HashMap;

//const FILENAME: &str = "src/day22/day22_input.txt";
const FILENAME: &str = "src/day22/day22_exampleA.txt";
//const FILENAME: &str = "src/day22/day22_exampleB.txt";

#[derive (Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    pub fn new(start: i64, end: i64) -> Range {
        if end < start {
            panic!("end < start!!!");
            // Should we auto-swap?  For now, no.
            // Range {start:end, end:start}
        } else {
            Range {
                start: start,
                end: end,
            }
        }
    }

    pub fn width(&self) -> i64 {
        self.end - self.start
    }
}

#[derive (Debug)]
struct Cube {
    xrange: Range,
    yrange: Range,
    zrange: Range,
}

impl Cube {
    pub fn new(xstart: i64, xend: i64, ystart: i64, yend: i64, zstart: i64, zend: i64) -> Cube {
        Cube {
            xrange: Range::new(xstart, xend),
            yrange: Range::new(ystart, yend),
            zrange: Range::new(zstart, zend),
        }
    }

    fn volume(&self) -> i64 {
        self.xrange.width() * self.yrange.width() * self.zrange.width()
    }

    fn check_intersect(&self, other: &Cube) -> bool {
        false
    }

    // Slice?
}

#[allow(dead_code)]
pub fn day22_p1() {
    println!("Day 22 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        // Looks like: off x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        let re_cube =
            Regex::new(r"(o.+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_cube.captures(&ip) {
                    //println!("cap:{:?}",cap);
                    let cube = Cube::new(
                        cap[2].parse::<i64>().unwrap(),
                        cap[3].parse::<i64>().unwrap(),
                        cap[4].parse::<i64>().unwrap(),
                        cap[5].parse::<i64>().unwrap(),
                        cap[6].parse::<i64>().unwrap(),
                        cap[7].parse::<i64>().unwrap());
                    println!("Found cube: type:{}, {:?}", &cap[1], cube);
                } else {
                    println!("Ignoring line '{}'", ip);
                }
            }
        }
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
