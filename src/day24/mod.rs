use crate::*;
use regex::Regex;
//use std::collections::HashSet;
//use std::collections::VecDeque;
use std::fmt;
//use std::mem;

const FILENAME: &str = "src/day24/day24_input.txt";

struct Registers {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
    wtaint: bool,
    xtaint: bool,
    ytaint: bool,
    ztaint: bool,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            wtaint: false,
            xtaint: false,
            ytaint: false,
            ztaint: false,
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "w:{:3}{} x:{:3}{} y:{:3}{} z:{:3}{}",
            self.w,
            if self.wtaint { "*" } else { " " },
            self.x,
            if self.xtaint { "*" } else { " " },
            self.y,
            if self.ytaint { "*" } else { " " },
            self.z,
            if self.ztaint { "*" } else { " " }
        )
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Arg {
    W,
    X,
    Y,
    Z,
    Val(i32),
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arg::W => write!(f, "  w"),
            Arg::X => write!(f, "  x"),
            Arg::Y => write!(f, "  y"),
            Arg::Z => write!(f, "  z"),
            Arg::Val(v) => write!(f, "{:3}", v),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Op {
    Inp(Arg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Div(Arg, Arg),
    Mod(Arg, Arg),
    Eql(Arg, Arg),
}

impl Op {
    fn new() -> Op {
        Op::Inp(Arg::Val(0))
    }

    fn execute(&self, regs: &mut Registers) {
        match self {
            Op::Inp(a) => {
                panic!("Todo");
            }
            Op::Add(a, b) => {
                panic!("Todo");
            }
            Op::Mul(a, b) => {
                panic!("Todo");
            }
            Op::Div(a, b) => {
                panic!("Todo");
            }
            Op::Mod(a, b) => {
                panic!("Todo");
            }
            Op::Eql(a, b) => {
                panic!("Todo");
            }
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Inp(a) => write!(f, "inp {}", a),
            Op::Add(a, b) => write!(f, "add {} {}", a, b),
            Op::Mul(a, b) => write!(f, "mul {} {}", a, b),
            Op::Div(a, b) => write!(f, "div {} {}", a, b),
            Op::Mod(a, b) => write!(f, "mod {} {}", a, b),
            Op::Eql(a, b) => write!(f, "eql {} {}", a, b),
        }
    }
}

#[allow(dead_code)]
pub fn day24_p1() {
    println!("Day 24 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let re_comment = Regex::new(r"^#.*").unwrap();
        let re_opinp = Regex::new(r"^inp [wxyz]$").unwrap();
        let re_opregval = Regex::new(r"^(...) ([wxyz]) (-?\d+)$").unwrap();
        let re_opregreg = Regex::new(r"^(...) ([wxyz]) ([wxyz])$").unwrap();

        for line in lines {
            if let Ok(ip) = line {}
        }
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}
