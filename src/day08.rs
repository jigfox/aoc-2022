use super::*;

fn is_visible(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> bool {
    if x == 0 || y == 0 || x == trees[y].len() - 1 || y == trees.len() - 1 {
        return true;
    }
    if is_visible_top(x, y, trees) {
        return true;
    }
    if is_visible_bottom(x, y, trees) {
        return true;
    }
    if is_visible_left(x, y, trees) {
        return true;
    }
    if is_visible_right(x, y, trees) {
        return true;
    }
    false
}

fn is_visible_top(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> bool {
    for yi in 0..y {
        if trees[yi][x] >= trees[y][x] {
            return false;
        }
    }
    true
}

fn is_visible_bottom(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> bool {
    for yi in y + 1..trees.len() {
        if trees[yi][x] >= trees[y][x] {
            return false;
        }
    }
    true
}

fn is_visible_left(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> bool {
    for xi in 0..x {
        if trees[y][xi] >= trees[y][x] {
            return false;
        }
    }
    true
}

fn is_visible_right(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> bool {
    for xi in x + 1..trees[y].len() {
        if trees[y][xi] >= trees[y][x] {
            return false;
        }
    }
    true
}

fn get_scenic_score(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> usize {
    let top = get_scenic_score_top(x, y, trees);
    let bottom = get_scenic_score_bottom(x, y, trees);
    let right = get_scenic_score_right(x, y, trees);
    let left = get_scenic_score_left(x, y, trees);
    return top * bottom * right * left;
}

fn get_scenic_score_top(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> usize {
    let mut count = 0;
    let mut yi = y;
    while yi > 0 {
        count += 1;
        if trees[yi - 1][x] >= trees[y][x] {
            break;
        }
        yi -= 1;
    }
    count
}

fn get_scenic_score_bottom(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> usize {
    let mut count = 0;
    for yi in y + 1..trees.len() {
        count += 1;
        if trees[yi][x] >= trees[y][x] {
            break;
        }
    }
    count
}

fn get_scenic_score_left(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> usize {
    let mut count = 0;
    let mut xi = x;
    while xi > 0 {
        count += 1;
        if trees[y][xi - 1] >= trees[y][x] {
            break;
        }
        xi -= 1;
    }
    count
}

fn get_scenic_score_right(x: usize, y: usize, trees: &Vec<Vec<u8>>) -> usize {
    let mut count = 0;
    for xi in x + 1..trees[y].len() {
        count += 1;
        if trees[y][xi] >= trees[y][x] {
            break;
        }
    }
    count
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        let mut trees: Vec<Vec<u8>> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut row: Vec<u8> = Vec::new();
            if let Ok(ip) = line {
                for tree in ip.chars().into_iter() {
                    if let Ok(height) = tree.to_string().parse::<u8>() {
                        row.push(height)
                    }
                }
            }
            trees.push(row);
        }

        let mut sum = 0;
        let mut tallest = 0;
        for y in 0..trees.len() {
            for x in 0..trees[y].len() {
                if is_visible(x, y, &trees) {
                    sum += 1;
                }
                let t = get_scenic_score(x, y, &trees);
                if t > tallest {
                    tallest = t;
                }
            }
        }
        println!("part 1: {}", sum);
        println!("part 2: {}", tallest);
    }
}
