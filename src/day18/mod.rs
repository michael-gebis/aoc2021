use crate::*;
//use num::{Integer, NumCast};
//use regex::Regex;
//use std::collections::HashMap;

const FILENAME: &str = "src/day18/day18_input.txt";
//const FILENAME: &str = "src/day18/day18_example.txt";

// Day 18: I'm tempted to try to hack something together
// without a full tree implementation, but hey we're in
// it to learn, to BinTree it is.
#[allow(dead_code)]
#[derive(Debug)]
enum BinTreeNode {
    Leaf(i64),
    Branch {
        left: Box<BinTreeNode>,
        right: Box<BinTreeNode>,
    },
}

#[allow(dead_code)]
impl BinTreeNode {
    fn magnitude(self) -> i64 {
        match self {
            Self::Leaf(t) => t,
            Self::Branch { left, right } => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_left() {
        let x: BinTreeNode = BinTreeNode::Leaf(9);
        let y: BinTreeNode = BinTreeNode::Leaf(1);
        let z: BinTreeNode = BinTreeNode::Branch {
            left: Box::new(x),
            right: Box::new(y),
        };
        println!("Node {:?}", z);
        assert_eq!(z.magnitude(), 29);
    }

    #[test]
    fn test_parse_snailfish() {
        let q = parse_snailfish("[9,1]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 29);

        let q = parse_snailfish("[1,9]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 21);

        let q = parse_snailfish("[[9,1],[1,9]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 129);

        let q = parse_snailfish("[[1,2],[[3,4],5]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 143);

        let q = parse_snailfish("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 1384);
        let q = parse_snailfish("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 445);

        let q = parse_snailfish("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(dbg!(q.magnitude()), 791);

        let q = parse_snailfish("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 1137);

        let q = parse_snailfish("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(dbg!(q.magnitude()), 3488);
    }
}

// I don't want to write a full lexer/parser... but maybe I have to.
// A snailfish number is [(.*),(.*)], where .* can be either
//  a number or a snailfish number
#[allow(dead_code)]
fn parse_snailfish(x: &str) -> BinTreeNode {
    println!("Parsing Snailfish {}", x);

    let mut pos: usize = 0;
    let c: Vec<char> = x.chars().collect();
    parse_snailfish_pos(&c, &mut pos)
}

// If there is a syntax error, this parser calls panic!()
#[allow(dead_code)]
fn parse_snailfish_pos(x: &[char], pos: &mut usize) -> BinTreeNode {
    match x[*pos] {
        '[' => {
            //println!("open bracket");
            *pos += 1;
            let left: BinTreeNode = parse_snailfish_pos(x, pos);
            if x[*pos] != ',' {
                panic!("expected comma at pos {}", *pos);
            }
            *pos += 1;
            let right: BinTreeNode = parse_snailfish_pos(x, pos);
            if x[*pos] != ']' {
                panic!("expected close bracet at pos {}", *pos);
            }
            *pos += 1;
            return BinTreeNode::Branch {
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        _ => {
            let mut val: i64 = 0;

            if x[*pos].is_digit(10) {
                while x[*pos].is_digit(10) {
                    val *= 10;
                    val += x[*pos].to_digit(10).unwrap() as i64;
                    *pos += 1;
                }
            } else {
                panic!("expected digit at pos {}", *pos);
            }
            return BinTreeNode::Leaf(num::NumCast::from(val).unwrap());
        }
    }
}

// Some binary tree links:

// https://hackernoon.com/how-to-insert-binary-tree-in-rust
// https://codereview.stackexchange.com/questions/133209/binary-tree-implementation-in-rust
// https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
// https://levelup.gitconnected.com/rust-binary-tree-30efdd355b60
// https://medium.com/swlh/rust-binary-tree-a-refactor-1b090a88e24

#[allow(dead_code)]
pub fn day18_p1() {
    println!("Day 18 Puzzle 2");

    if let Ok(_lines) = util::read_lines(FILENAME) {
        /* 
        let mut rules: HashMap<(String, String), String> = HashMap::new();
        let mut polymer: Vec<String> = Vec::new();
        let re_insertion_rules = Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();
        let re_polymer_template = Regex::new(r"^([A-Z]+)$").unwrap();
        let mut pair_counts: HashMap<(String, String), i64> = HashMap::new();
        */
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}
