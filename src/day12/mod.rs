use crate::*;
use itertools::Itertools;
use std::collections::HashMap;

const FILENAME: &str = "src/day12/day12_input.txt";
//const FILENAME: &str = "src/day12/day12_exampleA.txt";
//const FILENAME: &str = "src/day12/day12_exampleB.txt";
//const FILENAME: &str = "src/day12/day12_exampleC.txt";

// TODO: make this less inefficient, way too much copying
// TODO: make it handle both part 1 and 2 of problem
pub fn path_is_legal(
    path: &Vec<String>,
    addend: String,
    neighbors: &HashMap<String, Vec<String>>,
) -> bool {
    if addend == "start" {
        return false;
    }
    let mut little_caves_with_multiple_visits: Vec<String> = Vec::new();

    let mut newpath = path.clone();
    newpath.push(addend);
    //println!("checking path {:?}", newpath);
    for n in neighbors.keys() {
        //println!("  neighbor={}", n);
        if n.to_uppercase() == *n {
            continue;
        }
        let count = newpath.iter().filter(|x| *x == n).count();
        //println!("  count={}", count);
        if count > 2 {
            return false;
        } else if count > 1 {
            little_caves_with_multiple_visits.push(n.clone());
        }
    }
    //println!("  lc={:?}", little_caves_with_multiple_visits);
    if little_caves_with_multiple_visits.len() > 1 {
        return false;
    }
    true
}

pub fn day12_p1() {
    println!("Day 12 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut neighbors: HashMap<String, Vec<String>> = HashMap::new();
        let mut count = 0;
        for line in lines {
            //let mut mystack: Vec<&str> = Vec::new();
            if let Ok(ip) = line {
                let (left, right) = ip.trim().split("-").collect_tuple().unwrap();

                // I think there's a better way than this
                if let Some(v) = neighbors.get_mut(left) {
                    v.push(right.to_string());
                } else {
                    neighbors.insert(left.to_string(), vec![right.to_string()]);
                    //neighbors[left].push(right);
                }

                if let Some(v) = neighbors.get_mut(right) {
                    v.push(left.to_string());
                } else {
                    neighbors.insert(right.to_string(), vec![left.to_string()]);
                }

                println!("{},{}", left, right);
                //linecount += 1;
                //let row: Vec<&str> = ip.trim().split("").filter(|x| !x.is_empty()).collect();
            }
        }

        let mut todo: Vec<Vec<String>> = Vec::new();
        //dbg!(&neighbors);

        todo.push(vec!["start".to_string()]);

        while !todo.is_empty() {
            let x = todo.pop().unwrap();

            //println!("Extending {:?}", x);
            for n in neighbors[x.last().unwrap()].iter() {
                if n == "end" {
                    count += 1;
                    let mut y = x.clone();
                    y.push(n.clone());
                    println!("Found path {:?}", y);
                }
                //            else if n != "start" && (n.to_uppercase() == *n || x.iter().filter(|q| *q==n).count() < 2) {
                else if path_is_legal(&x, n.to_string(), &neighbors) {
                    let mut y = x.clone();
                    y.push(n.clone());
                    //println!("pushing {:?}", y);
                    todo.push(y);
                }
            }
        }
        println!("Found {} paths", count);
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
