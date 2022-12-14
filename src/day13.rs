use super::*;
use serde_json::{Result, Value, Number};

#[derive(Debug)]
enum PacketResult {
    True,
    False,
    Indifferent,
}

fn compare(left: &Value, right: &Value) -> PacketResult {
    match left {
        Value::Number(l) => match right {
            Value::Number(right) => {
                return if l.as_u64() < right.as_u64() {
                    PacketResult::True
                } else if l.as_u64() > right.as_u64() {
                    PacketResult::False
                } else {
                    PacketResult::Indifferent
                }
            }
            Value::Array(_) => {
                return compare(&Value::Array(vec![left.clone()]), right);
            }
            _ => panic!("invalid"),
        },
        Value::Array(l) => match right {
            Value::Number(_) => {
                return compare(left, &Value::Array(vec![right.clone()]));
            }
            Value::Array(right) => {
                let mut count = -1isize;
                loop {
                    count += 1;
                    if count as usize == l.len() && count as usize == right.len() {
                        return PacketResult::Indifferent;
                    }
                    if count as usize == l.len() {
                        return PacketResult::True;
                    }
                    if count as usize == right.len() {
                        return PacketResult::False;
                    }
                    let r = compare(&left[count as usize], &right[count as usize]);
                    match r {
                        PacketResult::Indifferent => continue,
                        _ => return r,
                    }
                }
            }
            _ => panic!("invalid"),
        },
        _ => panic!("invalid"),
    }
}

pub fn solve(filename: String) -> Result<()> {
    if let Ok(lines) = read_lines(&filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut pairs: Vec<(Value, Value)> = Vec::new();
        let mut pair = (Value::Null, Value::Null);
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("[") {
                    let v: Value = serde_json::from_str(ip.as_str())?;
                    match pair.0 {
                        Value::Null => {
                            pair.0 = v;
                        }
                        _ => {
                            pair.1 = v;
                        }
                    }
                } else {
                    pairs.push(pair.clone());
                    pair = (Value::Null, Value::Null);
                }
            }
        }
        pairs.push(pair);
        // let i = 1;
        // println!("{:?}", compare(&pairs[i].0, &pairs[i].1));
        let mut sum = 0;
        let mut packets: Vec<Value> = Vec::new();
        for (idx, pair) in pairs.iter().enumerate() {
            packets.push(pair.0.clone());
            packets.push(pair.1.clone());
            let r = compare(&pair.0, &pair.1);
            match r {
                PacketResult::False => {}
                _ => sum += idx + 1,
            }
        }
        let two = Value::Array(vec![Value::Array(vec![Value::Number(Number::from(2))])]);
        let six = Value::Array(vec![Value::Array(vec![Value::Number(Number::from(6))])]);
        packets.push(two.clone());
        packets.push(six.clone());
        println!("part1: {}", sum);
        packets.sort_by(|l, r| match compare(l, r) {
            PacketResult::Indifferent => std::cmp::Ordering::Equal,
            PacketResult::True => std::cmp::Ordering::Less,
            PacketResult::False => std::cmp::Ordering::Greater,
        });
        let two_pos = packets.iter().position(|x| match compare(&x, &two) {
            PacketResult::Indifferent => true,
            _ => false,
        }).unwrap() + 1;
        let six_pos = packets.iter().position(|x| match compare(&x, &six) {
            PacketResult::Indifferent => true,
            _ => false,
        }).unwrap() + 1;
        println!("part2: {}", two_pos * six_pos)
    }
    Ok(())
}
