use crate::*;
use std::collections::HashMap;
use regex::Regex;
use num::{Integer,NumCast};

const FILENAME: &str = "src/day18/day18_input.txt";
//const FILENAME: &str = "src/day18/day18_example.txt";

// Day 18: I'm tempted to try to hack something together
// without a full tree implementation, but hey we're in
// it to learn, to BinTree it is.
#[derive(Debug)]
enum BinTreeNode<T: num::Integer + num::NumCast> {
    Leaf(T),
    Branch {
        left: Box<BinTreeNode<T>>,
        right: Box<BinTreeNode<T>>,
    },
}

impl <T: num::Integer + num::NumCast> BinTreeNode<T> {
    fn magnitude(self) -> T {
        match self {
            Self::Leaf(t) => t,
            Self::Branch { left, right } => left.magnitude() * (T::one() + T::one() + T::one()) + right.magnitude() * (T::one() + T::one()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_left() {
        let x:BinTreeNode<i64> = BinTreeNode::Leaf(9);
        let y:BinTreeNode<i64> = BinTreeNode::Leaf(1);
        let z:BinTreeNode<i64> = BinTreeNode::Branch{ left:Box::new(x), right:Box::new(y) };
        println!("Node {:?}", z);
        assert_eq!(z.magnitude(), 29);
    }

    #[test]
    fn test_parse_snailfish() {
        let q = parse_snailfish::<u64>("[9,1]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 29);

        let q = parse_snailfish::<u64>("[1,9]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 21);

        let q = parse_snailfish::<u64>("[[9,1],[1,9]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 129);

        let q = parse_snailfish::<u64>("[[1,2],[[3,4],5]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 143);

        let q = parse_snailfish::<u64>("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 1384);
        
        let q = parse_snailfish::<u64>("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 445);

        let q = parse_snailfish::<u64>("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(dbg!(q.magnitude()), 791);

        let q = parse_snailfish::<u64>("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(q.magnitude(), 1137);

        let q = parse_snailfish::<u64>("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        println!("calculating magnitude of {:?}", q);
        assert_eq!(dbg!(q.magnitude()), 3488);

    }
}

// I don't want to write a full lexer/parser... but maybe I have to.
// A snailfish number is [(.*),(.*)], where .* can be either
//  a number or a snailfish number
fn parse_snailfish<T: num::Integer + num::NumCast>(x: &str) -> BinTreeNode<T> {
    println!("Parsing Snailfish {}", x);

    //let x=T::mul(T::one(), 77);
    let re_pair = Regex::new(r"^\[(.+),(.+)\]$").unwrap();
    let re_digits = Regex::new(r"^(\d+)$").unwrap();

    let mut pos:usize = 0;
    let c:Vec<char> = x.chars().collect();
    parse_snailfish_pos::<T>(&c, &mut pos)

    //BinTreeNode::Leaf(num::NumCast::from(77).unwrap())
}

fn parse_snailfish_pos<T: num::Integer + num::NumCast>(x: &[char], pos: &mut usize) -> BinTreeNode<T> {

    //let mut chars x[*pos..].chars();

    //let n = chars.next();
    //let n = x.get();
    
    match x[*pos] {
        '[' => {
            println!("open bracket");
            *pos +=1;
            let left:BinTreeNode<T> = parse_snailfish_pos(x,pos);
            *pos+=1;
            let right:BinTreeNode<T> = parse_snailfish_pos(x,pos);
            *pos+=1;
            return BinTreeNode::Branch{ left:Box::new(left), right:Box::new(right) };
        },
        _ => {
            println!("number..."); 
            let mut val:i64 = 0;

            if x[*pos].is_digit(10) {
                while x[*pos].is_digit(10) {
                    val *= 10;
                    val += x[*pos].to_digit(10).unwrap() as i64;
                    *pos += 1;
                }
                println!("Found number {}", val);
            } else {
                panic!("wtf!");
            }
            return BinTreeNode::Leaf(num::NumCast::from(val).unwrap());

        }
    }

    //println!("What oh...");
    //BinTreeNode::Leaf(num::NumCast::from(77).unwrap())
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

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut rules: HashMap<(String, String), String> = HashMap::new();
        let mut polymer:Vec<String> = Vec::new();
        let re_insertion_rules = Regex::new(r"^([A-Z])([A-Z]) -> ([A-Z])$").unwrap();
        let re_polymer_template = Regex::new(r"^([A-Z]+)$").unwrap();
        let mut pair_counts: HashMap<(String, String), i64> = HashMap::new();

    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}