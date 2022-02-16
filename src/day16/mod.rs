use crate::*;

const FILENAME: &str = "src/day16/day16_input.txt";
//const FILENAME: &str = "src/day16/day16_exampleA.txt";
//const FILENAME: &str = "src/day16/day16_exampleB.txt";

// returns bitcount, version sum, value
fn parse_packet(indent: usize, d: &[u8]) -> (usize, u32, u64) {
    let ibuf = (0..indent).map(|_| "    ").collect::<String>();

    println!("{}d.len() = {}", ibuf, d.len());

    let mut bitcount: usize = 0;
    let mut versum: u32 = 0;
    //let mut value:u32 = 0;

    // read version #:
    let ver = d[0] << 2 | d[1] << 1 | d[2];
    let type_id = d[3] << 2 | d[4] << 1 | d[5];
    bitcount += 6;
    versum += ver as u32;
    println!("{}d ver={} type={}", ibuf, ver, type_id);
    match type_id {
        4 => {
            let mut idx = 6;
            let mut val: u64 = 0;
            loop {
                //println!("  loop val={}", val);
                bitcount += 5;
                val <<= 4;
                val |= (d[idx + 1] << 3 | d[idx + 2] << 2 | d[idx + 3] << 1 | d[idx + 4]) as u64;
                //println!("     val={}", val);
                if d[idx] == 0 {
                    break;
                }
                idx += 5;
            }
            println!("{}  val={}", ibuf, val);
            (bitcount, versum, val as u64)
        }

        _ => {
            let mut subvals: Vec<u64> = Vec::new();

            println!("{}  op type {}", ibuf, type_id);
            let len_type = d[6];
            bitcount += 1;
            println!("{}    len_type={}", ibuf, len_type);
            if len_type == 0 {
                let mut mylen: u32 = 0;
                for i in 0..15 {
                    mylen <<= 1;
                    mylen |= d[i + 7] as u32;
                    bitcount += 1;
                }
                println!("{}    len={}", ibuf, mylen);

                while mylen != 0 {
                    let (bc, vc, myv) = parse_packet(indent + 1, &d[bitcount..]);
                    mylen -= bc as u32;
                    bitcount += bc;
                    versum += vc;
                    subvals.push(myv);
                }
            } else {
                let mut mycount: u32 = 0;
                for i in 0..11 {
                    mycount <<= 1;
                    mycount |= d[i + 7] as u32;
                    bitcount += 1;
                }
                println!("{}    count={}", ibuf, mycount);

                while mycount != 0 {
                    let (bc, vc, myv) = parse_packet(indent + 1, &d[bitcount..]);
                    mycount -= 1;
                    bitcount += bc;
                    versum += vc;
                    subvals.push(myv);
                }
            }

            let mut myvalue = 0;
            match type_id {
                0 => {
                    myvalue = subvals.iter().sum();
                    println!("{}{}=sum of {:?}", ibuf, myvalue, subvals);
                }
                1 => {
                    myvalue = subvals.iter().product();
                    println!("{}{}=prod of {:?}", ibuf, myvalue, subvals);
                }
                2 => {
                    myvalue = *subvals.iter().min().unwrap();
                    println!("{}{}=min of {:?}", ibuf, myvalue, subvals);
                }
                3 => {
                    myvalue = *subvals.iter().max().unwrap();
                    println!("{}{}=max of {:?}", ibuf, myvalue, subvals);
                }
                5 => {
                    if subvals.len() != 2 {
                        panic!("Malformed GT packet");
                    }
                    if subvals[0] > subvals[1] {
                        myvalue = 1
                    }
                    println!("{}{} = gt of {:?}", ibuf, myvalue, subvals);
                }
                6 => {
                    if subvals.len() != 2 {
                        panic!("Malformed LT packet");
                    }
                    if subvals[0] < subvals[1] {
                        myvalue = 1
                    };
                    println!("{}{} = lt of {:?}", ibuf, myvalue, subvals);
                }
                7 => {
                    if subvals.len() != 2 {
                        panic!("Malformed EQ packet");
                    }
                    if subvals[0] == subvals[1] {
                        myvalue = 1
                    }
                    println!("{}{} = eq of {:?}", ibuf, myvalue, subvals);
                }
                _ => panic!("unknown type_id"),
            };

            (bitcount, versum, myvalue)
        }
    }
}

#[allow(dead_code)]
pub fn day16_p1() {
    println!("Day 16 Puzzle 1");

    if let Ok(lines) = util::read_lines(FILENAME) {
        for line in lines {
            if let Ok(ip) = line {
                let row: Vec<u8> = ip
                    .trim()
                    .split("")
                    .filter(|x| !x.is_empty())
                    .map(|x| u8::from_str_radix(x, 16).unwrap())
                    .collect();
                let mut brow: Vec<u8> = Vec::new();
                for x in row.iter() {
                    for i in (0..4).rev() {
                        brow.push((x >> i) & 0x1);
                    }
                }
                println!("row = {:?}", brow);
                let (bitcount, versum, value) = parse_packet(0, brow.as_slice());
                println!("bitcount = {:?}", bitcount);
                println!("versum = {:?}", versum);
                println!("value= {:?}", value);
                println!("");
            }
        }
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}
