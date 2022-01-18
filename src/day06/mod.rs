use std::str::FromStr;
use crate::*;
//use regex::Regex;
//use std::cmp::Ord;

//const GRIDSIZE:usize = 1000;

//#[allow(dead_code)]


pub fn day06_p1() {
    println!("day06_p1!");

    if let Ok(lines) = util::read_lines("src/day06/day06_example.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut vals: Vec<i32> = ip.split(",").map(|x| x.parse().unwrap()).collect();

                println!("Initial state {}: {:?}", vals.len(), vals);
                for i in 0..80 {
                    let mut new = 0;
                    for i in vals.iter_mut() {
                        if *i == 0 {
                            new += 1;
                            *i = 6;
                        } else {
                            *i -=1;
                        }
                    }
                    for i in 0..new {
                        vals.push(8);
                    }
                    println!("After day {} ({})", i+1, vals.len());

                }
                //drawgrid(&arr);
            }
        }
    }
}

pub fn day06_p2() {
    println!("day06_p2!");
    if let Ok(lines) = util::read_lines("src/day06/day06_input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut vals: Vec<usize> = ip.split(",").map(|x| x.parse().unwrap()).collect();

                println!("Initial state {}: {:?}", vals.len(), vals);
                let mut states: [usize; 9] = [0; 9];

                for i in vals.iter_mut() {
                    states[*i]+=1;
                }
                let sum: usize = states.iter().sum();
                println!("states {} {:?}",  sum, states);

                for i in 0..256 {
                    let new:usize = states[0];
                    for j in 0..8 {
                        states[j] = states[j+1];
                    }
                    states[8] = new;
                    states[6] += new;
                    let sum: usize = states.iter().sum();
                    println!("states {} {:?}",  sum, states);                    
                }
/*
                for i in 0..80 {
                    let mut new = 0;
                    for i in vals.iter_mut() {
                        if *i == 0 {
                            new += 1;
                            *i = 6;
                        } else {
                            *i -=1;
                        }
                    }
                    for i in 0..new {
                        vals.push(8);
                    }
                    println!("After day {} ({})", i+1, vals.len());

                }
*/
                //drawgrid(&arr);
            }
        }
    }
}
