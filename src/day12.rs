use std::convert::identity;

use super::*;
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, map: &Vec<Vec<u8>>) -> Vec<Pos> {
        let &Pos(x, y) = self;
        let mut result = Vec::new();
        let max_target = map[x][y] + 1;
        if x > 0 && map[x - 1][y] <= max_target {
            result.push(Pos(x - 1, y));
        }
        if x < map.len() - 1 && map[x + 1][y] <= max_target {
            result.push(Pos(x + 1, y));
        }
        if y > 0 && map[x][y - 1] <= max_target {
            result.push(Pos(x, y - 1));
        }
        if y < map[x].len() - 1 && map[x][y + 1] <= max_target {
            result.push(Pos(x, y + 1));
        }
        result
    }
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut map: Vec<Vec<u8>> = Vec::new();
        let mut x = 0;
        let mut target = Pos(0, 0);
        let mut start = Pos(0, 0);
        let mut starts: Vec<Pos> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                map.push(ip.chars().map(|c| c as u8).collect());
                map[map.len() - 1].iter().enumerate().for_each(|(y, c)| {
                    if *c == ('E' as u8) {
                        target = Pos(x, y);
                    }
                    if *c == ('S' as u8) {
                        start = Pos(x, y);
                        starts.push(Pos(x, y));
                    }
                    if *c == ('a' as u8) {
                        starts.push(Pos(x, y));
                    }
                });
            }
            x += 1;
        }
        let Pos(x, y) = target;
        map[x][y] = 'z' as u8;
        let Pos(x, y) = start;
        map[x][y] = 'a' as u8;
        let result = bfs(&start, |p| p.successors(&map), |p| *p == target);
        println!("{:?}", result.expect("").len() - 1);
        let lens = starts
            .iter()
            .map(|s| bfs(s, |p| p.successors(&map), |p| *p == target))
            .filter_map(identity)
            .map(|r| r.len() - 1).min();
        println!("{:?}", lens)
    }
}
