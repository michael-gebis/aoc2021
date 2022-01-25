// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;
use std::str::FromStr;
mod day01;
mod day04;
mod day05;
mod day06;
mod day07;
mod util;
use util::*;


// TODO: refactor into being test driven
// TODO: since each day is mostly independant, split into day-by-day files

fn day2p1() {
    println!("Day 2 Puzzle 1");

    let mut hpos: i32 = 0;
    let mut depth: i32 = 0;

    if let Ok(lines) = read_lines("data/day2_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(" ").collect();
                let dir: &str = v[0];
                let val: i32 = i32::from_str(v[1]).unwrap();
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
    println!("hpos:{} depth:{} answer:{}", hpos, depth, hpos * depth);
}

fn day2p2() {
    println!("Day 2 Puzzle 2");

    let mut hpos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    if let Ok(lines) = read_lines("data/day2_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip.split(" ").collect();
                let dir: &str = v[0];
                let val: i32 = i32::from_str(v[1]).unwrap();
                // println!("dir:{} val:{}", dir, val);

                match dir {
                    "forward" => {
                        hpos += val;
                        depth += aim * val;
                    }
                    "up" => aim -= val,
                    "down" => aim += val,
                    _ => println!("bad input"),
                }

                // println!("hpos:{} depth:{} aim:{}", hpos, depth, aim);
            }
        }
    }
    println!(
        "hpos:{} depth:{} aim:{} answer:{}",
        hpos,
        depth,
        aim,
        hpos * depth
    );
}

fn get_common_in_pos(pos: usize, d: &Vec<String>) -> u32 {
    // TODO: maybe I should make this more generic but whatev
    let mut c0: usize = 0;
    let mut c1: usize = 0;

    d.iter().for_each(|x| {
        match x.chars().nth(pos).unwrap() {
            '0' => c0 += 1,
            '1' => c1 += 1,
            _ => panic!("bad input"),
        };
    });

    if c0 > c1 {
        return 0;
    }

    1
}

fn get_counts_in_pos(pos: usize, d: &Vec<String>) -> (u32, u32) {
    // TODO: maybe I should make this more generic but whatev
    let mut c0 = 0;
    let mut c1 = 0;

    d.iter().for_each(|x| {
        match x.chars().nth(pos).unwrap() {
            '0' => c0 += 1,
            '1' => c1 += 1,
            _ => panic!("bad input"),
        };
    });
    (c0, c1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_common_in_pos() {
        let v = vec![
            "000".to_string(),
            "001".to_string(),
            "011".to_string(),
            "101".to_string(),
        ];

        let common = get_common_in_pos(0, &v);
        assert_eq!(0, common);

        let common = get_common_in_pos(1, &v);
        assert_eq!(0, common);

        let common = get_common_in_pos(2, &v);
        assert_eq!(1, common);
    }
}

fn day3_p1a() -> i32 {
    println!("Day 3 Puzzle 1a");

    if let Ok(lines) = read_lines("data/day3_input.txt") {
        let mut bitcount = 0;
        let mut d = Vec::new();

        // Grab all lines, put in a vector; also extract bitcount.
        // Bet you a $million there's a better way to do this
        for line in lines {
            if let Ok(bits) = line {
                if bitcount == 0 {
                    bitcount = bits.len();
                    println!("Found {} bits", bitcount);
                }
                d.push(bits)
            }
        }

        let mut gamma = 0;
        let mut epsilon = 0;
        for bit in 0..bitcount {
            gamma *= 2;
            epsilon *= 2;
            match get_common_in_pos(bit, &d) {
                0 => gamma += 1,
                1 => epsilon += 1,
                _ => panic!("bad input"),
            }
        }
        println!(
            "gamma:{} epsilon:{} answer:{}",
            gamma,
            epsilon,
            gamma * epsilon
        );
        return gamma * epsilon;
    }

    0
}

fn bin2u32(x: &String) -> u32 {
    let mut v: u32 = 0;
    for i in 0..x.len() {
        v *= 2;
        match x.chars().nth(i).unwrap() {
            '0' => continue,
            '1' => v += 1,
            _ => panic!("bad input"),
        }
    }
    v
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
        let mut gamma: [u32; 12] = [0; 12];
        let mut epsilon: [u32; 12] = [0; 12];

        for bit in 0..12 {
            if count_0[bit] > count_1[bit] {
                gamma[bit] = 0;
                epsilon[bit] = 1;
            } else {
                gamma[bit] = 1;
                epsilon[bit] = 0;
            }
        }
        println!("gamma:  {:?}", gamma);
        println!("epsilon:{:?}", epsilon);
        let mut gval: u32 = 0;
        let mut eval: u32 = 0;

        for bit in 0..12 {
            gval = gval * 2;
            if gamma[bit] == 1 {
                gval += 1;
            }
            eval = eval * 2;
            if epsilon[bit] == 1 {
                eval += 1;
            }
        }
        println!("gval:{} eval:{} answer:{}", gval, eval, gval * eval);
    }
}

fn day3_p2a() {
    println!("Day 3 Puzzle 2a");

    if let Ok(lines) = read_lines("data/day3_input.txt") {
        let mut bitcount = 0;
        let mut d = Vec::new();

        // Grab all lines, put in a vector; also extract bitcount.
        // Bet you a $million there's a better way to do this
        for line in lines {
            if let Ok(bits) = line {
                if bitcount == 0 {
                    bitcount = bits.len();
                    println!("Found {} bits", bitcount);
                }
                d.push(bits)
            }
        }
        //        match x.chars().nth(pos).unwrap() {

        let mut ogen = d.clone();
        let mut bit = 0;
        while ogen.len() > 1 && bit < bitcount {
            let mut ogennew = Vec::new();
            let (c0, c1) = get_counts_in_pos(bit, &ogen);
            let mostcommon = if c0 > c1 { 0 } else { 1 };

            println!("Bit:{} c0:{} c1:{} mostcommon:{}", bit, c0, c1, mostcommon);
            for entry in ogen {
                if entry.chars().nth(bit).unwrap().to_digit(10).unwrap() == mostcommon {
                    ogennew.push(entry);
                }
            }
            println!("ogennew={:?}", ogennew);
            ogen = ogennew.clone();
            bit += 1;
        }
        println!("ogen={:?} val={}", ogen, bin2u32(&ogen[0]));
        let mut co2 = d.clone();
        let mut bit = 0;
        while co2.len() > 1 && bit < bitcount {
            let mut co2new = Vec::new();
            let (c0, c1) = get_counts_in_pos(bit, &co2);
            let leastcommon = if c0 <= c1 { 0 } else { 1 };

            println!(
                "Bit:{} c0:{} c1:{} leastcommon:{}",
                bit, c0, c1, leastcommon
            );
            for entry in co2 {
                if entry.chars().nth(bit).unwrap().to_digit(10).unwrap() == leastcommon {
                    co2new.push(entry);
                }
            }
            println!("co2new={:?}", co2new);
            co2 = co2new.clone();
            bit += 1;
        }
        println!("co2={:?} val={}", ogen, bin2u32(&co2[0]));

        println!("ogen*co2={}", bin2u32(&ogen[0]) * bin2u32(&co2[0]));
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

                let mut val: u32 = 0;

                for bit in 0..5 {
                    val *= 2;
                    //if ip.chars().nth(i).unwrap()
                    match ip.chars().nth(bit).unwrap() {
                        '0' => count_0[bit] += 1,
                        '1' => {
                            count_1[bit] += 1;
                            val += 1;
                        }
                        _ => println!("bad input"),
                    }
                }
                vals.push(val);
            }
        }
        let mut gamma: [u32; 12] = [0; 12];
        let mut epsilon: [u32; 12] = [0; 12];
        let mut gval: u32 = 0;
        let mut eval: u32 = 0;

        for bit in 0..5 {
            if count_0[bit] > count_1[bit] {
                gamma[bit] = 0;
                epsilon[bit] = 1;
                eval |= 1 << (4 - bit);
            } else {
                gamma[bit] = 1;
                epsilon[bit] = 0;
                gval |= 1 << (4 - bit);
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
                    println!(
                        "  val {:02x} bit {:02x} out {:02x}",
                        *val,
                        (1 << bit),
                        (*val & (1 << bit))
                    );
                    if (*val & (1 << bit)) != 0 {
                        println!("    adding {}", *val);
                        gvals_filt.push(*val);
                    }
                }
                gvals = gvals_filt.clone();
            }
            if evals.len() > 1 {
                let mut evals_filt = Vec::new();

                for val in &evals {
                    if (*val & 1 << bit) == 0 {
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
    day01::day1p1();
    day01::day1p2();

    //day2p1();
    //day2p2();

    day3_p2a();
    //day3p2();

    day04::day04_p1();

    day05::day05_p1();

    //day06::day06_p1();
    //day06::day06_p2();

    day07::day07_p1();
}

