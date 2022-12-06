use super::*;

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip)
            }
        }
    }
}

