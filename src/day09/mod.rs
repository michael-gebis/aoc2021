use std::collections::HashSet;
use crate::*;

const FILENAME: &str = "src/day09/day09_input.txt";
//const FILENAME: &str = "src/day09/day09_example.txt";

// Trying something a little clever: a North/East/West/South iterator
struct NEWS {
    origin: (i32, i32),
    boardsize: (i32, i32),
    count: u32, // 0=N, 1=E, 2=S, 3=W
}

impl Iterator for NEWS {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            self.count += 1;

            if self.origin.1 - 1 >= 0 {
                return Some((self.origin.0, self.origin.1 - 1));
            }
        }
        if self.count == 1 {
            self.count += 1;
            if self.origin.0 + 1 < self.boardsize.0 {
                return Some((self.origin.0 + 1, self.origin.1));
            }
        }
        if self.count == 2 {
            self.count += 1;
            if self.origin.1 + 1 < self.boardsize.1 {
                return Some((self.origin.0, self.origin.1 + 1));
            }
        }

        if self.count == 3 {
            self.count += 1;
            if self.origin.0 - 1 >= 0 {
                return Some((self.origin.0 - 1, self.origin.1));
            }
        }

        None
    }
}

fn news(origin: (i32, i32), boardsize: (i32, i32)) -> NEWS {
    NEWS {
        origin: origin,
        boardsize: boardsize,
        count: 0,
    }
}

// Note that the definition says that every point is a part of ONE basin;
// this means there are no saddle points, and that the basin is defined by
// a border of "9"s.  So this simplifies the search a bit
pub fn basinsearch(board: &Vec<Vec<i32>>, boardsize: (i32, i32), pos: (i32, i32)) -> i32 {
    let mut todo: Vec<(i32, i32)> = Vec::new();
    let mut done: HashSet<(i32, i32)> = HashSet::new();
    let mut count = 0;
    todo.push(pos);

    while !todo.is_empty() {
        //count += 1;
        let x = todo.pop().unwrap();
        if done.contains(&x) {
            continue;
        }
        count += 1;
        println!("processing {},{}", x.0, x.1);
        for zzz in news(x, boardsize) {
            if !done.contains(&zzz) /*&& !todo.contains(&zzz)*/ && board[zzz.1 as usize][zzz.0 as usize] != 9
            {
                println!("  added {},{} to todo", zzz.0, zzz.1);
                todo.push(zzz);
            }
        }
        done.insert(x);
    }
    dbg!(count);
    count
}
#[allow(dead_code)]
pub fn day09_p1() {
    println!("Day 9 Puzzle 1");

    for (x, y) in news((0, 0), (100, 100)) {
        println!("x={}, y={}", x, y);
    }

    let boardsize: (i32, i32);
    let mut linecount = 0;
    let mut board: Vec<Vec<i32>> = Vec::new();
    let mut rowsize: i32 = 0;
    if let Ok(lines) = util::read_lines(FILENAME) {
        for line in lines {
            if let Ok(ip) = line {
                linecount += 1;
                let row: Vec<i32> = ip
                    .trim()
                    .split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap())
                    .collect();
                if rowsize == 0 {
                    rowsize = row.len() as i32;
                } else {
                    assert_eq!(rowsize as usize, row.len());
                }
                board.push(row);
            }
        }
        boardsize = (rowsize, linecount);
        dbg!(boardsize);
    } else {
        panic!("couldn't open {}", FILENAME)
    }

    let mut basins: Vec<i32> = Vec::new();
    let mut risk: i32 = 0;
    for row in 0..linecount {
        for col in 0..rowsize {
            let mut gt = 0;
            let mut _lt = 0;
            let mut eq = 0;
            let val = board[row as usize][col as usize];
            println!("Val at {},{}: {}", col, row, val);
            for (x, y) in news((col, row), boardsize) {
                let v2 = board[y as usize][x as usize];
                println!(" compare to neighbor at {},{}: {}", x, y, v2);
                if val < v2 {
                    _lt += 1;
                } else if val > v2 {
                    gt += 1;
                } else {
                    eq += 1;
                }
            }
            if gt == 0 && eq == 0 {
                risk += val + 1;
                println!(
                    "{},{} is a low point.  val={}, risk={}",
                    row, col, val, risk
                );

                basins.push(basinsearch(&board, boardsize, (col, row)));
            }
        }
    }
    println!("risk={}", risk);
    basins.sort();
    println!(
        "basins[0]={}, basins[1]={}, basins2={}, product={}",
        basins[0].clone(),
        basins[1].clone(),
        basins[2].clone(),
        basins.into_iter().rev().take(3).product::<i32>()
    );

}
