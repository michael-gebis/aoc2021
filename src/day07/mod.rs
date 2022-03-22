use crate::*;

#[allow(dead_code)]
pub fn calculate_move_costa(v: &std::vec::Vec<usize>, m: usize) -> usize {
    let mut cost = 0;

    for i in v {
        cost += if *i < m { m - *i } else { *i - m }
    }

    cost
}

#[allow(dead_code)]
pub fn calculate_move_costb(v: &std::vec::Vec<usize>, m: usize) -> usize {
    let mut cost = 0;

    for i in v {
        let absdist = if *i < m { m - *i } else { *i - m };
        cost += (absdist * (absdist + 1)) / 2;
    }

    cost
}

#[allow(dead_code)]
pub fn day07_p1() {
    println!("day07_p1!");

    if let Ok(lines) = util::read_lines("src/day07/day07_input.txt") {
        for line in lines {
            if let Ok(ip) = dbg!(line) {
                let mut vals: Vec<usize> = ip.split(",").map(|x| x.parse().unwrap()).collect();
                vals.sort();
                dbg!(vals.clone());

                let minv = vals[0];
                let maxv = vals[vals.len() - 1];
                let mut mincost = usize::MAX;
                let mut minm = 0;
                for i in minv..maxv + 1 {
                    let cost = dbg!(calculate_move_costb(&vals, i));
                    if cost < mincost {
                        mincost = cost;
                        minm = i;
                    }
                }
                dbg!(mincost);
                dbg!(minm);
            }
        }
    }
}
