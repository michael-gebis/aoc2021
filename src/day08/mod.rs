use std::collections::HashMap;
//use std::str::FromStr;
use crate::*;

pub fn str_is_subset(big: &String, small:&String) -> bool {

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
        let mut total = 0;
        for line in lines {
            println!("**********");
            if let Ok(ip) = dbg!(line) {
                let vals: Vec<&str> = ip.split(" | ").collect();
                let left: Vec<&str> = vals[0].split(" ").collect();
                let right: Vec<&str> = vals[1].split(" ").collect();

                let mut fives: Vec<String> = Vec::new();
                let mut sixes: Vec<String> = Vec::new();

                let mut strtoval: HashMap<String, i32> = HashMap::new();
                // Not sure if I'll need this
                let mut valtostr: [String; 10] = Default::default();

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
                        2 => { strtoval.insert(x.clone(), 1); valtostr[1]=x; },
                        3 => { strtoval.insert(x.clone(), 7); valtostr[7]=x; },
                        4 => { strtoval.insert(x.clone(), 4); valtostr[4]=x; },
                        5 => { fives.push(x.clone()); } // should be 3 things
                        6 => { sixes.push(x.clone()); } // should be 3 things
                        7 => { strtoval.insert(x.clone(), 8); valtostr[8]=x; },
                        _ => panic!("help {}", x),
                    }


                    //q.append(chars);
                }

                dbg!(valtostr.clone());

                // find 9
                println!("sixes contains={:?}", sixes);
                for s in sixes.iter() {
                    println!("checking 6 segment={:?}", s);
                    if str_is_subset(s, &valtostr[4]) {
                        strtoval.insert(s.clone(), 9);
                        valtostr[9]=s.clone();
                        println!("{} is a subset of {}, thus 9={}", valtostr[4], s, s);
                        break;
                    }
                }

                let index = sixes.iter().position(|x| *x == valtostr[9]).unwrap();
                sixes.remove(index);
                println!("sixes (after removal of 9): {:?}", sixes);

                // find 0
                println!("sixes contains={:?}", sixes);
                for s in sixes.iter() {
                    println!("checking 6 segment={:?}", s);
                    if str_is_subset(s, &valtostr[1]) {
                        strtoval.insert(s.clone(), 0);
                        valtostr[0]=s.clone();
                        println!("{} is a subset of {}, thus 0={}", valtostr[1], s, s);
                        break;
                    }
                }

                let index = sixes.iter().position(|x| *x == valtostr[0]).unwrap();
                sixes.remove(index);
                println!("sixes (after removal of 0): {:?}", sixes);

                // Remaining six-segment is 6
                strtoval.insert(sixes[0].clone(), 6);
                valtostr[6]=sixes[0].clone();
                sixes.remove(0);

                // Find 3
                for s in fives.iter() {
                    println!("checking 5 digit ={:?}", s);
                    // The 
                    if str_is_subset(s, &valtostr[1]) {
                        strtoval.insert(s.clone(), 3);
                        valtostr[3]=s.clone();
                        println!("{:?} is a superset of {:?}, therefore 3={:?}",s, valtostr[1], s);
                        break;
                    }
                }
                let index = fives.iter().position(|x| *x == valtostr[3]).unwrap();
                fives.remove(index);
                println!("fives (after removal of 3): {:?}", fives);
                
                // Find 5
                for s in fives.iter() {
                    println!("checking 5 digit ={:?}", s);
                    // The 
                    if str_is_subset(&valtostr[6], s) {
                        strtoval.insert(s.clone(), 5);
                        valtostr[5]=s.clone();
                        println!("{:?} is a superset of {:?}, therefore 5={:?}",s, valtostr[1], s);
                        break;
                    }
                }
                let index = fives.iter().position(|x| *x == valtostr[5]).unwrap();
                fives.remove(index);
                println!("fives (after removal of 5): {:?}", fives);

                // Remaining five-segment is 2
                strtoval.insert(fives[0].clone(), 2);
                valtostr[2]=fives[0].clone();
                fives.remove(0);

                // Print the digits we've found.
                dbg!(valtostr.clone());

                let mut output_val = 0;
                for t in right {
                    output_val *= 10;
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
                    println!("output = {}", strtoval[&x]);
                    output_val += strtoval[&x];
                }
                println!("output_val={}", output_val);
                total+=output_val;
                //let mut vals: Vec<usize> = ip.split(",").map(|x| x.parse().unwrap()).collect();
            }
        }
        dbg!(counter);
        dbg!(total);

        dbg!(str_is_subset(&"abcdefg".to_string(),&"ace".to_string()));
        dbg!(str_is_subset(&"abcdefg".to_string(),&"acgh".to_string()));
    }
}