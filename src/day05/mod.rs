use crate::*;
use regex::Regex;
use std::cmp::Ord;

const GRIDSIZE:usize = 1000;

#[allow(dead_code)]
pub fn drawgrid(arr: &[[usize; GRIDSIZE]; GRIDSIZE]) {
    for i in 0..GRIDSIZE {
        for j in 0..GRIDSIZE {
            print!("{}", arr[i][j]);
        }
        println!("");
    }
}

pub fn day05_p1() {
    println!("day05_p1!");
    let re = Regex::new(r"^(\d+),(\d+)\s*->\s*(\d+),(\d+)$").unwrap();

    let mut arr: [[usize; GRIDSIZE]; GRIDSIZE] = [[0; GRIDSIZE]; GRIDSIZE];

    if let Ok(lines) = util::read_lines("src/day05/day05_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("Parsing '{}'", ip);
                let caps = re.captures(&ip).unwrap();
                println!("caps:{:?}", caps);
                let (startx, starty, endx, endy) : (usize,usize,usize,usize) = (
                    caps[1].parse().unwrap(),
                    caps[2].parse().unwrap(),
                    caps[3].parse().unwrap(),
                    caps[4].parse().unwrap());
                println!("startx:{} starty:{} endx:{} endy:{}", startx, starty, endx, endy);

                let (miny,maxy) = (starty.min(endy), starty.max(endy));
                let (minx,maxx) = (startx.min(endx), startx.max(endx));

                if startx == endx {
                    for j in miny..maxy+1 {
                        arr[startx][j] += 1
                    }
                } else if starty == endy {
                    for i in minx..maxx+1 {
                        arr[i][starty] += 1
                    }
                } else {
                    if (endx > startx && endy > starty) || (endx < startx && endy < starty) {
                        println!("diagonal A");
                        let diff = maxx - minx;
                        for i in 0..(diff+1) {
                            arr[minx+i][miny+i] += 1
                        }
                    } else {
                        println!("diagonal B");
                        let diff = maxx - minx;
                        for i in 0..(diff+1) {
                            arr[minx+i][maxy-i] += 1
                        }
                    }
                }

                //drawgrid(&arr);
            }
        }
    }

    let mut count = 0;
    for i in 0..GRIDSIZE {
        for j in 0..GRIDSIZE {
            if arr[i][j] > 1 {
                // println!("arr[{}][{}] = {}", i, j, arr[i][j]);
                count += 1;
            }
        }
    }
    println!("count={}", count);
}
