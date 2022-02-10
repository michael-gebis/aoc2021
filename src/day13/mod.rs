use crate::*;
use std::collections::HashSet;
use regex::Regex;

const FILENAME: &str = "src/day13/day13_input.txt";
//const FILENAME: &str = "src/day13/day13_example.txt";

fn foldx(val:usize, pairs:&mut HashSet<(usize,usize)>) {
    let mut newpairs:HashSet<(usize,usize)> = HashSet::new();
    for (x,y) in pairs.iter() {
        if *x < val {
            newpairs.insert((*x,*y));
        } else {
            let newx = val - (*x-val);
            newpairs.insert((newx,*y));
        }
    }
    pairs.clear();
    pairs.extend(newpairs);
}

fn foldy(val:usize, pairs:&mut HashSet<(usize,usize)>) {
    let mut newpairs:HashSet<(usize,usize)> = HashSet::new();
    for (x,y) in pairs.iter() {
        if *y < val {
            newpairs.insert((*x,*y));
        } else {
            let newy = val - (*y-val);
            newpairs.insert((*x,newy));
        }
    }
    pairs.clear();
    pairs.extend(newpairs);
}

fn draw(maxx:usize, maxy:usize, pairs:HashSet<(usize,usize)>) {
    let mut board:Vec<Vec<bool>> = vec![vec![false;maxx as usize];maxy as usize];

    for (x,y) in pairs {
        board[y][x] = true;
    }

    for row in 0..maxy {
        for col in 0..maxx {
            print!("{}", if board[row][col] {"#"} else {" "});
        }
        println!("");
    }

}

pub fn day13_p1() {
    println!("Day 13 Puzzle 1");

    let re_pairs = Regex::new(r"^(\d+),(\d+)$").unwrap();
    let re_folds = Regex::new(r"^fold along (.)=(\d+)$").unwrap();

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut pairs: HashSet<(usize,usize)> = HashSet::new();
        let mut xsize:usize = 0;
        let mut ysize:usize = 0;

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_pairs.captures(&ip) {
                    let (x,y):(usize,usize) =  (cap[1].parse().unwrap(), cap[2].parse().unwrap());
                    println!("inserting pair {},{}", x,y);
                    pairs.insert((x,y));

                } else if let Some(cap) = re_folds.captures(&ip) {
                    let (dir,val):(String,usize) = (cap[1].to_string(), cap[2].parse().unwrap());
                    println!("folding direction {} val {}", dir, val);

                    match dir.as_str() {
                        "x" => { xsize = val; foldx(val, &mut pairs)},
                        "y" => { ysize = val; foldy(val, &mut pairs)},
                        _ => panic!("Unknown direction!"),
                    };

                    println!("number of points={}", pairs.len());
                }
            }

            
        }
        draw(xsize, ysize, pairs);
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}