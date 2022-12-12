use super::*;

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut cycles = vec![1isize];
        for line in lines {
            if let Ok(ip) = line {
                if ip == "noop" {
                    cycles.push(*cycles.last().expect(""));
                } else {
                    let new = ip
                        .split(" ")
                        .last()
                        .expect("number")
                        .parse::<isize>()
                        .expect("parsable");
                    let last = *cycles.last().expect("");
                    cycles.push(last);
                    cycles.push(last + new);
                }
            }
        }
        println!(
            "part 1: {}",
            cycles[19] * 20
                + cycles[59] * 60
                + cycles[99] * 100
                + cycles[139] * 140
                + cycles[179] * 180
                + cycles[219] * 220
        );
        let mut screen = vec![
            vec!['.'; 40],
            vec!['.'; 40],
            vec!['.'; 40],
            vec!['.'; 40],
            vec!['.'; 40],
            vec!['.'; 40],
        ];
        for cycle in 0..240 {
            let pos = cycle % 40;
            let row = cycle / 40;
            screen[row][pos] = if pos as isize+1 >= cycles[cycle] && pos as isize + 1 <= cycles[cycle]+2 {
                '#'
            } else {
                '.'
            };
        }
        for line in screen {
            println!("{}", String::from_iter(line));
        }
    }
}
