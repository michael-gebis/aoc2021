use crate::*;
use regex::Regex;
use std::collections::HashMap;

const FILENAME: &str = "src/day14/day14_input.txt";
//const FILENAME: &str = "src/day14/day14_example.txt";

#[allow(dead_code)]
pub fn day14_p1() {
    println!("Day 14 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut rules: HashMap<(String, String), String> = HashMap::new();
        let mut polymer: Vec<String> = Vec::new();
        let re_insertion_rules = Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();
        let re_polymer_template = Regex::new(r"^([A-Z]+)$").unwrap();

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_insertion_rules.captures(&ip) {
                    let (left, right, out): (String, String, String) =
                        (cap[1].to_string(), cap[2].to_string(), cap[3].to_string());

                    println!("inserting rule {},{} -> {}", left, right, out);
                    rules.insert((left, right), out);
                } else if let Some(cap) = re_polymer_template.captures(&ip) {
                    polymer = cap[1]
                        .split("")
                        .filter(|x| !x.is_empty())
                        .map(|x| x.to_string())
                        .collect();
                    //let initial_polymer:Vec<&str> = cap[1].split("").filter(|x| !x.is_empty()).collect();
                    println!("initial polymer: {:?}", polymer);
                    //polymer = initial_polymer;
                } else {
                    println!("ignored line:'{}'", ip);
                }
            }
        }
        for step in 0..10 {
            let mut new_polymer: Vec<String> = Vec::new();

            println!("Step {}", step);
            println!("Step {}, polymer={:?}", step, polymer);
            for i in 0..polymer.len() - 1 {
                new_polymer.push(polymer[i].clone());
                let new_pair = (polymer[i].clone(), polymer[i + 1].clone());
                let new_letter = rules.get(&new_pair).unwrap().clone();
                new_polymer.push(new_letter);
                //new_polymer.push(polymer[i+1].clone());
            }
            new_polymer.push(polymer[polymer.len() - 1].clone());
            polymer = new_polymer;
        }
        //println!("Final polymer:{:?}", polymer);
        println!("Done polymerizign!");
        let mut counts = HashMap::new();
        for l in polymer.iter() {
            *counts.entry(l.clone()).or_insert(0) += 1;
        }
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut min_let = "".to_string();
        let mut max_let = "".to_string();

        for (l, c) in counts.iter() {
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

        println!(
            "max:{} ({}) min:{}({}) diff:{}",
            max,
            max_let,
            min,
            min_let,
            max - min
        );
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}

// Puzzle 2 demands a more clever solution.
// The size of the polymer is O(2n-1), so if we go to day 40,
// we're talking around a terabyte if we attempt to calculate
// and store the whole thing.

// But we don't need to do that; instead, count adjacent pairs,
// and then modify the counts at each step.

// For example, for the example NNCB:
//  NN: 1 pair
//  NC: 1 pair
//  CB: 1 pair

// Then for each rule, subtract and add as appropriate.
// NN-> C results in NCN, thus -1 NN, +1 NC, +1 CN
// NC-> B results in NBC, thus -1 NC, +1 NB, +1 BC
// CB-> H results in CHB, thus -1 CB, +1 CH, +1 HB
// (The actual polymer is NCNBCHB, which lo and behold matches
// the above counts)

// This applies to the counts; for example, if we had 5 NN pairs,
// to compute the next step would be -5 NN, +5 NC, +5 CN

// There's a little bit of trickiness in counting the actual
// elements and not getting an off-by-one error, since the pairs
// overlap, but that's just details.
#[allow(dead_code)]
pub fn day14_p2() {
    println!("Day 14 Puzzle 2");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut rules: HashMap<(String, String), String> = HashMap::new();
        let mut polymer: Vec<String> = Vec::new();
        let re_insertion_rules = Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();
        let re_polymer_template = Regex::new(r"^([A-Z]+)$").unwrap();
        let mut pair_counts: HashMap<(String, String), i64> = HashMap::new();

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_insertion_rules.captures(&ip) {
                    let (left, right, out): (String, String, String) =
                        (cap[1].to_string(), cap[2].to_string(), cap[3].to_string());

                    println!("inserting rule {},{} -> {}", left, right, out);
                    rules.insert((left, right), out);
                } else if let Some(cap) = re_polymer_template.captures(&ip) {
                    polymer = cap[1]
                        .split("")
                        .filter(|x| !x.is_empty())
                        .map(|x| x.to_string())
                        .collect();
                    //let initial_polymer:Vec<&str> = cap[1].split("").filter(|x| !x.is_empty()).collect();
                    println!("initial polymer: {:?}", polymer);
                    //polymer = initial_polymer;
                    for i in 0..polymer.len() - 1 {
                        *pair_counts
                            .entry((polymer[i].clone(), polymer[i + 1].clone()))
                            .or_insert(0) += 1;
                    }
                } else {
                    println!("ignored line:'{}'", ip);
                }
            }
        }
        println!("pair_counts={:?}", pair_counts);
        for step in 0..40 {
            println!("step {}", step);
            let mut new_pair_counts: HashMap<(String, String), i64> = HashMap::new();

            for (pair, count) in pair_counts.iter() {
                let new_letter = rules.get(pair).unwrap();
                let lpair = (pair.0.clone(), new_letter.clone());
                let rpair = (new_letter.clone(), pair.1.clone());

                *new_pair_counts.entry(lpair).or_insert(0) += count;
                *new_pair_counts.entry(rpair).or_insert(0) += count;
            }
            pair_counts = new_pair_counts;
            println!("pair_counts:{:?}", pair_counts);
        }

        let mut let_counts: HashMap<String, i64> = HashMap::new();
        for (pair, count) in pair_counts.iter() {
            *let_counts.entry(pair.0.clone()).or_insert(0) += count;
            *let_counts.entry(pair.1.clone()).or_insert(0) += count;
        }
        *let_counts.entry(polymer[0].clone()).or_insert(0) += 1;
        *let_counts
            .entry(polymer[polymer.len() - 1].clone())
            .or_insert(0) += 1;

        let mut min = i64::MAX;
        let mut max = i64::MIN;
        let mut min_let = "".to_string();
        let mut max_let = "".to_string();

        for (l, c) in let_counts.iter() {
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

        println!(
            "max:{} ({}) min:{}({}) diff:{} diff/2:{}",
            max,
            max_let,
            min,
            min_let,
            max - min,
            (max - min) / 2
        );
    }
}
