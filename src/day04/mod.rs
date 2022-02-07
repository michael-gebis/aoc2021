extern crate termion;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use termion::{color, style};

const GRIDSIZE: usize = 5;

#[derive(Debug)]
enum CellState {
    Unpicked,
    Picked,
}

#[derive(Debug)]
struct Cell {
    val: u32,
    state: CellState,
}

impl Cell {
    fn new(v: u32) -> Cell {
        Cell {
            val: v,
            state: CellState::Unpicked,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.state {
            CellState::Picked => {
                write!(f, "{}{:3}{}", color::Fg(color::Red), self.val, style::Reset)
            }
            _ => write!(f, "{:3}", self.val),
        }
    }
}

struct Board {
    data: Vec<Vec<Cell>>,
}

impl Board {
    fn new() -> Board {
        //        data : Vec::new();
        let mut v = Vec::with_capacity(GRIDSIZE);
        println!("New");
        let mut c = 0;
        for _i in 0..GRIDSIZE {
            let mut t: Vec<Cell> = Vec::with_capacity(GRIDSIZE);
            for _j in 0..GRIDSIZE {
                t.push(Cell::new(c));
                c += 1;
            }
            v.push(t);
        }
        Board { data: v }
    }

    /*
    fn set_picked(self:&mut Board, i:usize, j:usize) {
        self.data[i][j].state = CellState::Picked;
    }
    */

    fn set_cell(self: &mut Board, i: usize, j: usize, val: u32) {
        self.data[i][j].val = val;
        self.data[i][j].state = CellState::Unpicked;
    }

    fn pick(self: &mut Board, v: u32) {
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                if self.data[i][j].val == v {
                    self.data[i][j].state = CellState::Picked;
                }
            }
        }
    }

    fn check_win(self: &Board) -> bool {
        // Check rows
        'rowloop: for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                //println!("pre i,j: {},{}", i,j);
                match self.data[i][j].state {
                    CellState::Unpicked => continue 'rowloop,
                    _ => (),
                }
                //println!("pos i,j: {},{}", i,j);
            }
            //println!("Win in row {}", i);
            return true;
        }
        // check cols
        'colloop: for j in 0..GRIDSIZE {
            for i in 0..GRIDSIZE {
                match self.data[i][j].state {
                    CellState::Unpicked => continue 'colloop,
                    _ => (),
                };
            }
            println!("Win in col {}", j);
            return true;
        }
        println!("Loser");
        false
    }

    fn sum_of_unpicked(&self) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                match self.data[i][j].state {
                    CellState::Unpicked => sum += self.data[i][j].val,
                    _ => (),
                };
            }
        }
        sum
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                write!(f, "{} ", self.data[i][j])?;
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}

fn read_board_from_file(filename: &String) -> (Vec<u32>, Vec<Board>) {
    println!("Reading from file {}", filename);

    let picklist: Vec<u32>; //= Vec::new();
    let mut boards: Vec<Board> = Vec::new();

    let file = File::open(filename).expect("Couldn't open file");
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    let _line = reader.read_line(&mut buffer).expect("Couldn't get line");

    // println!("line:'{}' buffer:'{}'", line, buffer.trim());

    picklist = buffer
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    println!("picklist:{:?}", picklist);

    // Skip a line
    //buffer.clear();
    //let _ = reader.read_line(&mut buffer);

    // Reads a blank line...
    buffer.clear();
    while let Ok(line) = reader.read_line(&mut buffer) {
        if line == 0 {
            break;
        }
        println!("Reading board!: buffer='{}'", buffer);

        let mut b = Board::new();

        for i in 0..GRIDSIZE {
            buffer.clear();
            let _myline = reader.read_line(&mut buffer).unwrap();
            println!("Reading board!!!: buffer='{}'", buffer);
            let vals: Vec<u32> = buffer
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for (j, v) in vals.iter().enumerate() {
                b.set_cell(i, j, *v);
            }
            //            buffer.clear();
        }
        boards.push(b);
        buffer.clear();
    }
    (picklist, boards)
}

#[allow(dead_code)]
pub fn day04_p1() {
    let mut b = Board::new();

    println!("{}", b);
    b.pick(5);
    b.pick(6);
    b.pick(7);
    b.pick(8);
    println!("{}", b);
    b.check_win();

    b.pick(9);
    println!("{}", b);
    b.check_win();
    println!("Score={}", b.sum_of_unpicked());

    let mut b = Board::new();
    b.pick(2);
    b.pick(7);
    b.pick(12);
    b.pick(17);
    println!("{}", b);
    b.check_win();

    b.pick(22);
    println!("{}", b);
    b.check_win();
    println!("Score={}", b.sum_of_unpicked());
    let (picks, mut boards) = read_board_from_file(&String::from("src/day04/day4_input.txt"));

    for pick in picks.iter() {
        println!("Picking ball {} from {} boards", pick, boards.len());
        for (_, b) in boards.iter_mut().enumerate() {
            b.pick(*pick);
            if b.check_win() {
                println!(
                    "Board {} wins!!!, sum={}, score={}",
                    b,
                    b.sum_of_unpicked(),
                    pick * b.sum_of_unpicked()
                );
                //panic!("winner");
                //break;
            }
        }
        while let Some(pos) = boards.iter().position(|x| x.check_win()) {
            println!("Removing winning board");
            boards.remove(pos);
        }
        //boards.remove(boards.iter().position(|x| x.check_win()).expect("hey"));
    }
}
