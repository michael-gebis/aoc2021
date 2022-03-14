use crate::*;
use std::fmt;

//const FILENAME: &str = "src/day25/day25_example.txt";
const FILENAME: &str = "src/day25/day25_input.txt";

#[derive(Copy, Clone, PartialEq)]
enum LocationState {
    Empty,
    East,
    South,
}

impl fmt::Display for LocationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match &self {
            LocationState::Empty => ".",
            LocationState::East => ">",
            LocationState::South => "v",
        };
        write!(f, "{}", d)
    }
}

#[allow(dead_code)]
fn printcukes(cukes: &Vec<Vec<LocationState>>) {
    for row in cukes {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}

fn region_iterate(rowcount: usize, colcount: usize, cukes: Vec<Vec<LocationState>>) -> usize {
    let mut curcukes = cukes.clone();
    let mut steps = 0;

    loop {
        let mut moves = 0;
        steps += 1;

        let mut nextcukes = vec![vec![LocationState::Empty; colcount]; rowcount];

        for (i, rowdata) in curcukes.iter().enumerate() {
            for (j, coldata) in rowdata.iter().enumerate() {
                if *coldata == LocationState::East {
                    let targetj = (j + 1) % colcount;
                    let targeti = i;

                    if curcukes[targeti][targetj] == LocationState::Empty {
                        nextcukes[targeti][targetj] = LocationState::East;
                        moves += 1;
                    } else {
                        nextcukes[i][j] = curcukes[i][j];
                    }
                } else if *coldata == LocationState::South {
                    nextcukes[i][j] = curcukes[i][j];
                }
            }
        }

        curcukes = nextcukes;

        let mut nextcukes = vec![vec![LocationState::Empty; colcount]; rowcount];

        for (i, rowdata) in curcukes.iter().enumerate() {
            for (j, coldata) in rowdata.iter().enumerate() {
                if *coldata == LocationState::South {
                    let targetj = j;
                    let targeti = (i + 1) % rowcount;

                    if curcukes[targeti][targetj] == LocationState::Empty {
                        nextcukes[targeti][targetj] = LocationState::South;
                        moves += 1;
                    } else {
                        nextcukes[i][j] = curcukes[i][j];
                    }
                } else if *coldata == LocationState::East {
                    nextcukes[i][j] = curcukes[i][j];
                }
            }
        }
        curcukes = nextcukes;
        println!("Step {}", steps);
        println!("Moves {}", moves);
        if moves == 0 {
            break;
        }
        //printcukes(&curcukes);
    }

    steps
}

pub fn day25_p1() {
    println!("Day 25 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut cukes: Vec<Vec<LocationState>> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                let cukeline: Vec<LocationState> = ip
                    .split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| match x {
                        "." => LocationState::Empty,
                        ">" => LocationState::East,
                        "v" => LocationState::South,
                        _ => panic!("Unknown char"),
                    })
                    .collect();
                cukes.push(cukeline);
            }
        }

        let rowcount = cukes.len();
        let colcount = cukes[0].len();

        region_iterate(rowcount, colcount, cukes);
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}
