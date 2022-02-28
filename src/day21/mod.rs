use crate::*;
use regex::Regex;

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
        Die {state:0, rolls:0}
    }

    fn roll(&mut self) -> i32 {
        let ret = self.state+1;
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

        let mut p1_pos:i32 = 0;
        let mut p2_pos:i32 = 0;

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_player1.captures(&ip) {
                    p1_pos = cap[1].parse().unwrap();
                    p1_pos -= 1;
                    println!("Got p1 {}", p1_pos+1);
                } else if let Some(cap) = re_player2.captures(&ip) {
                    p2_pos = cap[1].parse().unwrap();
                    p2_pos -= 1;
                    println!("Got p2 {}", p2_pos+1);
                } else {
                    println!("ignoring '{}'", ip);
                }
            } else {
                panic!("Unexpected result");
            }
        }

        println!("Start: p1_pos={} p2_pos={}", p1_pos+1,p2_pos+1);
        let mut p1_score:i32 = 0;
        let mut p2_score:i32 = 0;

        let mut die = Die::new();

        loop {
            //println!("p1 score={} p1_pos={}", p1_score, p1_pos+1);
            p1_pos += die.roll() + die.roll() + die.roll();
            p1_pos %= BOARD_SIZE;
            p1_score += p1_pos+1;

            if p1_score >= WINNING_SCORE {
                println!("P1 WINS!!!");
                println!("P1 score:{} P2 score:{} rolls:{}", p1_score, p2_score, die.rolls);
                println!("P2 score * rolls = {}", p2_score * die.rolls);
                break;
            }

            //println!("p2 score={} p2_pos={}", p2_score, p2_pos+1);
            p2_pos += die.roll() + die.roll() + die.roll();
            p2_pos %= BOARD_SIZE;
            p2_score += p2_pos+1;

            if p2_score >= WINNING_SCORE {
                println!("P2 WINS!!!");
                println!("P1 score:{} P2 score:{} rolls:{}", p1_score, p2_score, die.rolls);
                println!("P1 score * rolls = {}", p1_score * die.rolls);
                break;
            }

        }

    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
