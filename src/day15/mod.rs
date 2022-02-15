use crate::*;
//use regex::Regex;
//use std::collections::HashMap;
use std::cell::Cell;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::rc::Rc;

const FILENAME: &str = "src/day15/day15_input.txt";
//const FILENAME: &str = "src/day15/day15_example.txt";
//const FILENAME: &str = "src/day15/day15_simple.txt";

#[derive(Eq, Debug, Copy, Clone)]
struct Point(i32, i32);

struct NeighborIter {
    origin: Point,             // col,row
    boardsize: (usize, usize), // rows and columns
    count: usize,
}
const MAX_NC: usize = 4;

impl Iterator for NeighborIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let offsets: [(i32, i32); MAX_NC] = [
            //(-1, -1),
            (0, -1),
            //(1, -1),
            (-1, 0),
            /*(0,0),*/
            (1, 0),
            //(-1, 1),
            (0, 1),
            //(1, 1),
        ];

        while self.count < MAX_NC {
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

fn neighbor_iter(origin: Point, boardsize: (usize, usize)) -> NeighborIter {
    NeighborIter {
        origin: Point(origin.0, origin.1),
        boardsize: boardsize,
        count: 0,
    }
}

//type Point = (i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[derive(Eq, Debug)]
struct Node {
    p: Point,
    weight: i32,
    cost_so_far: Cell<i32>,
    came_from: Cell<Option<Point>>, // col, row
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.cost_so_far.get()).cmp(&Reverse(other.cost_so_far.get()))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p && self.weight == other.weight
    }
}

// TODO: Implement A* search rather than pure Dijkstra
// But it's still less than a a second of time even without it
#[allow(dead_code)]
fn heuristic(loc: Point, boardsize: (usize, usize)) -> i32 {
    // Just the manhattan distance to the bottom right corner.
    (boardsize.0 as i32 - loc.0) + (boardsize.1 as i32 - loc.1)
}

#[allow(dead_code)]
pub fn day15_p1() {
    println!("Day 15 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut boardsize: (usize, usize) = (0, 0);
        let mut board: Vec<Vec<Node>> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                let mut colcounter = 0;

                let row: Vec<Node> = ip
                    .trim()
                    .split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap())
                    .map(|x| {
                        let n = Node {
                            p: Point(colcounter, boardsize.1 as i32),
                            weight: x,
                            cost_so_far: Cell::new(i32::MAX),
                            came_from: Cell::new(None),
                        };
                        colcounter += 1;
                        n
                    })
                    .collect();
                boardsize.1 += 1;
                if boardsize.0 == 0 {
                    boardsize.0 = row.len()
                } else if boardsize.0 != row.len() {
                    panic!("non-square board???");
                }
                board.push(row);
            }
        }

        //let x = board[0][0]; //.borrow_mut();
        //x.borrow_mut().cost_so_far = 0;

        board[0][0].cost_so_far.set(0);

        let mut frontier = BinaryHeap::new();
        frontier.push(&board[0][0]);

        while !frontier.is_empty() {
            let x = frontier.pop().expect("Node");
            //println!("Node {:?}", x);
            for n in neighbor_iter(x.p, boardsize) {
                let np = &board[n.1 as usize][n.0 as usize];
                if x.cost_so_far.get() + np.weight < np.cost_so_far.get() {
                    let new_cost = x.cost_so_far.get() + np.weight;
                    np.cost_so_far.set(new_cost);
                    np.came_from.set(Some(x.p));
                    //println!("  np:{:?}", np);
                    frontier.push(&board[n.1 as usize][n.0 as usize]);
                }
                //println!("  {:?}", n);
            }
        }

        println!("Final corner node: {:?}", board[boardsize.0-1][boardsize.1-1]);

    //println!("board:{:?}", board);
    } else {
        panic!("couldn't open {}", FILENAME);
    }
}

#[allow(dead_code)]
pub fn day15_p2() {
    println!("Day 15 Puzzle 2");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut boardsize: (usize, usize) = (0, 0);
        let mut board: Vec<Vec<Node>> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                let mut colcounter = 0;

                let row: Vec<Node> = ip
                    .trim()
                    .split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse().unwrap())
                    .map(|x| {
                        let n = Node {
                            p: Point(colcounter, boardsize.1 as i32),
                            weight: x,
                            cost_so_far: Cell::new(i32::MAX),
                            came_from: Cell::new(None),
                        };
                        colcounter += 1;
                        n
                    })
                    .collect();
                boardsize.1 += 1;
                if boardsize.0 == 0 {
                    boardsize.0 = row.len()
                } else if boardsize.0 != row.len() {
                    panic!("non-square board???");
                }
                board.push(row);
            }
        }

        // OK, now multiply the board by 5
        let mut bigboard: Vec<Vec<Node>> = Vec::new();
        for rowmult in 0..5 {
            for row in 0..boardsize.0 {
                let mut new_row:Vec<Node> = Vec::new();
                for colmult in 0..5 {
                    for col in 0..boardsize.1 {
                        let rn = rowmult*boardsize.0 + row;
                        let cn = colmult*boardsize.1 + col;
                        let mut val = board[row][col].weight+ rowmult as i32+ colmult as i32;
                        if val > 9 {
                            val = val - 9;
                        }
                        new_row.push(Node {
                            p: Point(cn as i32,rn as i32),
                            weight: val,
                            cost_so_far: Cell::new(i32::MAX),
                            came_from: Cell::new(None),
                        });
                    }
                }
                bigboard.push(new_row);
            }
        }

        board = bigboard;
        boardsize.0 *= 5;
        boardsize.1 *= 5;

        //let x = board[0][0]; //.borrow_mut();
        //x.borrow_mut().cost_so_far = 0;

        board[0][0].cost_so_far.set(0);

        let mut frontier = BinaryHeap::new();
        frontier.push(&board[0][0]);

        while !frontier.is_empty() {
            let x = frontier.pop().expect("Node");
            //println!("Node {:?}", x);
            for n in neighbor_iter(x.p, boardsize) {
                let np = &board[n.1 as usize][n.0 as usize];
                if x.cost_so_far.get() + np.weight < np.cost_so_far.get() {
                    let new_cost = x.cost_so_far.get() + np.weight;
                    np.cost_so_far.set(new_cost);
                    np.came_from.set(Some(x.p));
                    //println!("  np:{:?}", np);
                    frontier.push(&board[n.1 as usize][n.0 as usize]);
                }
                //println!("  {:?}", n);
            }
        }

        println!("Final corner node: {:?}", board[boardsize.0-1][boardsize.1-1]);

    //println!("board:{:?}", board);
    } else {
        panic!("couldn't open {}", FILENAME);
    }
}
