use crate::*;
use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::mem;

//const FILENAME: &str = "src/day23/day23_example.txt";
//const FILENAME: &str = "src/day23/day23_input.txt";
//const FILENAME: &str = "src/day23/day23_example_big.txt";
const FILENAME: &str = "src/day23/day23_input_big.txt";

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
const MAX_ROOM_SIZE: usize = 4;
const NUM_ROOMS: usize = 4;
const MAX_BOARDSIZE: usize = HALLSIZE + MAX_ROOM_SIZE * NUM_ROOMS;

struct BoardIter {
    board: Board,
    srcpos: usize,
    tarpos: usize,
}

impl BoardIter {
    fn new(board: &Board) -> BoardIter {
        BoardIter {
            board: board.clone(),
            srcpos: 0,
            tarpos: 0,
        }
    }
}

/*
Roomsize = 2
#############
#01.2.3.4.56#
###7#8#9#A###
  #B#C#D#E#
  #########

Roomsize = 4
#############
#01.2.3.4.56#
###7#B#F#3###
  #8#C#0#4#  
  #9#D#1#5#
  #A#E#2#6#  
  #########
*/

impl Iterator for BoardIter {
    type Item = Board;
    fn next(&mut self) -> Option<Self::Item> {
        //println!("next: srcpos={}, n={}", self.srcpos, self.tarpos);
        let paths2: Vec<Vec<i32>> = vec![
            vec![0, 1, -1,  7,  8],
            vec![0, 1, -1,  2, -1,  9, 10],
            vec![0, 1, -1,  2, -1,  3, -1, 11, 12],
            vec![0, 1, -1,  2, -1,  3, -1,  4, -1, 13, 14],
            vec![6, 5, -1,  4, -1,  3, -1,  2, -1,  7,  8],
            vec![6, 5, -1,  4, -1,  3, -1,  9, 10],
            vec![6, 5, -1,  4, -1, 11, 12],
            vec![6, 5, -1, 13, 14],
        ];

        let paths4:  Vec<Vec<i32>> = vec![
            vec![0, 1, -1, 7,  8,  9, 10],
            vec![0, 1, -1, 2, -1, 11, 12, 13, 14 ],
            vec![0, 1, -1, 2, -1,  3, -1, 15, 16, 17, 18],
            vec![0, 1, -1, 2, -1,  3, -1,  4, -1, 19, 20, 21, 22],
            vec![6, 5, -1, 4, -1,  3, -1,  2, -1, 7, 8, 9, 10],
            vec![6, 5, -1, 4, -1,  3, -1, 11, 12, 13, 14],
            vec![6, 5, -1, 4, -1, 15, 16, 17, 18],
            vec![6, 5, -1, 19, 20,21, 22],
        ];

        let paths:Vec<Vec<i32>>;
        match self.board.roomsize {
            2 => paths = paths2,
            4 => paths = paths4,
            _ => panic!("Unknown roomsize"),
        }        

        let boardsize = HALLSIZE + NUM_ROOMS * self.board.roomsize;

        for src in self.srcpos..boardsize {
            if self.board.spaces[src] == BState::Empty {
                continue;
            }

            // If the piece is already in the right place, don't move it
            if self.board.piece_is_home(src) {
                continue;
            }

            'tarloop: for tar in self.tarpos..boardsize {
                if self.board.spaces[tar] != BState::Empty {
                    continue;
                }
                // both in hallway
                if src < 7 && tar < 7 {
                    continue;
                }
                // both in rooms
                if src >= 7 && tar >= 7 {
                    continue;
                }

                // Only allow hallway pieces to move into the correct room.
                if tar >= 7 {
                    if !self.board.piece_may_enter_room(src,tar) {
                        continue;
                    }
                }

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
                    BState::A => newboard.cost += 1 * moves as i32,
                    BState::B => newboard.cost += 10 * moves as i32,
                    BState::C => newboard.cost += 100 * moves as i32,
                    BState::D => newboard.cost += 1000 * moves as i32,
                    _ => panic!("Unknown state"),
                }
                //println!("newboard, cost={}", newboard.cost);
                self.srcpos = src;
                self.tarpos = tar + 1;
                if self.tarpos >= boardsize {
                    self.srcpos += 1;
                    self.tarpos = 0;
                    //println!("self.srcpos={}, self.tarpos={}", self.srcpos, self.tarpos);
                }
                //println!("newboard=\n{}", newboard);
                return Some(newboard);
            }
            self.tarpos = 0;
        }
        None
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Board {
    spaces: [BState; MAX_BOARDSIZE],
    cost: i32,
    roomsize: usize,
}

impl Board {
    fn new() -> Board {
        Board {
            spaces: [BState::Empty; MAX_BOARDSIZE],
            cost: 0,
            roomsize: 2,
        }
    }

    fn get_hallway(&self, pos: usize) -> BState {
        if pos >= HALLSIZE {
            panic!("Illegal pos {}", pos);
        }
        return self.spaces[pos];
    }

    fn get_room(&self, rtype: RoomType, pos: usize) -> BState {
        if pos >= self.roomsize {
            panic!("Illegal pos {}", pos);
        }

        match rtype {
            RoomType::A => self.spaces[HALLSIZE + self.roomsize*0 + pos],
            RoomType::B => self.spaces[HALLSIZE + self.roomsize*1 + pos],
            RoomType::C => self.spaces[HALLSIZE + self.roomsize*2 + pos],
            RoomType::D => self.spaces[HALLSIZE + self.roomsize*3 + pos],
        }
    }

    fn set_room(&mut self, rtype: RoomType, pos: usize, state: BState) {
        if pos >= self.roomsize {
            panic!("Illegal pos {}", pos);
        }

        match rtype {
            RoomType::A => self.spaces[HALLSIZE + self.roomsize*0 + pos] = state,
            RoomType::B => self.spaces[HALLSIZE + self.roomsize*1 + pos] = state,
            RoomType::C => self.spaces[HALLSIZE + self.roomsize*2 + pos] = state,
            RoomType::D => self.spaces[HALLSIZE + self.roomsize*3 + pos] = state,
        }
    }

    fn piece_may_enter_room(&self, src:usize, tar:usize) -> bool {
        // piece is already in room
        if src >= HALLSIZE {
            return false;
        }

        // target is not a room
        if tar < HALLSIZE {
            panic!("Only supposed to call if target is known to be a room");
        }

        let p = tar - HALLSIZE;
        let offset = p % self.roomsize;
        
        let room_type = 
            match p / self.roomsize {
                0 => BState::A,
                1 => BState::B,
                2 => BState::C,
                3 => BState::D,
                _ => panic!("unknown state")
        };
        //println!("src={} tar={} p={} offset={} room_type={}", src,tar,p,offset, room_type);
        //println!("{}", self);
        // Is the piece targeting the right room?
        if self.spaces[src] != room_type {
            //println!("target not correct");
            return false;
        }

        let room_start = (p/self.roomsize) * self.roomsize + HALLSIZE;

        // Make sure walkway and target space is empty
        for i in 0..(offset) {
            if self.spaces[i+room_start] != BState::Empty {
                //println!("walkway not empty: {}", i+p+HALLSIZE);
                return false;
            }
        }
        // Make sure rest of room is settled
        for i in offset+1..self.roomsize {
            if i+room_start > 23 {
                println!("src:{} tar:{} i:{} p:{} HALLSIZE:{} offset:{} self.roomsize:{}",
                    src,tar,i,p,HALLSIZE,offset,self.roomsize);
            }
            if self.spaces[i+room_start] != room_type {
                //println!("room not settled: {}", i+p+HALLSIZE);
                return false;
            }
        }

        //println!("coolio");
        return true;
    }

    fn piece_is_home(&self, src: usize) -> bool {
        if src < HALLSIZE {
            return false;
        }
        let p = src - HALLSIZE;
        let offset = p % self.roomsize;
        let room_type = 
            match p / self.roomsize {
                0 => BState::A,
                1 => BState::B,
                2 => BState::C,
                3 => BState::D,
                _ => panic!("unknown state")
        };

        // Is the piece in the right room?
        if self.spaces[src] != room_type {
            return false;
        }
        //println!("srcpos={} p={} offset={}", srcpos, p, offset);
  
        let room_start = (p/self.roomsize) * self.roomsize + HALLSIZE;

        // Are the other deeper pieces also in the right room?
        for i in (offset+1) .. self.roomsize {
            //println!("i={}, target_state={}", i, target_state);
            if self.spaces[i+room_start] != room_type {
                //println!("False");
                return false;
            }
        }
        //println!("True");
        return true;
    }

    fn is_done(&self) -> bool {
        for i in 0..HALLSIZE {
            if self.spaces[i] != BState::Empty {
                return false;
            }
        }

        for i in 0 .. self.roomsize * NUM_ROOMS {
            let room_type = 
                match i / self.roomsize {
                    0 => BState::A,
                    1 => BState::B,
                    2 => BState::C,
                    3 => BState::D,
                    _ => panic!("unknown state")
            };
            if self.spaces[HALLSIZE+i] != room_type {
                return false;
            }
        }

        println!("{}",self);
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
        for i in 1..self.roomsize {
            writeln!(
                f,
                "  #{}#{}#{}#{}#",
                self.get_room(RoomType::A, i),
                self.get_room(RoomType::B, i),
                self.get_room(RoomType::C, i),
                self.get_room(RoomType::D, i)
            )?;
        }
        writeln!(f, "  #########")?;
        writeln!(f, "cost: {}", self.cost)
    }
}

fn solution_search(orig_board: &Board) -> i64 {
    let mut best = i32::MAX;
    let mut todo: VecDeque<Board> = VecDeque::new();
    let mut cache: HashSet<Board> = HashSet::new();

    todo.push_back(orig_board.clone());
    cache.insert(orig_board.clone());

    while !todo.is_empty() {
        println!("Evaluating next board on todo list, len={}", todo.len());
        let b = todo.pop_front().unwrap();

        for nb in BoardIter::new(&b) {
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

            if cache.contains(&nb) {
                continue;
            }

            cache.insert(nb.clone());
            todo.push_back(nb);
        }
    }
    println!("Cache size = {}", cache.len());
    println!("Best = {}", best);

    0
}

#[allow(dead_code)]
pub fn day23_p1() {
    println!("Day 23 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {

        let re_line0 = Regex::new(r"^#############$").unwrap();
        let re_line1 = Regex::new(r"^#\.\.\.\.\.\.\.\.\.\.\.#$").unwrap();
        let re_line2 = Regex::new(r"^###([A-D])#([A-D])#([A-D])#([A-D])###$").unwrap();
        let re_linex = Regex::new(r"^  #([A-D])#([A-D])#([A-D])#([A-D])#$").unwrap();
        let re_line4 = Regex::new(r"^  #########$").unwrap();

        let mut board = Board::new();
        board.roomsize = 4;
        let mut roomsize = 0;
        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_line0.captures(&ip) {
                    // nothing
                } else if let Some(cap) = re_line1.captures(&ip) {
                    // nothing
                } else if let Some(cap) = re_line2.captures(&ip) {
                    board.set_room(RoomType::A, 0, BState::new(&cap[1]));
                    board.set_room(RoomType::B, 0, BState::new(&cap[2]));
                    board.set_room(RoomType::C, 0, BState::new(&cap[3]));
                    board.set_room(RoomType::D, 0, BState::new(&cap[4]));
                    roomsize += 1;
                } else if let Some(cap) = re_linex.captures(&ip) {
                    board.set_room(RoomType::A, roomsize, BState::new(&cap[1]));
                    board.set_room(RoomType::B, roomsize, BState::new(&cap[2]));
                    board.set_room(RoomType::C, roomsize, BState::new(&cap[3]));
                    board.set_room(RoomType::D, roomsize, BState::new(&cap[4]));
                    roomsize += 1;
                } else if let Some(cap) = re_line4.captures(&ip) {
                    // nothing
                } else {
                    println!("Unexpected line '{}'", ip);
                }
            }
        }
        board.roomsize = roomsize;
        println!("{}", board);
        solution_search(&board);
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
