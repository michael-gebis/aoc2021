use crate::*;

const FILENAME: &str = "src/day10/day10_input.txt";
//const FILENAME: &str = "src/day10/day10_example.txt";
pub fn day10_p1() {
    println!("Day 10 Puzzle 1");

    //let mut linecount = 0;
    let mut score = 0;
    let mut autocomp_scores: Vec<u64> = Vec::new();
    if let Ok(lines) = util::read_lines(FILENAME) {
        for line in lines {
            let mut mystack: Vec<&str> = Vec::new();
            if let Ok(ip) = line {
                //linecount += 1;
                let row: Vec<&str> = ip.trim().split("").filter(|x| !x.is_empty()).collect();

                let mut syn_err: bool = false;

                for l in row {
                    match l {
                        "(" | "[" | "{" | "<" => mystack.push(l),
                        ")" => {
                            if mystack.pop().unwrap() != "(" {
                                score += 3;
                                syn_err = true;
                                break;
                            }
                        }
                        "]" => {
                            if mystack.pop().unwrap() != "[" {
                                score += 57;
                                syn_err = true;
                                break;
                            }
                        }
                        "}" => {
                            if mystack.pop().unwrap() != "{" {
                                score += 1197;
                                syn_err = true;
                                break;
                            }
                        }
                        ">" => {
                            if mystack.pop().unwrap() != "<" {
                                score += 25137;
                                syn_err = true;
                                break;
                            }
                        }
                        _ => panic!("unknown symbol {}", l),
                    }
                }

                if syn_err == false {
                    let mut comp_sum:u64 = 0;
                    while let Some(v) = mystack.pop() {
                        //println!("  v={}", v);
                        println!("v={}, comp_sum={}", v, comp_sum);
                        comp_sum *= 5;
                        comp_sum += match v {
                            "(" => 1,
                            "[" => 2,
                            "{" => 3,
                            "<" => 4,
                            _ => panic!("unknwon char")
                        };
                    }
                    println!("comp_sum = {}", comp_sum);
                    autocomp_scores.push(comp_sum);
                }
            }
        }
        println!("myscore={}", score);
        autocomp_scores.sort();
        dbg!(autocomp_scores.clone());
        println!("middle autocomp_score = {}", autocomp_scores[(autocomp_scores.len()-1)/2]);
    }
}
