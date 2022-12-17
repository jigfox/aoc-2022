use std::collections::HashSet;

use super::*;

enum Move {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coords(usize, usize);

impl Coords {
    fn push_right(&mut self) {
        self.0 += 1;
    }
    fn push_left(&mut self) {
        self.0 -= 1;
    }
    fn push_down(&mut self) {
        self.1 -= 1;
    }
    fn set_entry(&mut self, level: usize) {
        self.1 += level;
    }
}

#[derive(Debug)]
struct Rock {
    fills: Vec<Coords>,
}

impl Rock {
    fn clone(&self) -> Rock {
        Rock {
            fills: self.fills.to_vec(),
        }
    }
    fn push_right(&mut self, blocked: &HashSet<Coords>) {
        if self.most_right() < 6
            && !self.fills.to_vec().iter_mut().any(|c| {
                c.push_right();
                blocked.contains(c)
            })
        {
            self.fills.iter_mut().for_each(|c| c.push_right());
        }
    }

    fn push_left(&mut self, blocked: &HashSet<Coords>) {
        if self.most_left() > 0
            && !self.fills.to_vec().iter_mut().any(|c| {
                c.push_left();
                blocked.contains(c)
            })
        {
            self.fills.iter_mut().for_each(|c| c.push_left());
        }
    }

    fn set_entry(&mut self, level: usize) {
        self.fills.iter_mut().for_each(|c| c.set_entry(level));
    }

    fn push_down(&mut self, blocked: &HashSet<Coords>) -> bool {
        let most_deep = self.most_deep();
        // println!("a{:?}, {:?}", &self, most_deep);
        if *most_deep.iter().min().unwrap() > 0
            && !self.fills.to_vec().iter_mut().any(|c| {
                c.push_down();
                blocked.contains(c)
            })
        {
            self.fills.iter_mut().for_each(|c| c.push_down());
            true
        } else {
            false
        }
    }

    fn most_right(&self) -> usize {
        self.fills.iter().map(|c| c.0).max().unwrap()
    }

    fn most_left(&self) -> usize {
        self.fills.iter().map(|c| c.0).min().unwrap()
    }

    fn most_deep(&self) -> Vec<usize> {
        (0..7)
            .into_iter()
            .map(|i| {
                self.fills
                    .iter()
                    .find(|c| c.0 == i)
                    .map(|c| c.1)
                    .unwrap_or(usize::MAX)
            })
            .collect::<Vec<usize>>()
    }
}

struct Board {
    highest: Vec<usize>,
}

impl Board {
    fn get_entry(&self) -> usize {
        self.highest.iter().max().unwrap() + 4
    }
    fn get_highest(&self) -> usize {
        *self.highest.iter().max().unwrap()
    }
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut board = Board {
            highest: vec![0, 0, 0, 0, 0, 0, 0],
        };
        let rocks = vec![
            // ..0123.
            Rock {
                fills: vec![Coords(2, 0), Coords(3, 0), Coords(4, 0), Coords(5, 0)],
            },
            // ...4...
            // ..123..
            // ...0...
            Rock {
                fills: vec![
                    Coords(3, 0),
                    Coords(2, 1),
                    Coords(3, 1),
                    Coords(4, 1),
                    Coords(3, 2),
                ],
            },
            // ....4..
            // ....3..
            // ..012..
            Rock {
                fills: vec![
                    Coords(2, 0),
                    Coords(3, 0),
                    Coords(4, 0),
                    Coords(4, 1),
                    Coords(4, 2),
                ],
            },
            // ..3....
            // ..2....
            // ..1....
            // ..0....
            Rock {
                fills: vec![Coords(2, 0), Coords(2, 1), Coords(2, 2), Coords(2, 3)],
            },
            // ..23...
            // ..01...
            Rock {
                fills: vec![Coords(2, 0), Coords(3, 0), Coords(2, 1), Coords(3, 1)],
            },
        ];
        for line in lines {
            if let Ok(ip) = line {
                let movements: Vec<Move> = ip
                    .chars()
                    .map(|c| match c {
                        '>' => Move::Right,
                        '<' => Move::Left,
                        _ => panic!("unsupported"),
                    })
                    .collect();
                let mut rocks_count = 0;
                let mut counter = 0;
                let mut blocked: HashSet<Coords> = HashSet::new();
                blocked.insert(Coords(0, 0));
                blocked.insert(Coords(1, 0));
                blocked.insert(Coords(2, 0));
                blocked.insert(Coords(3, 0));
                blocked.insert(Coords(4, 0));
                blocked.insert(Coords(5, 0));
                blocked.insert(Coords(6, 0));
                let mut last = 0;
                while rocks_count < 2022 {
                    let mut rock = rocks[rocks_count % rocks.len()].clone();
                    rock.set_entry(board.get_entry());
                    // println!("->{:?}", rock);
                    loop {
                        match movements[counter % movements.len()] {
                            Move::Left => {
                                rock.push_left(&blocked);
                            }
                            Move::Right => {
                                rock.push_right(&blocked);
                            }
                        }
                        counter += 1;
                        if !rock.push_down(&blocked) {
                            for fill in rock.fills {
                                board.highest[fill.0] = board.highest[fill.0].max(fill.1);
                                blocked.insert(fill);
                            }
                            break;
                        }
                    }
                    rocks_count += 1;
                    if rocks_count % (rocks.len() * movements.len()) == 0 {
                        println!("{}: {} ({})", rocks_count, board.get_highest(), board.get_highest() - last);
                        last = board.get_highest();
                    }
                    if rocks_count == 2022 {
                        println!("{}", board.get_highest());
                    }
                    // println!("{}: {:?}", rocks_count, board.highest);
                }
                println!("{}", board.get_highest());
            }
        }
    }
}
