use crate::*;
use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;

const FILENAME: &str = "src/day19/day19_input.txt";
//const FILENAME: &str = "src/day19/day19_example.txt";
//const FILENAME: &str = "src/day19/day19_miniA.txt";

#[derive(Debug, Clone, Eq, Hash, Copy)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone)]
struct Offset {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialEq for Point3d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Point3d {
    fn new(x: i64, y: i64, z: i64) -> Point3d {
        Point3d { x: x, y: y, z: z }
    }

    fn roll(&mut self) -> &mut Self {
        // Can't do this in one step, yet. https://github.com/rust-lang/rfcs/issues/372
        let (tmpx, tmpy, tmpz) = (self.x, self.y, self.z);
        self.x = tmpx;
        self.y = tmpz;
        self.z = -1 * tmpy;
        self
    }

    fn turn(&mut self) -> &mut Self {
        let (tmpx, tmpy, tmpz) = (self.x, self.y, self.z);
        self.x = -1 * tmpy;
        self.y = tmpx;
        self.z = tmpz;
        self
    }

    fn get_offset(&self, other: &Point3d) -> Offset {
        //Offset { x: self.x - other.x, y:self.y - other.y, z:self.z - other.z }
        Offset {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    // Returns a new point offset by a point
    fn add_offset(&self, offset: &Offset) -> Point3d {
        Point3d {
            x: self.x + offset.x,
            y: self.y + offset.y,
            z: self.z + offset.z,
        }
    }
}

#[derive(Debug)]
struct ProbeData {
    points: HashSet<Point3d>,
    rotation_count: i32,
    probe_id: usize,
    probe_offset: Offset,
}

impl ProbeData {
    fn new(id: usize) -> ProbeData {
        ProbeData {
            points: HashSet::new(),
            rotation_count: 0,
            probe_id: id,
            probe_offset: Offset { x: 0, y: 0, z: 0 },
        }
    }

    fn rotate(&mut self) {
        let mut tmp_points: HashSet<Point3d> = HashSet::new();
        for point in &self.points {
            let mut tmp_point = point.clone();
            if (self.rotation_count % 4) == 0 {
                tmp_point.roll();
            } else {
                tmp_point.turn();
            }
            if self.rotation_count == 11 {
                tmp_point.roll().turn().roll();
            }
            tmp_points.insert(tmp_point);
        }
        self.points = tmp_points;
        self.rotation_count += 1;
        if self.rotation_count >= 24 {
            self.rotation_count = 0;
        }
    }

    fn apply_offset(&mut self, offset: &Offset) {
        let mut tmp_points: HashSet<Point3d> = HashSet::new();

        for point in &self.points {
            tmp_points.insert(point.add_offset(offset));
        }
        self.points = tmp_points;
        self.probe_offset = offset.clone();
    }

    fn insert(&mut self, point: Point3d) {
        self.points.insert(point);
    }
}

fn check_overlaps(allpoints: &HashSet<Point3d>, probe: &ProbeData, offset: &mut Offset) -> i64 {
    for fixed_p in allpoints.iter() {
        for probe_p in probe.points.iter() {
            let tmp_offset = probe_p.get_offset(fixed_p);
            //let tmp_offset = fixed_p.get_offset(probe_p);
            let mut oq = 0;

            //println!("checking overlap {:?}", tmp_offset);
            //println!("fixed_p: {:?}", fixed_p);
            //println!("probe_p: {:?}", probe_p);

            for probe_p in probe.points.iter() {
                let test_p = probe_p.add_offset(&tmp_offset);
                //println!("  test_p : {:?}", test_p);
                if allpoints.contains(&test_p) {
                    //println!("Allpoints contains point {:?}", test_p);
                    oq += 1;
                }
            }
            //println!("-->oq={}",oq);
            if oq >= 12 {
                //println!("Found {} overlaps", oq);
                //offset = tmp_offset;
                offset.x = tmp_offset.x;
                offset.y = tmp_offset.y;
                offset.z = tmp_offset.z;
                return oq;
            }
        }
    }
    0
}

// Some cleverness here:
// https://stackoverflow.com/questions/16452383/how-to-get-all-24-rotations-of-a-3-dimensional-array
// We can generate all 24 rotations by just repeating a sequence of 4 rotations
// Clever
#[allow(dead_code)]
pub fn day19_p1() {
    println!("Day 19 Puzzle 1");

    // --- scanner 36 ---
    let re_scanner = Regex::new(r"^--- scanner (\d+) ---$").unwrap();
    // 721,-680,490
    let re_point = Regex::new(r"^(.+),(.+),(.+)$").unwrap();

    if let Ok(lines) = util::read_lines(FILENAME) {
        let mut probelist: VecDeque<ProbeData> = VecDeque::new();
        let mut cur_scanner: usize = 0;
        for line in lines {
            if let Ok(ip) = line {
                if let Some(cap) = re_scanner.captures(&ip) {
                    cur_scanner = cap[1].parse().unwrap();
                    println!("Scanner {}", cur_scanner);
                    probelist.push_back(ProbeData::new(cur_scanner));
                    // TODO: check for skipped/duplicated scanner numbers...
                } else if let Some(cap) = re_point.captures(&ip) {
                    let point = Point3d {
                        x: cap[1].parse().unwrap(),
                        y: cap[2].parse().unwrap(),
                        z: cap[3].parse().unwrap(),
                    };
                    probelist[cur_scanner].insert(point);
                } else {
                    // Ignore line
                }
            }
        }

        println!("{:#?}", probelist);
        let mut completed_beacons: HashSet<Point3d> = HashSet::new();
        let mut completed_scanners: VecDeque<ProbeData> = VecDeque::new();

        for point in probelist[0].points.iter() {
            completed_beacons.insert(*point);
        }
        completed_scanners.push_back(probelist.pop_front().unwrap());

        let mut checked_list: HashSet<(usize, usize)> = HashSet::new();

        'a: while !probelist.is_empty() {
            let mut tmp_probe = probelist.pop_front().unwrap();

            for s in &completed_scanners {
                if checked_list.contains(&(s.probe_id, tmp_probe.probe_id)) {
                    //println!("already compared {} to {}, skipping", s.probe_id, tmp_probe.probe_id);
                    continue;
                }

                for r in 0..24 {
                    let mut offset = Offset { x: 0, y: 0, z: 0 };
                    let overlap_count = check_overlaps(&s.points, &tmp_probe, &mut offset);
                    if overlap_count >= 12 {
                        println!(
                            "fixed: {} probe: {} oc: {}",
                            s.probe_id, tmp_probe.probe_id, overlap_count
                        );
                        // TODO: apply offset
                        // TODO: add points to completed_beacons list
                        tmp_probe.apply_offset(&offset);
                        for point in tmp_probe.points.iter() {
                            completed_beacons.insert(point.clone());
                        }
                        completed_scanners.push_back(tmp_probe);
                        continue 'a;
                    }
                    tmp_probe.rotate();
                }
                checked_list.insert((s.probe_id, tmp_probe.probe_id));
            }
            println!(
                "Nothing found for probe {}; pushing to back of list",
                tmp_probe.probe_id
            );
            probelist.push_back(tmp_probe);
        }

        println!("beacon count = {}", completed_beacons.len());

        let mut max_md = i64::MIN;
        for a in &completed_scanners {
            for b in &completed_scanners {
                let manhattan_distance = (a.probe_offset.x - b.probe_offset.x).abs()
                    + (a.probe_offset.y - b.probe_offset.y).abs()
                    + (a.probe_offset.z - b.probe_offset.z).abs();
                println!(
                    "scanners {},{} md={}",
                    a.probe_id, b.probe_id, manhattan_distance
                );
                max_md = i64::max(max_md, manhattan_distance);
            }
        }
        println!("max_md={}", max_md);
    } else {
        panic!("Couldn't open {}", FILENAME);
    }
}
