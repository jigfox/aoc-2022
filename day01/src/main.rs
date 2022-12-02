use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod day01 {
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

    pub fn day01(args: Vec<String>) {
        let mut elfes: Vec<Elf> = Vec::new();
        // File hosts must exist in current path before this produces output
        let mut elf = Elf::new();
        if let Ok(lines) = read_lines(&args[0]) {
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
}

mod day02 {
    use super::*;

    enum Figure {
        Rock,
        Paper,
        Scissor,
    }

    enum GameResult {
        Win,
        Loose,
        Draw,
    }

    struct Game {
        player1: Figure,
        player2: Figure,
        result: GameResult,
    }

    fn points_for_result(r: GameResult) -> i32 {
        match r {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loose => 0,
        }
    }

    fn points_for_figure(f: Figure) -> i32 {
        match f {
            Figure::Scissor => 3,
            Figure::Rock => 1,
            Figure::Paper => 2,
        }
    }
    impl Game {
        fn new(player1: char, player2: char) -> Game {
            Game {
                player1: match player1 {
                    'Y' => Figure::Paper,
                    'X' => Figure::Rock,
                    'Z' => Figure::Scissor,
                    a => panic!("invalid: {}", a),
                },
                player2: match player2 {
                    'A' => Figure::Rock,
                    'B' => Figure::Paper,
                    'C' => Figure::Scissor,
                    _ => panic!("invalid"),
                },
                result: match player1 {
                    'Y' => GameResult::Draw,
                    'X' => GameResult::Loose,
                    'Z' => GameResult::Win,
                    a => panic!("invalid: {}", a),
                },
            }
        }

        fn points(&self) -> i32 {
            let mut sum: i32 = 0;
            match self.player1 {
                Figure::Paper => {
                    sum += points_for_figure(Figure::Paper);
                    match self.player2 {
                        Figure::Paper => sum += points_for_result(GameResult::Draw),
                        Figure::Scissor => sum += points_for_result(GameResult::Loose),
                        Figure::Rock => sum += points_for_result(GameResult::Win),
                    }
                }
                Figure::Rock => {
                    sum += points_for_figure(Figure::Rock);
                    match self.player2 {
                        Figure::Rock => sum += points_for_result(GameResult::Draw),
                        Figure::Scissor => sum += points_for_result(GameResult::Win),
                        Figure::Paper => sum += points_for_result(GameResult::Loose),
                    }
                }
                Figure::Scissor => {
                    sum += points_for_figure(Figure::Scissor);
                    match self.player2 {
                        Figure::Scissor => sum += points_for_result(GameResult::Draw),
                        Figure::Paper => sum += points_for_result(GameResult::Win),
                        Figure::Rock => sum += points_for_result(GameResult::Loose),
                    }
                }
            }
            sum
        }

        fn points2(&self) -> i32 {
            let mut sum: i32 = 0;
            match self.result {
                GameResult::Draw => {
                    sum += points_for_result(GameResult::Draw);
                    match self.player2 {
                        Figure::Paper => sum += points_for_figure(Figure::Paper),
                        Figure::Scissor => sum += points_for_figure(Figure::Scissor),
                        Figure::Rock => sum += points_for_figure(Figure::Rock),
                    }
                }
                GameResult::Win => {
                    sum += points_for_result(GameResult::Win);
                    match self.player2 {
                        Figure::Rock => sum += points_for_figure(Figure::Paper),
                        Figure::Scissor => sum += points_for_figure(Figure::Rock),
                        Figure::Paper => sum += points_for_figure(Figure::Scissor),
                    }
                }
                GameResult::Loose => {
                    sum += points_for_result(GameResult::Loose);
                    match self.player2 {
                        Figure::Scissor => sum += points_for_figure(Figure::Paper),
                        Figure::Paper => sum += points_for_figure(Figure::Rock),
                        Figure::Rock => sum += points_for_figure(Figure::Scissor),
                    }
                }
            }
            sum
        }
    }

    pub fn day02(args: Vec<String>) {
        if let Ok(lines) = read_lines(&args[0]) {
            // Consumes the iterator, returns an (Optional) String
            let mut sum: i32 = 0;
            let mut sum2: i32 = 0;
            for line in lines {
                if let Ok(ip) = line {
                    let player2 = ip.chars().nth(0).unwrap();
                    let player1 = ip.chars().nth(2).unwrap();
                    let game = Game::new(player1, player2);
                    sum += game.points();
                    sum2 += game.points2();
                }
            }
            println!("{}, {}", sum, sum2)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // day01::day01(args[1..].to_vec());
    day02::day02(args[1..].to_vec())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
