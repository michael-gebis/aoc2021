use crate::*;
use regex::Regex;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::mem;

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

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum BState {
    Empty,
    A,
    B,
    C,
    D,
}

#[derive(Copy, Clone)]
enum RoomType {
    A,
    B,
    C,
    D,
}

impl fmt::Display for BState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
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
    pub fn new(x: &str) -> BState {
        match x {
            "A" => BState::A,
            "B" => BState::B,
            "C" => BState::C,
            "D" => BState::D,
            _ => BState::Empty,
        }
    }
}

const HALLSIZE: usize = 7;
const BOARDSIZE: usize = HALLSIZE + 2 * 4;

struct BoardIter {
    board: Board,
    pos: usize,
    n: usize,
}

impl BoardIter {
    fn new(board: &Board) -> BoardIter {
        BoardIter {
            board: board.clone(),
            pos: 0,
            n: 0,
        }
    }
}

/*
#############
#01.2.3.4.56#
###7#8#9#A###
  #B#C#D#E#
  #########
*/

impl Iterator for BoardIter {
    type Item = Board;
    fn next(&mut self) -> Option<Self::Item> {
        //println!("next: pos={}, n={}", self.pos, self.n);
        let paths: Vec<Vec<i32>> = vec![
            vec![0, 1, -1, 7, 8],
            vec![0, 1, -1, 2, -1, 9, 10],
            vec![0, 1, -1, 2, -1, 3, -1, 11, 12],
            vec![0, 1, -1, 2, -1, 3, -1, 4, -1, 13, 14],
            vec![6, 5, -1, 4, -1, 3, -1, 2, -1, 7, 8],
            vec![6, 5, -1, 4, -1, 3, -1, 9, 10],
            vec![6, 5, -1, 4, -1, 11, 12],
            vec![6, 5, -1, 13, 14],
        ];

        for src in self.pos..BOARDSIZE {
            if self.board.spaces[src] == BState::Empty {
                continue;
            }

            // If the piece is already in the right place, don't move it
            match self.board.spaces[src] {
                BState::A => {
                    if src == 8 {
                        continue;
                    }
                    if src == 7 && self.board.spaces[8] == BState::A {
                        continue;
                    }
                }
                BState::B => {
                    if src == 10 {
                        continue;
                    }
                    if src == 9 && self.board.spaces[10] == BState::B {
                        continue;
                    }
                }
                BState::C => {
                    if src == 12 {
                        continue;
                    }
                    if src == 11 && self.board.spaces[12] == BState::C {
                        continue;
                    }
                }
                BState::D => {
                    if src == 14 {
                        continue;
                    }
                    if src == 13 && self.board.spaces[14] == BState::D {
                        continue;
                    }
                }
                _ => {
                    continue;
                }
            }

            'tarloop: for tar in self.n..BOARDSIZE {
                //self.n = tar;

                if self.board.spaces[tar] != BState::Empty {
                    continue;
                }
                //if tar == src {
                //println!("tar==src");
                //    continue;
                //}
                // both in hallway
                if src < 7 && tar < 7 {
                    //println!("both hall");
                    continue;
                }
                // both in rooms
                if src >= 7 && tar >= 7 {
                    //println!("both room");
                    continue;
                }

                //println!("src={} tar={}", src,tar);
                // Only allow hallway pieces to move into the correct room.
                if tar >= 7 {
                    match self.board.spaces[src] {
                        BState::A => {
                            if tar != 7 && tar != 8 {
                                continue;
                            }
                            if tar == 7 && self.board.spaces[8] != self.board.spaces[src] {
                                continue;
                            }
                        }
                        BState::B => {
                            if tar != 9 && tar != 10 {
                                continue;
                            }
                            if tar == 9 && self.board.spaces[10] != self.board.spaces[src] {
                                continue;
                            }
                        }
                        BState::C => {
                            if tar != 11 && tar != 12 {
                                continue;
                            }
                            if tar == 11 && self.board.spaces[12] != self.board.spaces[src] {
                                continue;
                            }
                        }
                        BState::D => {
                            if tar != 13 && tar != 14 {
                                continue;
                            }
                            if tar == 13 && self.board.spaces[14] != self.board.spaces[src] {
                                continue;
                            }
                        }
                        _ => panic!("Impossible state"),
                    }
                }
                //let moves:usize = 0;

                //println!("attempting move from {} to {}", src, tar);

                let mut a: usize = 0;
                let mut b: usize = 0;
                for p in paths.iter() {
                    if p.contains(&(src as i32)) && p.contains(&(tar as i32)) {
                        a = p.iter().position(|x| (*x) as usize == src).unwrap();
                        b = p.iter().position(|x| (*x) as usize == tar).unwrap();
                        if a > b {
                            mem::swap(&mut a, &mut b);
                        }
                        //println!("a={}, b={}, p={:?}", a,b,p);
                        for i in (a + 1)..b {
                            if p[i] == -1 {
                                continue;
                            }
                            if self.board.spaces[p[i] as usize] != BState::Empty {
                                //println!("space is not empty: i={}, p[i]={}, {}", i, p[i], self.board.spaces[p[i] as usize]);
                                continue 'tarloop;
                            }
                        }
                        break;
                    }
                }
                let moves = b - a;

                let mut newboard = self.board.clone();
                newboard.spaces[src] = BState::Empty;
                newboard.spaces[tar] = self.board.spaces[src];

                match self.board.spaces[src] {
                    BState::A => newboard.cost += 1 * moves as i64,
                    BState::B => newboard.cost += 10 * moves as i64,
                    BState::C => newboard.cost += 100 * moves as i64,
                    BState::D => newboard.cost += 1000 * moves as i64,
                    _ => panic!("Unknown state"),
                }
                //println!("newboard, cost={}", newboard.cost);
                self.pos = src;
                self.n = tar + 1;
                if self.n >= BOARDSIZE {
                    self.pos += 1;
                    self.n = 0;
                    //println!("self.pos={}, self.n={}", self.pos, self.n);
                }
                //println!("newboard=\n{}", newboard);
                return Some(newboard);

                /*
                match board.spaces[i] {
                    BState::A => {
                        // Piece in hallway must target Room A
                        if src < 7 && tar != 7 && tar != 11 {
                            next;
                        }
                        // Piece in any room must target hallway.
                        if src >= 7 && tar >= 7 {
                            next;
                        }

                        // Hallway -> Room
                        if src < 7 {
                            if board.spaces[7] != BState::Empty {
                                next;
                            }
                            if tar == 7 && board.spaces[11] != BState::A {
                                next;
                            }
                            if src <= 1 {
                                for k in src..1 {
                                    if board.spaces[k] != BState::Empty {
                                        next;
                                    }
                                    moves += 1;
                                }
                                moves +=1;
                            } else {
                                for k in (2..src+1).rev() {
                                    if board.spaces[k] != BState::Empty {
                                        next;
                                    }
                                    moves += 1;
                                }
                                moves +=1;
                            }
                            if tar == 7 { moves +=1;} else { moves +=2;}
                        } else {
                            if src == 7 {}
                        }

                    },
                    BState::B => {
                        if src < 7 && tar != 8 && tar != 12 {
                            next;
                        }
                    }
                    BState::C => {
                        if src < 7 && tar != 9 && tar != 13 {
                            next;
                        }
                    }
                    BState::D => {
                        if src < 7 && tar != 10 && tar != 14 {
                            next;
                        }
                    }
                }
                */
            }
            self.n = 0;
        }
        None
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Board {
    spaces: [BState; BOARDSIZE],
    cost: i64,
}

impl Board {
    fn new() -> Board {
        Board {
            spaces: [BState::Empty; BOARDSIZE],
            cost: 0,
        }
    }

    fn get_hallway(&self, pos: usize) -> BState {
        if pos >= HALLSIZE {
            panic!("Illegal pos {}", pos);
        }
        return self.spaces[pos];
    }

    fn get_room(&self, rtype: RoomType, pos: usize) -> BState {
        if pos >= 2 {
            panic!("Illegal pos {}", pos);
        }

        match rtype {
            RoomType::A => self.spaces[HALLSIZE + pos],
            RoomType::B => self.spaces[HALLSIZE + 2 + pos],
            RoomType::C => self.spaces[HALLSIZE + 4 + pos],
            RoomType::D => self.spaces[HALLSIZE + 6 + pos],
        }
    }

    fn set_room(&mut self, rtype: RoomType, pos: usize, state: BState) {
        if pos >= 2 {
            panic!("Illegal pos {}", pos);
        }

        match rtype {
            RoomType::A => self.spaces[HALLSIZE + pos] = state,
            RoomType::B => self.spaces[HALLSIZE + 2 + pos] = state,
            RoomType::C => self.spaces[HALLSIZE + 4 + pos] = state,
            RoomType::D => self.spaces[HALLSIZE + 6 + pos] = state,
        }
    }

    fn is_done(&self) -> bool {
        for i in 0..HALLSIZE {
            if self.spaces[i] != BState::Empty {
                return false;
            }
        }
        if self.get_room(RoomType::A, 0) != BState::A
            || self.get_room(RoomType::A, 1) != BState::A
            || self.get_room(RoomType::B, 0) != BState::B
            || self.get_room(RoomType::B, 1) != BState::B
            || self.get_room(RoomType::C, 0) != BState::C
            || self.get_room(RoomType::C, 1) != BState::C
            || self.get_room(RoomType::D, 0) != BState::D
            || self.get_room(RoomType::D, 1) != BState::D
        {
            return false;
        }
        true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            self.get_hallway(0),
            self.get_hallway(1),
            self.get_hallway(2),
            self.get_hallway(3),
            self.get_hallway(4),
            self.get_hallway(5),
            self.get_hallway(6)
        )?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            self.get_room(RoomType::A, 0),
            self.get_room(RoomType::B, 0),
            self.get_room(RoomType::C, 0),
            self.get_room(RoomType::D, 0)
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#",
            self.get_room(RoomType::A, 1),
            self.get_room(RoomType::B, 1),
            self.get_room(RoomType::C, 1),
            self.get_room(RoomType::D, 1)
        )?;
        writeln!(f, "  #########")?;
        writeln!(f, "cost: {}", self.cost)
    }
}

fn solution_search(orig_board: &Board) -> i64 {
    //let board = orig_board.clone();
    //println!("HeyHey2");
    let mut best = i64::MAX;
    let mut count: i64 = 0;

    let mut todo: VecDeque<Board> = VecDeque::new();
    let mut cache: HashSet<Board> = HashSet::new();

    todo.push_back(orig_board.clone());
    cache.insert(orig_board.clone());

    while !todo.is_empty() {
        println!("Evaluating next board on todo list, len={}", todo.len());
        let b = todo.pop_front().unwrap();
        //println!("{}", b);
        //println!("");

        let mut nb_count = 0;
        for nb in BoardIter::new(&b) {
            nb_count += 1;
            if nb.cost > best {
                continue;
            }


            if nb.is_done() {
                println!("Solution found!  Cost = {}", nb.cost);
                if nb.cost < best {
                    best = nb.cost;
                }
                continue;
            }
            let mut nb2 = nb.clone();
            //nb2.cost = 0;

            if cache.contains(&nb2) {
                //println!("Cache hit!!!");
                continue;
            }

            cache.insert(nb2);
            todo.push_back(nb);
        }
        //println!("nb_count={}", nb_count);
    }
    println!("Best = {}", best);

    0
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

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_line0.captures(&ip) {
                    // nothing
                } else if let Some(cap) = re_line1.captures(&ip) {
                    // nothing
                } else if let Some(cap) = re_line2.captures(&ip) {
                    //println!("line2");
                    board.set_room(RoomType::A, 0, BState::new(&cap[1]));
                    board.set_room(RoomType::B, 0, BState::new(&cap[2]));
                    board.set_room(RoomType::C, 0, BState::new(&cap[3]));
                    board.set_room(RoomType::D, 0, BState::new(&cap[4]));
                } else if let Some(cap) = re_line3.captures(&ip) {
                    //println!("line3");
                    board.set_room(RoomType::A, 1, BState::new(&cap[1]));
                    board.set_room(RoomType::B, 1, BState::new(&cap[2]));
                    board.set_room(RoomType::C, 1, BState::new(&cap[3]));
                    board.set_room(RoomType::D, 1, BState::new(&cap[4]));
                } else if let Some(cap) = re_line4.captures(&ip) {
                    // nothing
                } else {
                    println!("Unexpected line '{}'", ip);
                }
            }
        }
        //println!("{}", board);
        solution_search(&board);
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
