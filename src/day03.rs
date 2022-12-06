use super::*;
fn get_prio(c: char) -> i32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}

#[derive(Debug)]
struct Rucksack {
    both: Vec<char>,
    compartment1: Vec<char>,
    compartment2: Vec<char>,
}

impl Rucksack {
    fn new(content: String) -> Rucksack {
        let mut compartment1: Vec<char> = content.chars().collect();
        let compartment2: Vec<char> = compartment1.split_off(content.len() / 2);

        Rucksack {
            both: content.chars().collect(),
            compartment1,
            compartment2,
        }
    }

    fn find_duplicates(&self) -> char {
        let mut c = '.';
        for item in &self.compartment1[..] {
            let pos = &self.compartment2[..].into_iter().find(|x| x == &item);
            if let Some(position) = pos {
                c = **position;
            }
        }
        c
    }
}


fn find_common_item(group: &[Rucksack]) -> char {
    let mut c = '.';
    for item in &group[0].both[..] {
        let pos = &group[1].both[..].into_iter().find(|x| x == &item);
        if let Some(_) = pos {
            let pos2 = &group[2].both[..].into_iter().find(|x| x == &item);
            if let Some(second) = pos2 {
                c = **second
            }
        }
    }
    c
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        let mut rucksacks: Vec<Rucksack> = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for line in lines {
            if let Ok(ip) = line {
                let rucksack = Rucksack::new(ip);
                sum += get_prio(rucksack.find_duplicates());
                rucksacks.push(rucksack)
            }
        }
        let groups = rucksacks.chunks(3);
        let mut sum2 = 0;
        for group in groups {
            sum2 += get_prio(find_common_item(group));
        }
        println!("part1: {}", sum);
        println!("part2: {}", sum2)
    }
}
