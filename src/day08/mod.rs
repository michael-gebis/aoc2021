use std::collections::HashMap;
//use std::str::FromStr;
use crate::*;

pub fn str_is_subset(big: String, small:String) -> bool {

    //let bigc: Vec<&str> = big.split("").collect();
    //let smallc: Vec<&str> = small.split("").collect();

    for c in small.split("") {
        //match bigc.into_iter().find(|&x| x==c) {
        match big.split("").find(|&x| x== c ) {
            Some(_) => (),
            None => { return false; }
        }
    }

    true
}

pub fn day08_p1() {
    println!("day08_p1!");

    if let Ok(lines) = util::read_lines("src/day08/day08_input.txt") {
        let mut counter = 0;
        for line in lines {
            if let Ok(ip) = dbg!(line) {
                let vals: Vec<&str> = ip.split(" | ").collect();
                let left: Vec<&str> = vals[0].split(" ").collect();
                let right: Vec<&str> = vals[1].split(" ").collect();

                let mut fives: Vec<String> = Vec::new();
                let mut sixes: Vec<String> = Vec::new();

                let mut strtoval: HashMap<String, i32> = HashMap::new();

                for t in left {
                    let mut chars: Vec<&str> = t.split("").collect();
                    chars.sort();
                    let mut x: String = "".to_string();
                    for c in chars {
                        x.push_str(c);
                    }
                    dbg!(x.clone());
                    dbg!(x.len());

                    match x.len() {
                        2 => { strtoval.insert(x, 1); },
                        3 => { strtoval.insert(x, 7); },
                        4 => { strtoval.insert(x, 4); },
                        5 => { fives.push(x); }
                        6 => { sixes.push(x); }
                        7 => { strtoval.insert(x, 8); },
                        _ => panic!("help {}", x),
                    }
                    //q.append(chars);


                }



                for t in right {
                    let mut chars: Vec<&str> = t.split("").collect();
                    chars.sort();
                    let mut x: String = "".to_string();
                    for c in chars {
                        x.push_str(c);
                    }
                    match x.len() {
                        2 | 3 | 4 | 7 => counter += 1,
                        _ => (),
                    }
                }
                //let mut vals: Vec<usize> = ip.split(",").map(|x| x.parse().unwrap()).collect();
            }
        }
        dbg!(counter);

        dbg!(str_is_subset("abcdefg".to_string(),"ace".to_string()));
        dbg!(str_is_subset("abcdefg".to_string(),"acgh".to_string()));
    }
}