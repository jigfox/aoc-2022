use super::*;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    calories: Vec<i32>,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            calories: Vec::new(),
        }
    }

    fn sum_calories(&self) -> i32 {
        let mut sum = 0;
        for item in &self.calories {
            sum += item
        }
        sum
    }
}

pub fn day01(filename: String) {
    let mut elfes: Vec<Elf> = Vec::new();
    // File hosts must exist in current path before this produces output
    let mut elf = Elf::new();
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let result = ip.to_string().parse::<i32>();
                match result {
                    Ok(num) => {
                        elf.calories.push(num);
                    }
                    Err(_) => {
                        elfes.push(elf);
                        elf = Elf::new()
                    }
                }
            }
        }
    }
    elfes.sort_by(|a, b| b.sum_calories().cmp(&a.sum_calories()));
    println!("part 1: {}", elfes[0].sum_calories());
    println!(
        "part 2: {}",
        elfes[0].sum_calories() + elfes[1].sum_calories() + elfes[2].sum_calories()
    )
}
