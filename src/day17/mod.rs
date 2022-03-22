//use crate::*;

//const FILENAME: &str = "src/day17/day17_input.txt";
//const FILENAME: &str = "src/day17/day17_example.txt";

// minx, maxx, miny, maxy
struct TargetRange(i64, i64, i64, i64);

// Example
//const RANGE: TargetRange = TargetRange(20, 30,-10,-5);
// Input
const RANGE: TargetRange = TargetRange(150, 171, -129, -70);

#[allow(dead_code)]
pub fn day17_p1() {
    println!("Day 17 Puzzle 1");

    let mut ymaxmax = i64::MIN;
    let mut hitcount = 0;
    let mut speeds: Vec<(i64, i64)> = Vec::new();

    for xinitvelocity in 1..(RANGE.1 + 1) {
        for yinitvelocity in (RANGE.2)..(RANGE.2.abs() + 1) {
            let mut xvel = xinitvelocity;
            let mut yvel = yinitvelocity;
            let (mut x, mut y): (i64, i64) = (0, 0);
            let mut ymax = y;
            //let mut hit: bool = false;

            println!("xinit,yinit]={},{}", xinitvelocity, yinitvelocity);
            while x <= RANGE.1 && y >= RANGE.2 {
                println!("  x,y={},{} xvel,yvel={},{}", x, y, xvel, yvel);
                x += xvel;
                y += yvel;

                if y > ymax {
                    ymax = y
                }
                //if ymax > ymaxmax { ymaxmax = ymax }
                if xvel > 0 {
                    xvel -= 1;
                }
                yvel -= 1;
                if x >= RANGE.0 && x <= RANGE.1 && y >= RANGE.2 && y <= RANGE.3 {
                    println!("  HIT!!!");
                    //hit = true;
                    hitcount += 1;
                    if ymax > ymaxmax {
                        ymaxmax = ymax
                    }
                    speeds.push((xinitvelocity, yinitvelocity));
                    break;
                }
            }
            println!("  next: x,y={},{} xvel,yvel={},{}", x, y, xvel, yvel);

            println!("");
        }
    }

    println!("ymaxmax={}", ymaxmax);
    println!("hitcount={}", hitcount);
    println!("{:?}", speeds);
}
