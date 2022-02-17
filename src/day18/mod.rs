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
}

// I don't want to write a full lexer/parser... but maybe I have to.
// A snailfish number is [(.*),(.*)], where .* can be either
//  a number or a snailfish number
fn parse_snailfish<T: num::Integer + num::NumCast>(x: &str) -> BinTreeNode<T> {
    println!("Parsing Snailfish {}", x);

    //let x=T::mul(T::one(), 77);
    let re_pair = Regex::new(r"^\[(.+),(.+)\]$").unwrap();
    let re_digits = Regex::new(r"^(\d+)$").unwrap();

    BinTreeNode::Leaf(num::NumCast::from(77).unwrap())
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