use crate::*;

const FILENAME: &str = "src/day11/day11_input.txt";
//const FILENAME: &str = "src/day11/day11_exampleB.txt";
//const FILENAME: &str = "src/day11/day11_exampleA.txt";

struct NeighborIter {
    origin: (i32, i32),        // col,row
    boardsize: (usize, usize), // rows and columns
    count: usize,
}

impl Iterator for NeighborIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let offsets: [(i32, i32); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            /*(0,0),*/
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        while self.count < 8 {
            let pos: (i32, i32) = (
                self.origin.0 as i32 + offsets[self.count].0 as i32,
                self.origin.1 as i32 + offsets[self.count].1 as i32,
            );
            self.count += 1;

            if pos.0 < 0
                || pos.0 > self.boardsize.0 as i32 - 1
                || pos.1 < 0
                || pos.1 > self.boardsize.1 as i32 - 1
            {
                continue;
            }
            return Some(pos);
        }

        None
    }
}

fn neighbor_iter(origin: (i32, i32), boardsize: (usize, usize)) -> NeighborIter {
    NeighborIter {
        origin: origin,
        boardsize: boardsize,
        count: 0,
    }
}

fn print_board(board: &Vec<Vec<i32>>, boardsize: (usize, usize)) {
    for row in 0..boardsize.0 {
        for col in 0..boardsize.1 {
            print!("{}", board[row][col]);
        }
        println!("");
    }
}

#[allow(dead_code)]
pub fn day11_p1() {
    println!("Day 11 Puzzle 1");

    for (row, col) in neighbor_iter((0, 0), (100, 100)) {
        println!("row={}, col={}", row, col);
    }

    let boardsize: (usize, usize);
    let mut linecount: usize = 0;
    let mut board: Vec<Vec<i32>> = Vec::new();
    let mut rowsize: usize = 0;
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
                    rowsize = row.len();
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

    let mut flashes = 0;
    for step in 0..1000 {
        let mut stepflashes = 0;
        println!("Step {}", step);
        print_board(&board, boardsize);

        let mut toflash: Vec<(usize, usize)> = Vec::new();
        let mut flashed = vec![vec![false; rowsize]; linecount];

        println!("  Increasing energy of all squares by one...");
        for row in 0..linecount as usize {
            for col in 0..rowsize as usize {
                board[row][col] += 1;
                if board[row as usize][col as usize] > 9 {
                    println!("  adding {},{} to toflash", col, row);
                    toflash.push((row, col));
                    //flashes += 1;
                }
            }
        }
        println!("  Processing flashes...");
        while !toflash.is_empty() {
            let p = toflash.pop().unwrap();
            if flashed[p.0][p.1] {
                continue;
            }
            println!("  flashing {},{}", p.1, p.0);
            stepflashes += 1;
            board[p.0][p.1] = 0;
            flashed[p.0][p.1] = true;
            for (col, row) in neighbor_iter((p.1 as i32, p.0 as i32), boardsize) {
                if !flashed[row as usize][col as usize] {
                    board[row as usize][col as usize] += 1;
                    if board[row as usize][col as usize] > 9 {
                        toflash.push((row as usize, col as usize));
                        //flashes += 1;
                    }
                }
            }
        }
        println!("Step flashes:{}", stepflashes);
        flashes += stepflashes;
        println!("Total flashes:{}", flashes);
        println!("");
        if stepflashes == boardsize.0 as i32 * boardsize.1 as i32 {
            println!("Everybody flashed! Step = {}", step + 1);
            break;
        }
    }
}
