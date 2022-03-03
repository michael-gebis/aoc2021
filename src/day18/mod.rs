use crate::*;
use std::fmt;

const FILENAME: &str = "src/day18/day18_input.txt";
//const FILENAME: &str = "src/day18/day18_example.txt";

// Day 18: I'm tempted to try to hack something together
// without a full tree implementation, but hey we're in
// it to learn, to BinTree it is.
#[derive(Debug, PartialEq, Clone)]
enum BinTreeNode {
    Leaf(i64),
    Branch {
        left: Box<BinTreeNode>,
        right: Box<BinTreeNode>,
    },
}

impl BinTreeNode {
    /// magnitude() returns the single i64 magnitude value
    /// as defined in the README. Pop-pop!
    fn magnitude(self) -> i64 {
        match self {
            Self::Leaf(t) => t,
            Self::Branch { left, right } => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }

    /// Add two trees, but doesn't reduce the result
    /// Useful for testing and debug
    fn add_noreduce(l: &BinTreeNode, r: &BinTreeNode) -> BinTreeNode {
        return BinTreeNode::Branch {
            left: Box::new(l.clone()),
            right: Box::new(r.clone()),
        };
    }

    /// Add two trees and reduce the result
    fn add(l: &BinTreeNode, r: &BinTreeNode) -> BinTreeNode {
        let mut tmp = BinTreeNode::add_noreduce(l, r);
        BinTreeNode::reduce(&mut tmp);
        return tmp;
    }

    // Descend the tree, looking for any Leaf with a value
    // of 10 or more.  If so, split it, and return true.

    // If no nodes are found, return false.
    
    // At most, a single split will happen.
    fn split(n: &mut BinTreeNode) -> bool {
        match n {
            BinTreeNode::Leaf(t) => {
                if *t >= 10 {
                    let lval = *t / 2; // round down
                    let rval = (*t + 1) / 2; // round up
                    *n = BinTreeNode::Branch {
                        left: Box::new(BinTreeNode::Leaf(lval)),
                        right: Box::new(BinTreeNode::Leaf(rval)),
                    };
                    // println!("  DID split!");
                    return true;
                }
                false
            }
            Self::Branch { left, right } => BinTreeNode::split(left) || BinTreeNode::split(right),
        }
    }

    fn addto_left(root: Option<&mut BinTreeNode>, val: &mut BinTreeNode) {
        // println!("addto_left");
        match root {
            None => {
                //println!("  left was none;  to do");
            }
            Some(mut node) => {
                while let BinTreeNode::Branch { left: _, right } = node {
                    node = right;
                }
                if let BinTreeNode::Leaf(x) = node {
                    // println!("adding to {}", x);
                    if let BinTreeNode::Leaf(q) = val {
                        *node = BinTreeNode::Leaf(*q + *x);
                    }
                }
            }
        }
    }

    fn addto_right(root: Option<&mut BinTreeNode>, val: &BinTreeNode) {
        // println!("addto_right");
        match root {
            None => {
                // println!("  right was none; Nothing to do");
            }
            Some(mut node) => {
                while let BinTreeNode::Branch { left, right: _ } = node {
                    node = left;
                }
                if let BinTreeNode::Leaf(x) = node {
                    // println!("adding to {}", x);
                    if let BinTreeNode::Leaf(q) = val {
                        *node = BinTreeNode::Leaf(*q + *x);
                    }
                }
            }
        }
    }

    fn assplode(
        n: &mut BinTreeNode,
        lbud: Option<&mut BinTreeNode>,
        rbud: Option<&mut BinTreeNode>,
        depth: i64,
    ) -> bool {
        // println!("assploding at depth={}", depth);
        match n {
            Self::Leaf(_val) => {
                // println!("Found leaf val={}", val);
                return false;
            }
            Self::Branch { left, right } => {
                if depth >= 4 {
                    // println!("BOOOM");
                    Self::addto_left(lbud, left);
                    Self::addto_right(rbud, right);
                    *n = BinTreeNode::Leaf(0);
                    return true;
                }
                return Self::assplode(left, lbud, Some(right), depth + 1)
                    || Self::assplode(right, Some(left), rbud, depth + 1);
            }
        }
    }

    fn explode(n: &mut BinTreeNode) -> bool {
        BinTreeNode::assplode(n, None, None, 0)
    }

    fn reduce(n: &mut BinTreeNode) {
        while BinTreeNode::explode(n) || BinTreeNode::split(n) {
            //println!("reducing");
        }
    }
}

// I just think recursion is prety neat.
impl fmt::Display for BinTreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Leaf(val) => write!(f, "{}", val),
            Self::Branch { left, right } => write!(f, "[{},{}]", left, right),
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
        println!("calculating magnitude of {}", q);
        assert_eq!(q.magnitude(), 29);

        let q = parse_snailfish("[1,9]");
        println!("calculating magnitude of {}", q);
        assert_eq!(q.magnitude(), 21);

        let q = parse_snailfish("[[9,1],[1,9]]");
        println!("calculating magnitude of {}", q);
        assert_eq!(q.magnitude(), 129);

        let q = parse_snailfish("[[1,2],[[3,4],5]]");
        println!("calculating magnitude of {}", q);
        assert_eq!(q.magnitude(), 143);

        let q = parse_snailfish("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        println!("calculating magnitude of {}", q);
        assert_eq!(q.magnitude(), 1384);

        let q = parse_snailfish("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        println!("calculating magnitude of {}", q);
        assert_eq!(q.magnitude(), 445);

        let q = parse_snailfish("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        println!("calculating magnitude of {}", q);
        assert_eq!(dbg!(q.magnitude()), 791);

        let q = parse_snailfish("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        println!("calculating magnitude of {}", q);
        assert_eq!(q.magnitude(), 1137);

        let q = parse_snailfish("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        println!("calculating magnitude of {}", q);
        assert_eq!(dbg!(q.magnitude()), 3488);
    }

    #[test]
    fn test_split() {
        let q = &mut parse_snailfish("[10,1]");
        BinTreeNode::split(q);
        let expected = parse_snailfish("[[5,5],1]");
        println!("split={}", q);
        assert_eq!(expected, *q);

        let q = &mut parse_snailfish("[1,11]");
        BinTreeNode::split(q);
        let expected = parse_snailfish("[1,[5,6]]");
        println!("split={}", q);
        assert_eq!(expected, *q);

        // Only one split is done at a time; this
        // snailfish needs two splits
        let q = &mut parse_snailfish("[12,15]");
        while BinTreeNode::split(q) {
            println!("doing split...");
        }
        println!("split={}", q);
    }

    #[test]
    fn test_explode() {
        let q = &mut parse_snailfish("[[[[[9,8],1],2],3],4]");
        BinTreeNode::explode(q);
        println!("explode={}", q);

        let q = &mut parse_snailfish("[7,[6,[5,[4,[3,2]]]]]");
        BinTreeNode::explode(q);
        println!("explode={}", q);

        let q = &mut parse_snailfish("[[6,[5,[4,[3,2]]]],1]");
        BinTreeNode::explode(q);
        println!("explode={}", q);

        let q = &mut parse_snailfish("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        BinTreeNode::explode(q);
        println!("explode={}", q);

        let q = &mut parse_snailfish("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        BinTreeNode::explode(q);
        println!("explode={}", q);
    }

    #[test]
    fn test_reduce() {
        let q = &mut parse_snailfish("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        BinTreeNode::reduce(q);
        println!("reduce={}", q);
    }

    #[test]
    fn test_add_noreduce() {
        let l = parse_snailfish("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let r = parse_snailfish("[1,1]");
        let sum = &mut BinTreeNode::add_noreduce(&l, &r);

        let expected = &mut parse_snailfish("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        assert_eq!(expected, sum);
    }

    #[test]
    fn test_add() {
        let l = parse_snailfish("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let r = parse_snailfish("[1,1]");
        let sum = &mut BinTreeNode::add(&l, &r);

        let expected = &mut parse_snailfish("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(expected, sum);
        /*
        let mut split_count = 0;
        while BinTreeNode::split(sum) {
            println!("Did a split; trying again...");
            split_count += 1;
        }
        println!("split={}",sum);
        assert_eq!(2,split_count);
        */
    }
}

// I don't want to write a full lexer/parser... but maybe I have to.
// A snailfish number is [(.*),(.*)], where .* can be either
//  a number or a snailfish number
fn parse_snailfish(x: &str) -> BinTreeNode {
    println!("Parsing Snailfish {}", x);

    let mut pos: usize = 0;
    let tokens: Vec<char> = x.chars().collect();
    parse_snailfish_pos(&tokens, &mut pos)
}

// If there is a syntax error, this parser calls panic!()
fn parse_snailfish_pos(tokens: &[char], pos: &mut usize) -> BinTreeNode {
    match tokens[*pos] {
        '[' => {
            //println!("open bracket");
            *pos += 1;
            let left: BinTreeNode = parse_snailfish_pos(tokens, pos);
            if tokens[*pos] != ',' {
                panic!("expected comma at pos {}", *pos);
            }
            *pos += 1;
            let right: BinTreeNode = parse_snailfish_pos(tokens, pos);
            if tokens[*pos] != ']' {
                panic!("expected close bracket at pos {}", *pos);
            }
            *pos += 1;
            return BinTreeNode::Branch {
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        _ => {
            let mut val: i64 = 0;

            if tokens[*pos].is_digit(10) {
                // In theory, there shouldn't be a multi digit value,
                // since the "reduce()" function should have split any
                // values greater than 10, and the input files should
                // contain snailfish numbers that are already reduced.
                // But just in case (and for testing) handle it anyways.
                while tokens[*pos].is_digit(10) {
                    val *= 10;
                    val += tokens[*pos].to_digit(10).unwrap() as i64;
                    *pos += 1;
                }
            } else {
                panic!("expected digit at pos {}", *pos);
            }
            return BinTreeNode::Leaf(val);
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
    println!("Day 18 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut total: Option<BinTreeNode> = None;
        for line in lines {
            if let Ok(ip) = line {
                let snailfish = parse_snailfish(&ip);
                if let Some(sf) = total {
                    total = Some(BinTreeNode::add(&sf, &snailfish));
                } else {
                    total = Some(snailfish);
                }
            }
        }
        println!("Final magnitude={}", BinTreeNode::magnitude(total.unwrap()));
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}

#[allow(dead_code)]
pub fn day18_p2() {
    println!("Day 18 Puzzle 2");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut all_snailfish: Vec<BinTreeNode> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                all_snailfish.push(parse_snailfish(&ip));
            }
        }

        let mut max_sum = i64::MIN;
        // O(n^2) as we have to test all pairs, and the
        // computation is not commutative
        for i in &all_snailfish {
            for j in &all_snailfish {
                // Problem says "all different pairs"
                if *i == *j {
                    continue;
                }
                max_sum = i64::max(max_sum, BinTreeNode::magnitude(BinTreeNode::add(i, j)));
            }
        }
        println!("Biggest magnitude = {}", max_sum);
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}
