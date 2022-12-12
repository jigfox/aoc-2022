use std::collections::VecDeque;

use super::*;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Addition,
    Multiplikation,
}

struct Monkey {
    items: VecDeque<usize>,
    items2: VecDeque<usize>,
    devisible_by: usize,
    true_target: usize,
    false_target: usize,
    action_counter: usize,
    action_counter2: usize,
    operation: Operation,
    operate_by: usize,
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut monkeys: Vec<Monkey> = Vec::new();
        let mut items: VecDeque<usize> = VecDeque::new();
        let mut devisible_by: usize = 0;
        let mut true_target: usize = 0;
        let mut false_target: usize = 0;
        let mut operation: Operation = Operation::Addition;
        let mut operate_by: usize = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("Monkey") {
                    items = VecDeque::new();
                } else if ip.starts_with("  Starting items") {
                    let split = ip.split(":").last();
                    if let Some(nums) = split {
                        items = nums
                            .trim()
                            .split(",")
                            .filter_map(|s| s.trim().parse::<usize>().ok())
                            .collect();
                    }
                } else if ip.starts_with("  Operation") {
                    let split = ip.split(" = ").last().unwrap_or("");
                    if split.starts_with("old + ") {
                        operation = Operation::Addition;
                        operate_by = split
                            .split(" + ")
                            .last()
                            .unwrap_or("")
                            .parse::<usize>()
                            .unwrap_or(0);
                    } else if split.starts_with("old * ") {
                        operation = Operation::Multiplikation;
                        operate_by = split
                            .split(" * ")
                            .last()
                            .unwrap_or("")
                            .parse::<usize>()
                            .unwrap_or(0);
                    } else {
                        unimplemented!();
                    }
                } else if ip.starts_with("  Test") {
                    let split = ip.split(" by ").last();
                    if let Some(num) = split {
                        devisible_by = num.trim().parse::<usize>().unwrap_or(0)
                    }
                } else if ip.starts_with("    If true") {
                    let split = ip.split(" monkey ").last();
                    if let Some(num) = split {
                        true_target = num.trim().parse::<usize>().unwrap_or(0)
                    }
                } else if ip.starts_with("    If false") {
                    let split = ip.split(" monkey ").last();
                    if let Some(num) = split {
                        false_target = num.trim().parse::<usize>().unwrap_or(0)
                    }
                    monkeys.push(Monkey {
                        items: items.clone(),
                        items2: items.clone(),
                        devisible_by,
                        true_target,
                        false_target,
                        action_counter: 0,
                        action_counter2: 0,
                        operation,
                        operate_by,
                    });
                }
            }
        }
        let divisor = monkeys
            .iter()
            .map(|m| m.devisible_by)
            .fold(1usize, |a, b| a * b);
        for _ in 0..20 {
            for mi in 0..monkeys.len() {
                let mut items = monkeys[mi].items.clone();
                monkeys[mi].items = VecDeque::new();

                while let Some(item) = items.pop_front() {
                    monkeys[mi].action_counter += 1;
                    let level = match monkeys[mi].operation {
                        Operation::Addition => {
                            if monkeys[mi].operate_by > 0 {
                                item + monkeys[mi].operate_by
                            } else {
                                item + item
                            }
                        }
                        Operation::Multiplikation => {
                            if monkeys[mi].operate_by > 0 {
                                item * monkeys[mi].operate_by
                            } else {
                                item * item
                            }
                        }
                    } / 3;
                    let target = if level % monkeys[mi].devisible_by == 0 {
                        monkeys[mi].true_target
                    } else {
                        monkeys[mi].false_target
                    };
                    monkeys[target].items.push_back(level % divisor);
                }
            }
        }
        for _ in 0..10000 {
            for mi in 0..monkeys.len() {
                let mut items = monkeys[mi].items2.clone();
                monkeys[mi].items2 = VecDeque::new();

                while let Some(item) = items.pop_front() {
                    monkeys[mi].action_counter2 += 1;
                    let level = match monkeys[mi].operation {
                        Operation::Addition => {
                            if monkeys[mi].operate_by > 0 {
                                item + monkeys[mi].operate_by
                            } else {
                                item + item
                            }
                        }
                        Operation::Multiplikation => {
                            if monkeys[mi].operate_by > 0 {
                                item * monkeys[mi].operate_by
                            } else {
                                item * item
                            }
                        }
                    };
                    let target = if level % monkeys[mi].devisible_by == 0 {
                        monkeys[mi].true_target
                    } else {
                        monkeys[mi].false_target
                    };
                    monkeys[target].items2.push_back(level % divisor);
                }
            }
        }
        let mut s = monkeys
            .iter()
            .map(|m| m.action_counter)
            .collect::<Vec<usize>>();
        s.sort();
        s.reverse();
        let s = s.iter().take(2).fold(1usize, |a, b| a * b);
        println!("part 1: {:?}", s);
        let mut s = monkeys
            .iter()
            .map(|m| m.action_counter2)
            .collect::<Vec<usize>>();
        s.sort();
        s.reverse();
        let s = s.iter().take(2).fold(1usize, |a, b| a * b);
        println!("part 2: {:?}", s);
    }
}
