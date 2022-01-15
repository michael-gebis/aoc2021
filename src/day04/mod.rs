extern crate termion;
use std::fmt;
use termion::{color, style};

// TODO: create board object, a vec of vecs containing i32
// TODO: method check_win()
// TODO: method mark(drawn_number)
// TODO: method calculate_score()

// Read file, which creates:
// Vec of draws
// Vec of Boards

const GRIDSIZE: usize = 5;


#[derive(Debug)]
enum CellState {
    Unpicked,
    Picked,
}

#[derive(Debug)]
struct Cell {
    val: u32,
    state: CellState,
}

impl Cell {
    fn new(v:u32) -> Cell {
        Cell { val:v, state:CellState::Unpicked, }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.state {
            CellState::Picked => write!(f, "{}{:3}{}", color::Fg(color::Red), self.val, style::Reset),
            _ => write!(f, "{:3}", self.val),
        }
    }
}

struct Board {
    data: Vec<Vec<Cell>>,
}

impl Board {    
    fn new() -> Board {
//        data : Vec::new();
        let mut v = Vec::with_capacity(GRIDSIZE);
        println!("New");
        let mut c = 0;
        for _i in 0..GRIDSIZE {
            let mut t: Vec<Cell> = Vec::with_capacity(GRIDSIZE);
            for _j in 0..GRIDSIZE {
                t.push(Cell::new(c));
                c += 1;
            }
            v.push(t);
        }
        Board{ data: v }
    }

    /*
    fn set_picked(self:&mut Board, i:usize, j:usize) {
        self.data[i][j].state = CellState::Picked;
    }
    */

    fn pick(self:&mut Board, v:u32) {
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                if self.data[i][j].val == v {
                    self.data[i][j].state = CellState::Picked;
                }
            }
        }
    }

    fn check_win(self:&Board) -> bool {
        // Check rows
        'rowloop: for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                //println!("pre i,j: {},{}", i,j);
                match self.data[i][j].state {
                    CellState::Unpicked => continue 'rowloop,
                    _ => (),
                }
                //println!("pos i,j: {},{}", i,j);

            }
            //println!("Win in row {}", i);
            return true;
        }
        // check cols
        'colloop: for j in 0..GRIDSIZE {
            for i in 0..GRIDSIZE {
                match self.data[i][j].state {
                    CellState::Unpicked => continue 'colloop,
                    _ => (),
                };
            }
            println!("Win in col {}", j);
            return true;                

        }
        println!("Loser");
        false
    }

    fn sum_of_unpicked(&self) -> u32 {
        let mut sum: u32 = 0;
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                match self.data[i][j].state {
                    CellState::Unpicked => sum += self.data[i][j].val,
                    _ => (),
                };
            }
        }
        sum
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..GRIDSIZE {
            for j in 0..GRIDSIZE {
                write!(f, "{} ", self.data[i][j])?;
            }
            write!(f,"\n")?;
        }
        fmt::Result::Ok(())
    }
}

pub fn day04_p1() {
    let mut b = Board::new();

    println!("{}",b);
    b.pick(5);
    b.pick(6);
    b.pick(7);
    b.pick(8);
    println!("{}",b);
    b.check_win();


    b.pick(9);
    println!("{}",b);
    b.check_win();
    println!("Score={}",b.sum_of_unpicked());


    let mut b = Board::new();
    b.pick(2);
    b.pick(7);
    b.pick(12);
    b.pick(17);
    println!("{}",b);
    b.check_win();

    b.pick(22);
    println!("{}",b);
    b.check_win();
    println!("Score={}",b.sum_of_unpicked());
    
    // TODO: create board object, a vec of vecs containing i32
    // TODO: method calculate_score()

    // Read file, which creates:
    // Vec of draws
    // Vec of Boards
}