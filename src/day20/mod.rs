use crate::*;
use regex::Regex;

const FILENAME: &str = "src/day20/day20_input.txt";
//const FILENAME: &str = "src/day20/day20_example.txt";

pub fn print_board(board: &Vec<Vec<i32>>) {
    for row in board {
        for c in row {
            print!("{}", if *c == 1 { "#" } else { "." })
        }
        println!("");
    }
}

pub fn get_val(board: &Vec<Vec<i32>>, oob: i32, x: i32, y: i32) -> i32 {
    let xsize = board[0].len() as i32;
    let ysize = board.len() as i32;
    let val: i32;
    if x < 0 || x >= xsize || y < 0 || y >= ysize {
        val = oob;
    } else {
        val = board[y as usize][x as usize];
    }

    val
}

pub fn get_next_state(rules: &Vec<i32>, board: &Vec<Vec<i32>>, oob: i32, x: i32, y: i32) -> i32 {
    let mut val: i32 = 0;

    for tmpy in (y - 1)..(y + 2) {
        for tmpx in (x - 1)..(x + 2) {
            val *= 2;
            val += get_val(board, oob, tmpx, tmpy);
        }
    }
    rules[val as usize]
}

pub fn count_pixels(board: &Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for row in board {
        for col in row {
            if *col != 0 {
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
pub fn day20_p1() {
    println!("Day 20 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut rules: Vec<i32> = Vec::new();
        let mut board: Vec<Vec<i32>> = Vec::new();
        let re_rules = Regex::new(r"^([\.#]{512})$").unwrap();
        let re_image_row = Regex::new(r"^([\.#]+)$").unwrap();

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_rules.captures(&ip) {
                    rules = cap[1]
                        .chars()
                        .collect::<Vec<char>>()
                        .iter()
                        .map(|x| if *x == '#' { 1 } else { 0 })
                        .collect();
                } else if let Some(cap) = re_image_row.captures(&ip) {
                    let row = cap[1]
                        .chars()
                        .collect::<Vec<char>>()
                        .iter()
                        .map(|x| if *x == '#' { 1 } else { 0 })
                        .collect();
                    board.push(row);
                } else {
                    // ignore
                }
            }
        }
        print_board(&board);

        let mut oob: i32 = 0;
        for _i in 0..50 {
            let mut new_board: Vec<Vec<i32>> = Vec::new();

            let xsize = (board[0].len() + 2) as i32;
            let ysize = (board.len() + 2) as i32;

            for row in 0..ysize {
                let mut new_row: Vec<i32> = Vec::new();
                for col in 0..xsize {
                    new_row.push(get_next_state(&rules, &board, oob, col - 1, row - 1));
                }
                new_board.push(new_row);
            }
            board = new_board;
            oob = get_next_state(&rules, &board, oob, -2, -2);
        }
        print_board(&board);
        println!("let pixel count = {}", count_pixels(&board));
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}
