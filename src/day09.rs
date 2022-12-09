use std::collections::HashSet;

use super::*;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Posisition {
    x: isize,
    y: isize,
}
struct Head {
    position: Posisition,
}
struct Tail {
    position: Posisition,
}
struct Rope {
    head: Head,
    tail: Tail,
    knots: Vec<Tail>,
    visisted: HashSet<Posisition>,
    visisted2: HashSet<Posisition>,
}

impl Posisition {
    fn clone(&self) -> Posisition {
        Posisition { x: self.x, y: self.y }
    }
}

impl Tail {
    fn follow(self: &mut Tail, position: &Posisition) -> Posisition {
        let x_diff = position.x - self.position.x;
        let y_diff = position.y - self.position.y;
        // println!("{}, {}", x_diff, y_diff);
        if x_diff.abs() > 1 && y_diff.abs() > 0 || x_diff.abs() > 0 && y_diff.abs() > 1 {
            self.position.x += x_diff.signum();
            self.position.y += y_diff.signum();
        } else if x_diff.abs() > 1 {
            self.position.x += x_diff.signum();
        } else if y_diff.abs() > 1 {
            self.position.y += y_diff.signum();
        }
        // println!("H: {:?}, T: {:?}", position, self.position);
        self.position.clone()
    }
}

impl Rope {
    fn move_up(self: &mut Rope, amount: usize) {
        for _ in 0..amount {
            self.head.position.x += 1;
            self.visisted.insert(self.tail.follow(&self.head.position));
            let head_position = self.head.position.clone();
            let new_position = self.follow_knots(&head_position);
            self.visisted2.insert(new_position);
        }
    }
    fn move_down(self: &mut Rope, amount: usize) {
        for _ in 0..amount {
            self.head.position.x -= 1;
            self.visisted.insert(self.tail.follow(&self.head.position));
            let head_position = self.head.position.clone();
            let new_position = self.follow_knots(&head_position);
            self.visisted2.insert(new_position);
        }
    }
    fn move_left(self: &mut Rope, amount: usize) {
        for _ in 0..amount {
            self.head.position.y -= 1;
            self.visisted.insert(self.tail.follow(&self.head.position));
            let head_position = self.head.position.clone();
            let new_position = self.follow_knots(&head_position);
            self.visisted2.insert(new_position);
        }
    }
    fn move_right(self: &mut Rope, amount: usize) {
        for _ in 0..amount {
            self.head.position.y += 1;
            self.visisted.insert(self.tail.follow(&self.head.position));
            let head_position = self.head.position.clone();
            let new_position = self.follow_knots(&head_position);
            self.visisted2.insert(new_position);
        }
    }
    fn follow_knots(self: &mut Rope, head_position: &Posisition) -> Posisition {
        let mut position = head_position.clone();
        for i in 0..self.knots.len() {
            position = self.knots[i].follow(&position)
        }
        self.knots[self.knots.len() - 1].position.clone()
    }
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut rope = Rope {
            head: Head { position: Posisition { x: 0, y: 0} },
            tail: Tail { position: Posisition { x: 0, y: 0} },
            visisted: HashSet::new(),
            visisted2: HashSet::new(),
            knots: Vec::new(),
        };
        for _ in 0..9 {
            rope.knots.push(Tail { position: Posisition { x: 0, y: 0 } })
        }
        for line in lines {
            if let Ok(ip) = line {
                let split: Vec<&str> = ip.split(" ").collect();
                // println!("{:?}", split);
                match split[0] {
                    "R" => {
                        if let Ok(amount) = split[1].parse::<usize>() {
                            rope.move_right(amount)
                        }
                    },
                    "U" => {
                        if let Ok(amount) = split[1].parse::<usize>() {
                            rope.move_up(amount)
                        }
                    },
                    "L" => {
                        if let Ok(amount) = split[1].parse::<usize>() {
                            rope.move_left(amount)
                        }
                    },
                    "D" => {
                        if let Ok(amount) = split[1].parse::<usize>() {
                            rope.move_down(amount)
                        }
                    },
                    _ => panic!("unsupported direction")
                }
            }
        }
        // println!("{:?}", rope.visisted);
        println!("{}", rope.visisted.len());
        println!("{}", rope.visisted2.len());
    }
}

