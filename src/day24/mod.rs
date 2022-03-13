use crate::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
//use std::mem;

const FILENAME: &str = "src/day24/day24_input.txt";
//const FILENAME: &str = "src/day24/day24_mini1.txt";

struct Registers {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
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
    fn clear(&mut self) {
        self.w = 0;
        self.x = 0;
        self.y = 0;
        self.z = 0;
        self.wtaint = false;
        self.xtaint = false;
        self.ytaint = false;
        self.ztaint = false;
    }

    fn setreg(&mut self, reg: Arg, val: i64) {
        match reg {
            Arg::W => self.w = val,
            Arg::X => self.x = val,
            Arg::Y => self.y = val,
            Arg::Z => self.z = val,
            _ => panic!("Unknown reg"),
        }
    }

    fn settaint(&mut self, reg: Arg, val: bool) {
        match reg {
            Arg::W => self.wtaint = val,
            Arg::X => self.xtaint = val,
            Arg::Y => self.ytaint = val,
            Arg::Z => self.ztaint = val,
            _ => panic!("Unknown reg"),
        }
    }

    fn getreg(&self, reg: Arg) -> i64 {
        match reg {
            Arg::W => self.w,
            Arg::X => self.x,
            Arg::Y => self.y,
            Arg::Z => self.z,
            _ => panic!("Unknown reg"),
        }
    }

    fn gettaint(&self, reg: Arg) -> bool {
        match reg {
            Arg::W => self.wtaint,
            Arg::X => self.xtaint,
            Arg::Y => self.ytaint,
            Arg::Z => self.ztaint,
            _ => false,
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}w:{:3},{}x:{:3} {}y:{:3}, {}z:{:3}",
            if self.wtaint { "*" } else { " " },
            self.w,
            if self.xtaint { "*" } else { " " },
            self.x,
            if self.ytaint { "*" } else { " " },
            self.y,
            if self.ztaint { "*" } else { " " },
            self.z
        )
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Arg {
    W,
    X,
    Y,
    Z,
    Val(i64),
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

impl Arg {
    fn new(reg: &str) -> Arg {
        match reg {
            "w" => Arg::W,
            "x" => Arg::X,
            "y" => Arg::Y,
            "z" => Arg::Z,
            _ => Arg::Val(String::from(reg).parse().unwrap()),
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
    //fn new() -> Op {
    //    Op::Inp(Arg::Val(0))
    //}

    fn new_1op(op: &str, arg: &str) -> Op {
        match op {
            "inp" => Op::Inp(Arg::new(arg)),
            _ => panic!("unknown op '{}'", op),
        }
    }

    fn new_2op(op: &str, arg1: &str, arg2: &str) -> Op {
        //let arg1 = new Arg(reg)
        //let arg2 = new Arg(val);

        match op {
            "add" => Op::Add(Arg::new(arg1), Arg::new(arg2)),
            "mul" => Op::Mul(Arg::new(arg1), Arg::new(arg2)),
            "div" => Op::Div(Arg::new(arg1), Arg::new(arg2)),
            "mod" => Op::Mod(Arg::new(arg1), Arg::new(arg2)),
            "eql" => Op::Eql(Arg::new(arg1), Arg::new(arg2)),
            _ => panic!("Unknown op '{}'", op),
        }
    }

    fn execute(&self, regs: &mut Registers, input: &mut VecDeque<i64>) {
        //println!("Executing {}", self);
        match self {
            Op::Inp(a) => {
                regs.setreg(*a, input.pop_front().unwrap());
                regs.settaint(*a, true);
            }
            Op::Add(a, b) => {
                let val = if let Arg::Val(x) = b {
                    *x
                } else {
                    regs.getreg(*b)
                };
                let sum = regs.getreg(*a) + val;
                regs.setreg(*a, sum);
                regs.settaint(*a, regs.gettaint(*a) | regs.gettaint(*b));
            }
            Op::Mul(a, b) => {
                let val = if let Arg::Val(x) = b {
                    *x
                } else {
                    regs.getreg(*b)
                };
                let prod = regs.getreg(*a) * val;
                regs.setreg(*a, prod);
                if let Arg::Val(x) = b {
                    if *x == 0 {
                        regs.settaint(*a, false);
                    }
                } else {
                    regs.settaint(*a, regs.gettaint(*a) | regs.gettaint(*b));
                }
            }
            Op::Div(a, b) => {
                let val = if let Arg::Val(x) = b {
                    *x
                } else {
                    regs.getreg(*b)
                };
                let div = regs.getreg(*a) / val;
                regs.setreg(*a, div);
                regs.settaint(*a, regs.gettaint(*a) | regs.gettaint(*b));
            }
            Op::Mod(a, b) => {
                let val = if let Arg::Val(x) = b {
                    *x
                } else {
                    regs.getreg(*b)
                };
                let modulus = regs.getreg(*a) % val;
                regs.setreg(*a, modulus);
                regs.settaint(*a, regs.gettaint(*a) | regs.gettaint(*b));
            }
            Op::Eql(a, b) => {
                let val = if let Arg::Val(x) = b {
                    *x
                } else {
                    regs.getreg(*b)
                };
                let eql = if regs.getreg(*a) == val { 1 } else { 0 };
                regs.setreg(*a, eql);
                regs.settaint(*a, regs.gettaint(*a) | regs.gettaint(*b));
            }
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Inp(a) => write!(f, "inp {}    ", a),
            Op::Add(a, b) => write!(f, "add {} {}", a, b),
            Op::Mul(a, b) => write!(f, "mul {} {}", a, b),
            Op::Div(a, b) => write!(f, "div {} {}", a, b),
            Op::Mod(a, b) => write!(f, "mod {} {}", a, b),
            Op::Eql(a, b) => write!(f, "eql {} {}", a, b),
        }
    }
}
const VERBOSE: bool = true;
fn simulate(prog: VecDeque<Op>, input: u64) -> i64 {
    //let input:Vec<&str> = String::from(inp).split("").filter(|x| !x.is_empty()).collect();
    if VERBOSE {
        println!("Simulate: {}", input);
    }
    let mut inp: VecDeque<i64> = VecDeque::new();
    let mut val = input;
    for _i in 0..14 {
        inp.push_front((val % 10) as i64);
        val /= 10;
    }

    let mut regs = Registers::new();

    for op in prog {
        if VERBOSE {
            if let Op::Inp(_a) = op {
                println!("");
            }
        }
        op.execute(&mut regs, &mut inp);
        if VERBOSE {
            println!("{}     ::{}", op, regs);
        }
        //if let Op::Inp(_a) = op { println!("");}
    }

    return regs.getreg(Arg::Z);
}

fn staged_simulate(prog: VecDeque<Op>) {
    if VERBOSE {
        println!("Staged simulate");
    }

    let mut inp_offsets: Vec<i32> = Vec::new();
    let mut count = 0;

    for op in prog.clone() {
        if let Op::Inp(_) = op {
            inp_offsets.push(count);
        }
        count += 1;
    }
    inp_offsets.push(prog.len() as i32);

    println!("inp_offsets={:?}", inp_offsets);

    //0..1, 1..2, 2..3, 3..4, etc.
    let mut prev_zouts: HashMap<i64, i64> = HashMap::new();

    prev_zouts.insert(0, 0);
    for stage in 0..inp_offsets.len() - 1 {
        let mut zouts: HashMap<i64, i64> = HashMap::new();
        println!("Running stage {} with {} zs", stage, prev_zouts.len());

        for (z, biggest_inp) in prev_zouts {
            //if z == 275353 { println!("z={}, biggest_inp={}", z, biggest_inp); }
            for inpval in 1..10 {
                let mut inp: VecDeque<i64> = VecDeque::new();
                inp.push_back(inpval);
                let mut regs = Registers::new();
                regs.setreg(Arg::Z, z);

                for i in *inp_offsets.get(stage as usize).unwrap()
                    ..*inp_offsets.get(stage as usize + 1).unwrap()
                {
                    prog[i as usize].execute(&mut regs, &mut inp);
                }
                let zout = regs.getreg(Arg::Z);
                let tmpinp = biggest_inp * 10 + inpval;
                if zouts.contains_key(&zout) {
                    if zouts[&zout] > tmpinp {
                        zouts.insert(zout, tmpinp);
                    }
                } else {
                    zouts.insert(zout, tmpinp);
                }
            }
        }
        prev_zouts = zouts;
    }
    println!("prev_zouts.len()={}", prev_zouts.len());
    println!("inp={}", prev_zouts.get(&0).unwrap());
}
//} else if let Some(cap) = re_line2.captures(&ip) {
//    board.set_room(RoomType::A, 0, BState::new(&cap[1]));

#[allow(dead_code)]
pub fn day24_p1() {
    println!("Day 24 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let re_comment = Regex::new(r"^#.*").unwrap();
        let re_opinp = Regex::new(r"^(inp) ([wxyz])$").unwrap();
        let re_opregval = Regex::new(r"^(...) ([wxyz]) (-?\d+)$").unwrap();
        let re_opregreg = Regex::new(r"^(...) ([wxyz]) ([wxyz])$").unwrap();

        let mut prog: VecDeque<Op> = VecDeque::new();

        for line in lines {
            if let Ok(ip) = line {
                if let Some(_cap) = re_comment.captures(&ip) {
                    println!("Ignoring comment: {}", ip);
                } else if let Some(cap) = re_opinp.captures(&ip) {
                    prog.push_back(Op::new_1op(&cap[1], &cap[2]));
                } else if let Some(cap) = re_opregval.captures(&ip) {
                    prog.push_back(Op::new_2op(&cap[1], &cap[2], &cap[3]));
                } else if let Some(cap) = re_opregreg.captures(&ip) {
                    prog.push_back(Op::new_2op(&cap[1], &cap[2], &cap[3]));
                }
            }
        }
        staged_simulate(prog);
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}
