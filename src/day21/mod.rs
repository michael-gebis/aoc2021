use crate::*;
use regex::Regex;
use std::collections::HashMap;

const FILENAME: &str = "src/day21/day21_input.txt";
//const FILENAME: &str = "src/day21/day21_example.txt";

const DIE_SIDES: i32 = 100;
const BOARD_SIZE: i32 = 10;
const WINNING_SCORE: i32 = 1000;

struct Die {
    state: i32,
    rolls: i32,
}

impl Die {
    fn new() -> Die {
        Die { state: 0, rolls: 0 }
    }

    fn roll(&mut self) -> i32 {
        let ret = self.state + 1;
        self.rolls += 1;
        self.state = self.state + 1;
        self.state = self.state % DIE_SIDES;
        // println!("  roll {}", ret);
        ret
    }
}

#[allow(dead_code)]
pub fn day21_p1() {
    println!("Day 21 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let re_player1 = Regex::new(r"^Player 1 starting position: (\d+)$").unwrap();
        let re_player2 = Regex::new(r"^Player 2 starting position: (\d+)$").unwrap();

        let mut p1_pos: i32 = 0;
        let mut p2_pos: i32 = 0;

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_player1.captures(&ip) {
                    p1_pos = cap[1].parse().unwrap();
                    p1_pos -= 1;
                    println!("Got p1 {}", p1_pos + 1);
                } else if let Some(cap) = re_player2.captures(&ip) {
                    p2_pos = cap[1].parse().unwrap();
                    p2_pos -= 1;
                    println!("Got p2 {}", p2_pos + 1);
                } else {
                    println!("ignoring '{}'", ip);
                }
            } else {
                panic!("Unexpected result");
            }
        }

        println!("Start: p1_pos={} p2_pos={}", p1_pos + 1, p2_pos + 1);
        let mut p1_score: i32 = 0;
        let mut p2_score: i32 = 0;

        let mut die = Die::new();

        loop {
            //println!("p1 score={} p1_pos={}", p1_score, p1_pos+1);
            p1_pos += die.roll() + die.roll() + die.roll();
            p1_pos %= BOARD_SIZE;
            p1_score += p1_pos + 1;

            if p1_score >= WINNING_SCORE {
                println!("P1 WINS!!!");
                println!(
                    "P1 score:{} P2 score:{} rolls:{}",
                    p1_score, p2_score, die.rolls
                );
                println!("P2 score * rolls = {}", p2_score * die.rolls);
                break;
            }

            //println!("p2 score={} p2_pos={}", p2_score, p2_pos+1);
            p2_pos += die.roll() + die.roll() + die.roll();
            p2_pos %= BOARD_SIZE;
            p2_score += p2_pos + 1;

            if p2_score >= WINNING_SCORE {
                println!("P2 WINS!!!");
                println!(
                    "P1 score:{} P2 score:{} rolls:{}",
                    p1_score, p2_score, die.rolls
                );
                println!("P1 score * rolls = {}", p1_score * die.rolls);
                break;
            }
        }
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}

// [111] = +3       [211] = +4      [311] = +5
// [112] = +4       [212] = +5      [312] = +6
// [113] = +5       [213] = +6      [313] = +7
// [121] = +4       [221] = +5      [321] = +6
// [122] = +5       [222] = +6      [322] = +7
// [123] = +6       [223] = +7      [323] = +8
// [131] = +5       [231] = +6      [331] = +7
// [132] = +6       [232] = +7      [332] = +8
// [133] = +7       [233] = +8      [333] = +9
// +3:1 +4:3 +5:6 +6:7 +7:6 +8:3 +9:1
// Also see https://en.wikipedia.org/wiki/Trinomial_triangle
// In the example, the starting points are 4 and 8.
// The state is represented by (score1,pos1,score2,pos2)
// Start with
//  (0,4,0,8):1
// Next is:
//  (7,7,0,8):1
//  (8,8,0,8):3
//  (9,9,0,8):6
//  (10,10,0,8):7
//  (1,1,0,8):6
//  (2,2,0,8):3
//  (3,3,0,8):1

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Qstate {
    p1_score: i8,
    p1_pos: i8,
    p2_score: i8,
    p2_pos: i8,
}

const DIST: [i64; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

const QSCORE_WIN: i8 = 21;

#[allow(dead_code)]
pub fn day21_p2() {
    println!("Day 21 Puzzle 2");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let re_player1 = Regex::new(r"^Player 1 starting position: (\d+)$").unwrap();
        let re_player2 = Regex::new(r"^Player 2 starting position: (\d+)$").unwrap();

        let mut p1_pos: i8 = 0;
        let mut p2_pos: i8 = 0;

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_player1.captures(&ip) {
                    p1_pos = cap[1].parse().unwrap();
                    p1_pos -= 1;
                    println!("Got p1 {}", p1_pos + 1);
                } else if let Some(cap) = re_player2.captures(&ip) {
                    p2_pos = cap[1].parse().unwrap();
                    p2_pos -= 1;
                    println!("Got p2 {}", p2_pos + 1);
                } else {
                    println!("ignoring '{}'", ip);
                }
            } else {
                panic!("Unexpected result");
            }
        }

        println!("Start: p1_pos={} p2_pos={}", p1_pos + 1, p2_pos + 1);

        let mut states: HashMap<Qstate, i64> = HashMap::new();
        states.insert(
            Qstate {
                p1_score: 0,
                p1_pos,
                p2_score: 0,
                p2_pos,
            },
            1,
        );

        let mut p1_wins: i64 = 0;
        let mut p2_wins: i64 = 0;

        while !states.is_empty() {
            //println!("p1 goes");
            let mut newstates: HashMap<Qstate, i64> = HashMap::new();

            for (qstate, count) in states {
                //println!("in {} universes the qstate is {:?}", count, qstate);
                for a in 3..10 {
                    let mut mystate = qstate.clone();
                    mystate.p1_pos = (mystate.p1_pos + a) % 10;
                    mystate.p1_score = mystate.p1_score + mystate.p1_pos + 1;

                    if mystate.p1_score >= QSCORE_WIN {
                        p1_wins += count * DIST[a as usize] as i64;
                    } else {
                        let v = count * DIST[a as usize] as i64;
                        let n = newstates.entry(mystate).or_insert(0);
                        *n += v;
                    }
                }
            }
            states = newstates;

            //println!("p2 goes:");
            let mut newstates: HashMap<Qstate, i64> = HashMap::new();
            for (qstate, count) in states {
                //println!("in {} universes the qstate is {:?}", count, qstate);
                for a in 3..10 {
                    let mut mystate = qstate.clone();
                    mystate.p2_pos = (mystate.p2_pos + a) % 10;
                    mystate.p2_score = mystate.p2_score + mystate.p2_pos + 1;

                    if mystate.p2_score >= QSCORE_WIN {
                        p2_wins += count * DIST[a as usize];
                    } else {
                        let v = count * DIST[a as usize] as i64;
                        let n = newstates.entry(mystate).or_insert(0);
                        *n += v;
                    }
                }
            }
            states = newstates;
        }

        println!("p1_wins:{} p2:wins:{}", p1_wins, p2_wins);
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
