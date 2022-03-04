use crate::*;
use regex::Regex;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

//const FILENAME: &str = "src/day22/day22_input.txt";
const FILENAME: &str = "src/day22/day22_exampleA.txt";
//const FILENAME: &str = "src/day22/day22_exampleB.txt";

/*
Imagine the 2D squares:
    A: x=2..4, y=11..14
    B: x=3..6, y=13..15

    In the diagram, X represents where the two squares overlap.

    1   2   3   4   5   6   7
10
11      A   A   A
12      A   A   A
13      A   X   X   B   B
14      A   X   X   B   B
15          B   B   B   B
16

The trick is to slice the squares.  First, the X-axis

    1   2   3   4   5   6   7
10
11      A1  A2  A2
12      A1  A2  A2
13      A1  XX  XX  B2  B2
14      A1  XX  XX  B2  B2
15          B1  B1  B2  B2
16

A1: x=2..2,y=11..14 ( Sliced off, no intersections)
A2: x=3..4,y=11..14 ( Still intersects B1)
B1: x=3..4,y=13..15 ( Still intersects B2)
B2: x=5..6,y=13..14 ( Sliced off, no intersections)

Note that A1 and B2 don't overlap with anything.  A2 and B1 still
overlap and need to be sliced along the Y-axis.

        1   2   3   4   5   6   7
10
11      A1  A2  A2
12      A1  A2  A2
13      A1  XX  XX  B2  B2
14      A1  XX  XX  B2  B2
15          B1  B1  B2  B2
16

A1: x=2..2,y=11..14
A2: x=3..4,y=11..12
XX: x-2..3,y=13..14 (This is 100% overlap)
B1: x=3..4,y=15..15
B2: x=5..6,y=13..14

Thus the two squares have now been divided into 5 non-overlapping squares.

*/

#[derive(Debug, PartialEq, Eq, Hash)]
struct CubeRange {
    start: i64,
    end: i64,
}

//impl Eq for CubeRange {
//    fn eq(&self, other:&Self) -> bool {
//        self.start == other.start && self.end == other.end
//    }
//}

impl CubeRange {
    pub fn new(start: i64, end: i64) -> CubeRange {
        if end < start {
            panic!("end < start!!!");
            // Should we auto-swap?  For now, no.
            // CubeRange {start:end, end:start}
        } else {
            CubeRange {
                start: start,
                end: end,
            }
        }
    }

    pub fn width(&self) -> i64 {
        self.end - self.start
    }

    pub fn intersect(&self, other: &CubeRange) -> Option<CubeRange> {
        let left;
        let right;

        if other.start < self.start {
            left = other.clone();
            right = self.clone();
        } else {
            left = self.clone();
            right = other.clone();
        }

        // Non-overlap
        if left.end < right.start {
            //println!("None!");
            return None;
        }

        // Complete overlap
        if left.end >= right.end {
            //println!("Complete!");
            return Some(CubeRange::new(right.start, right.end));
        }

        // Partial overlap
        //println!("Partial!");
        return Some(CubeRange::new(right.start, left.end));
    }

    pub fn subtract(&self, other: &CubeRange) -> (Option<CubeRange>, Option<CubeRange>) {
        // Don't think I can do the swap trick due to the asymmetry of the operation.
        if self.start < other.start {
            // Non-overlap
            if self.end < other.start {
                return (None, None);
            }

            // Complete overlap with slop on both sides
            if self.end > other.end {
                return (
                    Some(CubeRange::new(self.start, other.start - 1)),
                    Some(CubeRange::new(other.end + 1, self.end)),
                );
            }

            // Complete overlap with slop on left but aligned right edges
            if self.end == other.end {
                return (Some(CubeRange::new(self.start, other.start - 1)), None);
            }

            // Partial overlap (I guess this is same as above)
            return (Some(CubeRange::new(self.start, other.start - 1)), None);
        } else if other.start <= self.start {
            // Non-overlap
            if other.end < self.start {
                return (None, None);
            }

            // Complete overlap with
            if other.end >= self.end {
                return (None, None);
            }

            // Partial overlap
            return (Some(CubeRange::new(other.end + 1, self.end)), None);
        }

        panic!("Should nae happen");
    }
}

impl fmt::Display for CubeRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

// Some goddamned tests
#[cfg(test)]
mod tests {
    use crate::day22::CubeRange;

    #[test]
    fn test_range_intersect_lr() {
        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(30, 40);
        let z = x.intersect(&y);
        assert_eq!(z, None);

        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(13, 17);
        let z = x.intersect(&y);
        assert_eq!(z, Some(CubeRange::new(13, 17)));

        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(17, 30);
        let z = x.intersect(&y);
        assert_eq!(z, Some(CubeRange::new(17, 20)));

        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(10, 15);
        let z = x.intersect(&y);
        assert_eq!(z, Some(CubeRange::new(10, 15)));

        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(15, 20);
        let z = x.intersect(&y);
        assert_eq!(z, Some(CubeRange::new(15, 20)));
    }

    #[test]
    fn test_range_intersect_rl() {
        // reversed left-to-right
        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(30, 40);
        let z = x.intersect(&y);
        assert_eq!(z, None);

        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(13, 17);
        let z = x.intersect(&y);
        assert_eq!(z, Some(CubeRange::new(13, 17)));

        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(17, 30);
        let z = x.intersect(&y);
        assert_eq!(z, Some(CubeRange::new(17, 20)));

        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(10, 15);
        let z = x.intersect(&y);
        assert_eq!(z, Some(CubeRange::new(10, 15)));

        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(15, 20);
        let z = x.intersect(&y);
        assert_eq!(z, Some(CubeRange::new(15, 20)));
    }

    #[test]
    fn test_range_subtract_lr() {
        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(30, 40);
        let z = x.subtract(&y);
        assert_eq!(z, (None, None));

        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(13, 17);
        let z = x.subtract(&y);
        assert_eq!(
            z,
            (Some(CubeRange::new(10, 12)), Some(CubeRange::new(18, 20)))
        );

        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(17, 30);
        let z = x.subtract(&y);
        assert_eq!(z, (Some(CubeRange::new(10, 16)), None));

        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(10, 15);
        let z = x.subtract(&y);
        assert_eq!(z, (Some(CubeRange::new(16, 20)), None));

        let x = CubeRange::new(10, 20);
        let y = CubeRange::new(15, 20);
        let z = x.subtract(&y);
        assert_eq!(z, (Some(CubeRange::new(10, 14)), None));
    }

    #[test]
    fn test_range_subtract_rl() {
        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(30, 40);
        let z = x.subtract(&y);
        assert_eq!(z, (None, None));

        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(13, 17);
        let z = x.subtract(&y);
        assert_eq!(z, (None, None));

        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(17, 30);
        let z = x.subtract(&y);
        assert_eq!(z, (Some(CubeRange::new(21, 30)), None));

        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(10, 15);
        let z = x.subtract(&y);
        assert_eq!(z, (None, None));

        let y = CubeRange::new(10, 20);
        let x = CubeRange::new(15, 20);
        let z = x.subtract(&y);
        assert_eq!(z, (None, None));
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum CubeType {
    On,
    Off,
}

impl fmt::Display for CubeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if *self == CubeType::On { "On" } else { "Off" })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cube {
    xrange: CubeRange,
    yrange: CubeRange,
    zrange: CubeRange,
    cubetype: CubeType,
}

impl Cube {
    pub fn new(
        ct: CubeType,
        xstart: i64,
        xend: i64,
        ystart: i64,
        yend: i64,
        zstart: i64,
        zend: i64,
    ) -> Cube {
        Cube {
            xrange: CubeRange::new(xstart, xend),
            yrange: CubeRange::new(ystart, yend),
            zrange: CubeRange::new(zstart, zend),
            cubetype: ct,
        }
    }

    fn volume(&self) -> i64 {
        self.xrange.width() * self.yrange.width() * self.zrange.width()
    }

    fn check_intersect(&self, other: &Cube) -> bool {
        self.xrange.intersect(&other.xrange) != None
            && self.yrange.intersect(&other.yrange) != None
            && self.zrange.intersect(&other.zrange) != None
    }

    // Slice?
}
impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Cube ({}): {} {} {}",
            self.cubetype, self.xrange, self.yrange, self.zrange
        )
    }
}

//impl PartialEq for Cube {
//    pub fn eq(&self, other:&Self) -> bool {
//        self.xrange == other.xrange && self.yrange==other.yrange && self.zrange==other.zrange && self.cubetype == other.cubetype
//    }
//}

#[allow(dead_code)]
pub fn day22_p1() {
    println!("Day 22 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        // Looks like: off x=-54112..-39298,y=-85059..-49293,z=-27449..7877
        let re_cube =
            Regex::new(r"(o.+) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
                .unwrap();
        let mut pending_cubes: VecDeque<Cube> = VecDeque::new();

        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_cube.captures(&ip) {
                    //println!("cap:{:?}",cap);
                    let cube = Cube::new(
                        if &cap[1] == "on" {
                            CubeType::On
                        } else {
                            CubeType::Off
                        },
                        cap[2].parse::<i64>().unwrap(),
                        cap[3].parse::<i64>().unwrap(),
                        cap[4].parse::<i64>().unwrap(),
                        cap[5].parse::<i64>().unwrap(),
                        cap[6].parse::<i64>().unwrap(),
                        cap[7].parse::<i64>().unwrap(),
                    );
                    println!("Found cube: {}", cube);
                    pending_cubes.push_back(cube);
                } else {
                    println!("Ignoring line '{}'", ip);
                }
            }
        }

        let mut completed_cubes: HashSet<Cube> = HashSet::new();
        let mut tmp_cube: Cube;

        'a: while !pending_cubes.is_empty() {
            tmp_cube = pending_cubes.pop_front().unwrap();
            //let mut tmp_completed: VecDeque<Cube> = VecDeque::new();
            for c in completed_cubes.iter() {
                if tmp_cube.check_intersect(&c) {
                    println!("Adding intersecting cube");
                    println!("  new cube {}", &tmp_cube);
                    println!("  existing {}", &c);
                    // Slice off non-intersecting bits on the x-axis:

                    let (x_left, x_right) = c.xrange.subtract(&tmp_cube.xrange);

                    match x_left {
                        Some(r) => {
                            let pc = Cube::new(
                                CubeType::On,
                                r.start,
                                r.end,
                                c.yrange.start,
                                c.yrange.end,
                                c.zrange.start,
                                c.zrange.end,
                            );
                            println!("  sliced off from existing {}", &pc);
                            pending_cubes.push_back(pc);
                        }
                        _ => (),
                    }

                    match x_right {
                        Some(r) => {
                            let pc = Cube::new(
                                CubeType::On,
                                r.start,
                                r.end,
                                c.yrange.start,
                                c.yrange.end,
                                c.zrange.start,
                                c.zrange.end,
                            );
                            println!("  sliced off from existing {}", &pc);
                            pending_cubes.push_back(pc);
                        }
                        _ => (),
                    }

                    let (x_left, x_right) = tmp_cube.xrange.subtract(&c.xrange);
                    match x_left {
                        Some(r) => {
                            let pc = Cube::new(
                                CubeType::On,
                                r.start,
                                r.end,
                                tmp_cube.yrange.start,
                                tmp_cube.yrange.end,
                                tmp_cube.zrange.start,
                                tmp_cube.zrange.end,
                            );
                            println!("  sliced off from new {}", &pc);
                            pending_cubes.push_back(pc);
                        }
                        _ => (),
                    }

                    match x_right {
                        Some(r) => {
                            let pc = Cube::new(
                                CubeType::On,
                                r.start,
                                r.end,
                                tmp_cube.yrange.start,
                                tmp_cube.yrange.end,
                                tmp_cube.zrange.start,
                                tmp_cube.zrange.end,
                            );
                            println!("  sliced off from new {}", &pc);
                            pending_cubes.push_back(pc);
                        }
                        _ => (),
                    }

                    let r = c.xrange.intersect(&tmp_cube.xrange).unwrap();
                    let c2 = Cube::new(
                        CubeType::On,
                        r.start,
                        r.end,
                        c.yrange.start,
                        c.yrange.end,
                        c.zrange.start,
                        c.zrange.end,
                    );

                    let c3 = Cube::new(
                        CubeType::On,
                        r.start,
                        r.end,
                        tmp_cube.yrange.start,
                        tmp_cube.yrange.end,
                        tmp_cube.zrange.start,
                        tmp_cube.zrange.end,
                    );
                    //println!("intersecting bit {:?}", tmp_cube);
                    println!("  Remaining c2 {}", c2);
                    println!("  Remaining c3 {}", c3);

                    // Slice in Y direction
                    // Slice in Z direction


                    panic!("xxx");
                    continue 'a;
                }
            }

            println!("Adding non-intersecting cube {}", tmp_cube);
            completed_cubes.insert(tmp_cube);

            //for x in tmp_completed {
            //    completed_cubes.insert(x);
            //}
        }

        println!("{} cubes", completed_cubes.len());

        let mut vol: i64 = 0;
        for c in completed_cubes {
            vol += c.volume();
        }
        println!("Total volume: {}", vol);
    } else {
        panic!("Couldn't open file {}", FILENAME);
    }
}
