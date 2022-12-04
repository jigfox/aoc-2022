use super::*;
use regex::Regex;

struct Assignment {
    from: i32,
    to: i32,
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        other.from >= self.from && other.to <= self.to
    }
}

struct Assignments {
    one: Assignment,
    two: Assignment,
}

impl Assignments {
    fn new(line: &str) -> Assignments {
        let re = Regex::new(r"(?P<a1>\d+)-(?P<a2>\d+),(?P<b1>\d+)-(?P<b2>\d+)").unwrap();
        let matches = re.captures(line).unwrap();
        if let Ok(from1) = matches["a1"].parse::<i32>() {
            if let Ok(to1) = matches["a2"].parse::<i32>() {
                if let Ok(from2) = matches["b1"].parse::<i32>() {
                    if let Ok(to2) = matches["b2"].parse::<i32>() {
                        return Assignments {
                            one: Assignment {
                                from: from1,
                                to: to1,
                            },
                            two: Assignment {
                                from: from2,
                                to: to2,
                            },
                        };
                    }
                }
            }
        }
        Assignments {
            one: Assignment { from: 0, to: 0 },
            two: Assignment { from: 0, to: 0 },
        }
    }

    fn is_contained(&self) -> bool {
        self.one.contains(&self.two) || self.two.contains(&self.one)
    }

    fn do_overlap(&self) -> bool {
        (self.one.from >= self.two.from && self.one.from <= self.two.to)
            || (self.one.to >= self.two.from && self.one.to <= self.two.to)
            || (self.two.from >= self.one.from && self.two.from <= self.one.to)
            || (self.two.to >= self.one.from && self.two.to <= self.one.to)
    }
}

pub fn solve(filename: String) {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut sum1 = 0;
        let mut sum2 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let job = Assignments::new(&ip);
                if job.is_contained() {
                    sum1 += 1;
                }
                if job.do_overlap() {
                    sum2 += 1;
                }
            }
        }
        println!("{}", sum1);
        println!("{}", sum2);
    }
}
