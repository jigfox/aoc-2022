use std::{collections::HashMap, hash::Hash};

use super::*;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Fill {
    Rock,
    Sand,
}

// fn print_cave(
//     most_deep: usize,
//     most_left: usize,
//     most_right: usize,
//     blocked: &HashMap<Coords, Fill>,
// ) {
//     for y in 0..most_deep + 3 {
//         for x in most_left - 1..most_right + 2 {
//             if y > most_deep + 1 {
//                 print!("#");
//             } else if let Some(fill) = blocked.get(&Coords { x, y }) {
//                 match fill {
//                     Fill::Rock => print!("#"),
//                     Fill::Sand => print!("o"),
//                 };
//             } else if x == 500 && y == 0 {
//                 print!("+");
//             } else {
//                 print!(".");
//             }
//         }
//         println!("");
//     }
// }

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut most_deep = 0usize;
        let mut most_left = usize::MAX;
        let mut most_right = 0usize;
        let mut blocked: HashMap<Coords, Fill> = HashMap::new();
        for line in lines {
            if let Ok(ip) = line {
                let coords = ip
                    .split(" -> ")
                    .map(|c| {
                        c.split(",")
                            .map(|i| i.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .map(|c| Coords { x: c[0], y: c[1] })
                    .collect::<Vec<Coords>>();
                for i in 0..coords.len() - 1 {
                    // println!("{}: {:?} -> {:?}", i, coords[i], coords[i+1]);
                    if coords[i].x == coords[i + 1].x {
                        let x = coords[i].x;
                        let y1 = coords[i].y;
                        let y2 = coords[i + 1].y;
                        if y1 > y2 {
                            for y in y2..y1 + 1 {
                                blocked.insert(Coords { x, y }, Fill::Rock);
                            }
                        } else {
                            for y in y1..y2 + 1 {
                                blocked.insert(Coords { x, y }, Fill::Rock);
                            }
                        }
                    } else if coords[i].y == coords[i + 1].y {
                        let y = coords[i].y;
                        let x1 = coords[i].x;
                        let x2 = coords[i + 1].x;
                        if x1 > x2 {
                            for x in x2..x1 + 1 {
                                blocked.insert(Coords { x, y }, Fill::Rock);
                            }
                        } else {
                            for x in x1..x2 + 1 {
                                blocked.insert(Coords { x, y }, Fill::Rock);
                            }
                        }
                    }
                }
                for c in &coords {
                    if c.y > most_deep {
                        most_deep = c.y;
                    }
                    if c.x > most_right {
                        most_right = c.x;
                    }
                    if c.x < most_left {
                        most_left = c.x;
                    }
                }
            }
        }
        // print_cave(most_deep, most_left, most_right, &blocked);

        let mut y = 0;
        let mut x = 500;
        let mut count = 0;
        loop {
            if let Some(_) = blocked.get(&Coords { x, y }) {
                if let Some(_) = blocked.get(&Coords { x: x - 1, y }) {
                    if let Some(_) = blocked.get(&Coords { x: x + 1, y }) {
                        blocked.insert(Coords { x, y: y - 1 }, Fill::Sand);
                        count += 1;
                        y = 0;
                        x = 500;
                    } else {
                        x += 1;
                    }
                } else {
                    x -= 1;
                }
            }
            if y > most_deep {
                break;
            }
            y += 1;
        }
        // print_cave(most_deep, most_left, most_right, &blocked);
        println!("part 1: {}", count);
        y = 0;
        x = 500;
        loop {
            if let Some(_) = blocked.get(&Coords { x: 500, y: 0 }) {
                break;
            }
            if y > most_deep+1 {
                blocked.insert(Coords{x,y: y-1}, Fill::Sand);
                blocked.insert(Coords{x,y}, Fill::Rock);
                if x < most_left {
                    most_left = x;
                }
                if x > most_right {
                    most_right = x;
                }
                count += 1;
                y = 0;
                x = 500;
            }
            if let Some(_) = blocked.get(&Coords { x, y }) {
                if let Some(_) = blocked.get(&Coords { x: x - 1, y }) {
                    if let Some(_) = blocked.get(&Coords { x: x + 1, y }) {
                        blocked.insert(Coords { x, y: y - 1 }, Fill::Sand);
                        count += 1;
                        y = 0;
                        x = 500;
                    } else {
                        x += 1;
                    }
                } else {
                    x -= 1;
                }
            }
            y += 1;
        }
        // print_cave(most_deep, most_left, most_right, &blocked);
        println!("part 2: {}", count);
    
    }
}
