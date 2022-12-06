use super::*;


fn find_marker(stream: Vec<char>, len: usize) -> usize {
    for i in 0..stream.len()-len-1 {
        let slice = &stream[i..len+i];
        if !(1..slice.len()).any(|c| slice[c..].contains(&slice[c-1])) {
            return i + len
        }
    }
    0
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("part one: {}", find_marker(ip.chars().collect(), 4));
                println!("part two: {}", find_marker(ip.chars().collect(), 14));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn first_example() {
        assert_eq!(
            find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect(),4),
            7
        )
    }

    #[test]
    fn second_example() {
        assert_eq!(
            find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect(),4),
            5
        )
    }

    #[test]
    fn third_example() {
        assert_eq!(
            find_marker("nppdvjthqldpwncqszvftbrmjlhg".chars().collect(),4),
            6
        )
    }

    #[test]
    fn fourth_example() {
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect(),4),
            10
        )
    }

    #[test]
    fn fifth_example() {
        assert_eq!(
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect(),4),
            11
        )
    }
}
