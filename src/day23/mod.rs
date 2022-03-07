use crate::*;
use regex::Regex;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

const FILENAME: &str = "src/day23/day23_example.txt";
//const FILENAME: &str = "src/day23/day23_input.txt";

// General strategery:
// Create a list of "boards yet to be evaluated"
// For the first board, look at each piece
// For each piece, generate legal moves
// Put that new board on the list of "boards yet to be evaluate"
// Also: Keep a cache of already evaluated boards
// Also: Keep a "best score so far" number; don't evaluate boards that are greater than that number

/* 
Empty board
#############
#...........#
###.#.#.#.###
  #.#.#.#.#
  #########

Note that pieces cannot stop in several positions, as shown by the X:
#############
#..X.X.X.X..#
###.#.#.#.###
  #.#.#.#.#
  #########

*/

#[derive(Copy,Clone)]
enum BState {
    Empty,
    A,
    B,
    C,
    D,
}

impl fmt::Display for BState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 
            match self {
                BState::Empty => ".",
                BState::A => "A",
                BState::B => "B",
                BState::C => "C",
                BState::D => "D",
            }
        )
    }
}

impl BState {
    pub fn new(x:&str) -> BState {
        match x {
            "A" => BState::A,
            "B" => BState::B,
            "C" => BState::C,
            "D" => BState::D,
            _ => BState::Empty,
        }
    }
}


const HALLSIZE: usize = 11;

struct Board {
    hallway: [BState; HALLSIZE],
    rooma: [BState; 2],
    roomb: [BState; 2],
    roomc: [BState; 2],
    roomd: [BState; 2],
    cost: i64,
}

impl Board {
    pub fn new() -> Board {
        Board { 
            hallway:[BState::Empty;11],
            rooma:  [BState::Empty;2],
            roomb:  [BState::Empty;2],
            roomc:  [BState::Empty;2],
            roomd:  [BState::Empty;2],
            cost:   0,
        }
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "#############").unwrap();
        write!  (f, "#").unwrap();
        for i in 0..HALLSIZE {
            write!(f,"{}", self.hallway[i]).unwrap();
        }
        writeln!(f, "#").unwrap();
        writeln!(f, "###{}#{}#{}#{}###", self.rooma[1], self.roomb[1], self.roomc[1], self.roomd[1]).unwrap();
        writeln!(f, "  #{}#{}#{}#{}#", self.rooma[0], self.roomb[0], self.roomc[0], self.roomd[0]).unwrap();
        writeln!(f, "  #########")
    }
}
#[allow(dead_code)]
pub fn day23_p1() {
    println!("Day 23 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        //let re_cube =
        //    Regex::new(r"(o.+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
        //        .unwrap();
        //let mut pending_cubes: VecDeque<Cube> = VecDeque::new();

        let re_line0 = Regex::new(r"^#############$").unwrap();
        let re_line1 = Regex::new(r"^#\.\.\.\.\.\.\.\.\.\.\.#$").unwrap();
        let re_line2 = Regex::new(r"^###([A-D])#([A-D])#([A-D])#([A-D])###$").unwrap();
        let re_line3 = Regex::new(r"^  #([A-D])#([A-D])#([A-D])#([A-D])#$").unwrap();
        let re_line4 = Regex::new(r"^  #########$").unwrap();

        let mut board = Board::new();
//                         cap[2].parse::<i64>().unwrap(),

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_line0.captures(&ip) {
                    // nothing
                } else if let Some(cap) = re_line1.captures(&ip) {
                    // nothing
                } else if let Some(cap) = re_line2.captures(&ip) {
                    //println!("line2");
                     board.rooma[1] = BState::new(&cap[1]);
                     board.roomb[1] = BState::new(&cap[2]);
                     board.roomc[1] = BState::new(&cap[3]);
                     board.roomd[1] = BState::new(&cap[4]);
                } else if let Some(cap) = re_line3.captures(&ip) {                    
                    //println!("line3");
                    board.rooma[0] = BState::new(&cap[1]);
                    board.roomb[0] = BState::new(&cap[2]);
                    board.roomc[0] = BState::new(&cap[3]);
                    board.roomd[0] = BState::new(&cap[4]);
                } else if let Some(cap) = re_line4.captures(&ip) {
                    // nothing
                } else {
                    println!("Unexpected line '{}'", ip);
                }
            }
        }
        println!("{}", board);
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
