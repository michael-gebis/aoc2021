use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn day1p1() {
    println!("Day 1 Puzzle 1");

    let mut prev:i32 = 99999;
    let mut count:i32 = 0;

    if let Ok(lines) = read_lines("data/day1_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cur:i32 = i32::from_str(&ip).unwrap();
                if cur > prev {
                    count += 1;
                }
                prev=cur;
                // println!("{}", cur);
            }
        }
    }
    println!("count={}",count);
}

fn day1p2() {
    println!("Day 1 Puzzle 2");
    
    let mut p1:i32 = -1;
    let mut p0:i32 = -1;

    let mut prevsum:i32 = 9999999;
    let mut count:i32 = 0;

    if let Ok(lines) = read_lines("data/day1_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let cur:i32 = i32::from_str(&ip).unwrap();

                if p1 == -1 {
                    p1 = cur;
                }
                else if p0 == -1 {
                    p0 = cur;
                }
                else {
                    let sum:i32 = cur + p0 + p1;
                    
                    if sum > prevsum {
                        count += 1;
                    }
                    p1 = p0;
                    p0 = cur;
                    prevsum = sum;
                }

                // println!("p1:{} p0:{} cur:{} prevsum:{}", p1, p0, cur, prevsum);
            }
        }
        println!("count={}",count);
    }
}

fn day2p1() {
    println!("Day 2 Puzzle 1");

    let mut hpos:i32 = 0;
    let mut depth:i32 = 0;

    if let Ok(lines) = read_lines("data/day2_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(" ").collect();
                let dir:&str = v[0];
                let val:i32 = i32::from_str(v[1]).unwrap();
                // println!("dir:{} val:{}", dir, val);

                match dir {
                    "forward" => hpos += val,
                    "up" => depth -= val,
                    "down" => depth += val,
                    _ => println!("bad input"),
                }
            }
        }
    }
    println!("hpos:{} depth:{} answer:{}", hpos, depth, hpos*depth);
}

fn day2p2() {
    println!("Day 2 Puzzle 2");

    let mut hpos:i32 = 0;
    let mut depth:i32 = 0;
    let mut aim:i32 = 0;

    if let Ok(lines) = read_lines("data/day2_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(" ").collect();
                let dir:&str = v[0];
                let val:i32 = i32::from_str(v[1]).unwrap();
                // println!("dir:{} val:{}", dir, val);

                match dir {
                    "forward" => { hpos += val; depth += aim * val;},
                    "up" => aim -= val,
                    "down" => aim += val,
                    _ => println!("bad input"),
                }

                // println!("hpos:{} depth:{} aim:{}", hpos, depth, aim);
            }
        }
    }
    println!("hpos:{} depth:{} aim:{} answer:{}", hpos, depth, aim, hpos*depth);
}

fn day3p1() {
    println!("Day 3 Puzzle 1");

    let mut count_0: [u32; 12] = [0; 12];
    let mut count_1: [u32; 12] = [0; 12];

    if let Ok(lines) = read_lines("data/day3_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                //let cur:i32 = i32::from_str(&ip).unwrap();

                for bit in 0..12 {
                    //if ip.chars().nth(i).unwrap() 
                    match ip.chars().nth(bit).unwrap() {
                        '0' => count_0[bit] += 1,
                        '1' => count_1[bit] += 1,
                        _ => println!("bad input"),
                    }
                }
            }
        }
        let mut gamma: [u32;12] = [0; 12];
        let mut epsilon: [u32;12] = [0; 12];

        for bit in 0..12 {
            if count_0[bit] > count_1[bit] {
                gamma[bit] = 0;
                epsilon[bit] = 1;
            }
            else {
                gamma[bit] = 1;
                epsilon[bit] = 0;
            }
        }
        println!("gamma:  {:?}", gamma);
        println!("epsilon:{:?}", epsilon);
        let mut gval:u32 = 0;
        let mut eval:u32 = 0;

        for bit in 0..12 {
            gval = gval*2;
            if gamma[bit] == 1 {
                gval += 1;
            }
            eval = eval*2;
            if epsilon[bit] == 1 {
                eval += 1;
            }
        }
        println!("gval:{} eval:{} answer:{}", gval, eval, gval*eval);
    }
}

fn day3p2() {
    println!("Day 3 Puzzle 2");

    let mut count_0: [u32; 12] = [0; 12];
    let mut count_1: [u32; 12] = [0; 12];

    let mut vals = Vec::new();

    if let Ok(lines) = read_lines("data/day3_ex.txt") {
        for line in lines {
            if let Ok(ip) = line {
                //let cur:i32 = i32::from_str(&ip).unwrap();

                //vals.push(i32::from_str(&ip).unwrap());

                let mut val:u32 = 0;

                for bit in 0..5 {
                    val *= 2;
                    //if ip.chars().nth(i).unwrap() 
                    match ip.chars().nth(bit).unwrap() {
                        '0' => count_0[bit] += 1,
                        '1' => { count_1[bit] += 1; val += 1; },
                        _ => println!("bad input"),
                    }
                }
                vals.push(val);
            }
        }
        let mut gamma: [u32;12] = [0; 12];
        let mut epsilon: [u32;12] = [0; 12];
        let mut gval:u32 = 0;
        let mut eval:u32 = 0;

        for bit in 0..5 {
            if count_0[bit] > count_1[bit] {
                gamma[bit] = 0;
                epsilon[bit] = 1;
                eval |= 1<<(4-bit);
            }
            else {
                gamma[bit] = 1;
                epsilon[bit] = 0;
                gval |= 1 << (4-bit);
            }
        }
        println!("gamma:  {:?}", gamma);
        println!("epsilon:{:?}", epsilon);
        println!("gval:{}", gval);
        println!("eval:{}", eval);

        println!("vals:{:?}", vals);
        let mut gvals = vals.clone();
        let mut evals = vals.clone();
    

        for bit in (0..5).rev() {
            println!("bit {}", bit);
            if gvals.len() > 1 {
                let mut gvals_filt = Vec::new();

                for val in &gvals {
                    println!("  val {:02x} bit {:02x} out {:02x}", *val,(1<<bit),  (*val & (1<<bit)));
                    if (*val & (1<<bit)) != 0 {
                        println!("    adding {}", *val);
                        gvals_filt.push(*val);
                    }
                }
                gvals = gvals_filt.clone();
            }
            if evals.len() > 1 {
                let mut evals_filt = Vec::new();

                for val in &evals {
                    if (*val & 1<<bit) == 0 {
                        evals_filt.push(*val);
                    }
                }
                evals = evals_filt.clone();
            }

            println!("gvals:{:?}", gvals);
            println!("evals:{:?}", evals);
        }
    

    }

}

fn main() {
    println!("Hello, world!");
    day1p1();
    day1p2();

    day2p1();
    day2p2();

    day3p1();
    day3p2();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}