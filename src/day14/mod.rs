use crate::*;
use std::collections::HashMap;
use regex::Regex;

//const FILENAME: &str = "src/day14/day14_input.txt";
const FILENAME: &str = "src/day14/day14_example.txt";

pub fn day14_p1() {
    println!("Day 14 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut rules: HashMap<(String, String), String> = HashMap::new();
        let mut polymer:Vec<String> = Vec::new();
        let re_insertion_rules = Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();
        let re_polymer_template = Regex::new(r"^([A-Z]+)$").unwrap();

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_insertion_rules.captures(&ip) {
                    let (left,right,out):(String,String,String) = 
                        (cap[1].to_string(), cap[2].to_string(), cap[3].to_string());

                    println!("inserting rule {},{} -> {}", left, right, out);
                    rules.insert((left,right), out);

                } else if let Some(cap) = re_polymer_template.captures(&ip) {
                    polymer = cap[1].split("").filter(|x| !x.is_empty()).map(|x| x.to_string()).collect();
                    //let initial_polymer:Vec<&str> = cap[1].split("").filter(|x| !x.is_empty()).collect();
                    println!("initial polymer: {:?}", polymer);
                    //polymer = initial_polymer;
                } else {
                    println!("ignored line:'{}'", ip);
                }
            }
        }
        for step in 0..10 {
            let mut new_polymer:Vec<String> = Vec::new();

            println!("Step {}", step);
            println!("Step {}, polymer={:?}", step, polymer);
            for i in 0..polymer.len()-1 {
                new_polymer.push(polymer[i].clone());
                let new_pair = (polymer[i].clone(), polymer[i+1].clone());
                let new_letter = rules.get(&new_pair).unwrap().clone();
                new_polymer.push(new_letter);
                //new_polymer.push(polymer[i+1].clone());
            }
            new_polymer.push(polymer[polymer.len()-1].clone());
            polymer=new_polymer;
        }
        //println!("Final polymer:{:?}", polymer);
        println!("Done polymerizign!");
        let mut counts = HashMap::new();
        for l in polymer.iter() {
            *counts.entry(l.clone()).or_insert(0) +=1;
        }
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut min_let = "".to_string();
        let mut max_let = "".to_string();

        for (l,c) in counts.iter() {
            println!("l={} c={}", *l, *c);
            if *c < min {
                min = *c;
                min_let = (*l).clone();
            }
            if *c > max {
                max = *c;
                max_let = (*l).clone();
            }
        }
        println!("max:{} ({}) min:{}({})", max, max_let, min, min_let);

        println!("max:{} ({}) min:{}({}) diff:{}", max, max_let, min, min_let, max-min);


    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}