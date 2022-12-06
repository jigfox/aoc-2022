use super::*;
use regex::Regex;

struct Stack {
    crates: Vec<char>,
}

pub fn solve(filename: String) {
    let spaces_re = Regex::new(r"^\s\s\s").unwrap();
    let spaces_re2 = Regex::new(r"\s\s\s\s").unwrap();
    let spaces_re3 = Regex::new(r"\s\s\s$").unwrap();
    let crate_re = Regex::new(r"[_A-Z]").unwrap();
    let move_re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d) to (?P<to>\d)").unwrap();
    let mut stacks: Vec<Stack> = Vec::new();
    let mut stacks2: Vec<Stack> = Vec::new();

    if let Ok(lines) = read_lines(&filename) {
        let mut parsing_crates = true;
        let mut first_line = true;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if first_line {
                    let stack_count = (ip.len() + 1) / 4;
                    for _ in 0..stack_count {
                        stacks.push(Stack { crates: Vec::new() });
                        stacks2.push(Stack { crates: Vec::new() });
                    }
                    first_line = false;
                }
                if ip.len() == 0 {
                    parsing_crates = false;
                    continue;
                }
                if parsing_crates {
                    let ip = spaces_re.replace_all(&ip, "[_]");
                    let ip = spaces_re2.replace_all(&ip, " [_]");
                    let ip = spaces_re3.replace_all(&ip, "[_]");
                    let caps: Vec<&str> = crate_re
                        .find_iter(&ip)
                        .filter_map(|i| Some(i.as_str()))
                        .collect();
                    for (i, c) in caps.iter().enumerate() {
                        if *c != "_" {
                            stacks[i].crates.push(c.chars().collect::<Vec<char>>()[0]);
                            stacks2[i].crates.push(c.chars().collect::<Vec<char>>()[0]);
                        }
                    }
                } else {
                    let caps = move_re.captures(&ip).unwrap();
                    if let Ok(count) = caps["count"].parse::<usize>() {
                        if let Ok(from) = caps["from"].parse::<usize>() {
                            if let Ok(to) = caps["to"].parse::<usize>() {
                                for i in 0..count {
                                    let c = stacks[from - 1].crates.remove(0);
                                    stacks[to - 1].crates.insert(0, c);
                                    let c2 = stacks2[from-1].crates.remove(0);
                                    stacks2[to -1].crates.insert(i, c2);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for stack in stacks {
        print!("{}", stack.crates[0]);
    }
    println!("");
    for stack in stacks2 {
        print!("{}", stack.crates[0]);
    }
    println!("");
}
