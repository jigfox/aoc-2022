use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use super::*;

#[derive(Debug)]
struct Pos(isize, isize);

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn distance(&self) -> isize {
        self.distanct_to(&self.beacon)
    }
    fn distanct_to(&self, pos: &Pos) -> isize {
        (self.pos.0 - pos.0).abs() + (self.pos.1 - pos.1).abs()
    }
    fn borders(&self) -> (isize, isize, isize, isize) {
        (
            self.pos.1 - self.distance(),
            self.pos.0 + self.distance(),
            self.pos.1 + self.distance(),
            self.pos.0 - self.distance(),
        )
    }
    fn within_distance(&self, pos: &Pos) -> bool {
        self.distanct_to(pos) <= self.distance()
    }
    fn is_beacon(&self, pos: &Pos) -> bool {
        self.beacon.0 == pos.0 && self.beacon.1 == pos.1
    }
}

impl FromStr for Sensor {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = s
            .split(":")
            .map(|s| {
                s.trim()
                    .split(",")
                    .map(|s| s.trim().split("=").map(|s| s.trim()).last())
                    .map(|s| s.unwrap().parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .collect::<Vec<Vec<isize>>>();
        let pos = Pos(a[0][0], a[0][1]);
        let beacon = Pos(a[1][0], a[1][1]);
        Ok(Sensor { pos, beacon })
    }
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        let mut sensors: Vec<Sensor> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let sensor = ip.parse::<Sensor>().unwrap();
                let (top, right, bottom, left) = sensor.borders();
                min_x = min_x.min(left);
                max_x = max_x.max(right);
                min_y = min_y.min(top);
                max_y = max_y.max(bottom);
                sensors.push(sensor);
            }
        }
        // println!("{},{},{},{}", min_x, max_x, min_y, max_y);
        // let y = 10;      // test input
        let y = 2000000; // real input
        let mut unavailable_pos: HashSet<isize> = HashSet::new();
        for x in min_x..max_x + 1 {
            for sensor in &sensors {
                if sensor.within_distance(&Pos(x,y)) {
                    unavailable_pos.insert(x);
                    break;
                }
            }
        }
        for x in min_x..max_x+1 {
            for sensor in &sensors {
                if sensor.is_beacon(&Pos(x,y)) {
                    unavailable_pos.remove(&x);
                }
            }
        }
        println!("part 1: {}", unavailable_pos.len());
        // let max = 20; // test input
        let max = 4000000; // real input
        let mut found = Pos(-1,-1);
        for sensor in &sensors {
            let distance = sensor.distance();

            let mut pos = Pos(sensor.pos.0 - distance - 1, sensor.pos.1);

            while pos.0 != sensor.pos.0 {
                pos.0 += 1;
                pos.1 += 1;
                if check(&pos, &sensors, max) {
                    found.0 = pos.0;
                    found.1 = pos.1;
                    break;
                }
            }
            while pos.1 != sensor.pos.1 {
                pos.0 += 1;
                pos.1 -= 1;
                if check(&pos, &sensors, max) {
                    found.0 = pos.0;
                    found.1 = pos.1;
                    break;
                }
            }
            while pos.0 != sensor.pos.0 {
                pos.0 -= 1;
                pos.1 -= 1;
                if check(&pos, &sensors, max) {
                    found.0 = pos.0;
                    found.1 = pos.1;
                    break;
                }
            }
            while pos.1 != sensor.pos.1 {
                pos.0 -= 1;
                pos.1 += 1;
                if check(&pos, &sensors, max) {
                    found.0 = pos.0;
                    found.1 = pos.1;
                    break;
                }
            }
            if found.1 >= 0 && found.0 >= 0 {
                break;
            }
        };
        println!("part 2:{:?}", found.0 * 4000000 + found.1);
    }
}

fn check(pos: &Pos, sensors: &Vec<Sensor>, limit: isize) -> bool {
    let mut fits = pos.0 >= 0 && pos.0 <= limit && pos.1 >=0 && pos.1 <= limit;

    if !fits { return false }

    for sensor in sensors {
        let distance = sensor.distance();
        fits = fits && sensor.distanct_to(pos) > distance;

        if !fits { return false }
    }
    fits
}
