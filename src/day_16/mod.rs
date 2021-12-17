use std::str::Chars;

use itertools::Itertools;

pub fn run() {
    let input = include_str!("./input.txt").trim();

    println!("[DAY 16] Part 1: {}", part_1(&input));
    println!("[DAY 16] Part 2: {}", part_2(&input));
}

fn hex_to_binary<'a>(ch: char) -> Chars<'a> {
    match ch {
        '0' => "0000".chars(),
        '1' => "0001".chars(),
        '2' => "0010".chars(),
        '3' => "0011".chars(),
        '4' => "0100".chars(),
        '5' => "0101".chars(),
        '6' => "0110".chars(),
        '7' => "0111".chars(),
        '8' => "1000".chars(),
        '9' => "1001".chars(),
        'A' => "1010".chars(),
        'B' => "1011".chars(),
        'C' => "1100".chars(),
        'D' => "1101".chars(),
        'E' => "1110".chars(),
        'F' => "1111".chars(),
        _ => panic!("Invalid hex character: {}", ch),
    }
}

#[derive(Debug)]
struct Bits<'a> {
    value: &'a str,
    pos: usize,
}

impl<'a> Bits<'a> {
    pub fn new(value: &'a str) -> Self {
        Bits { value, pos: 0 }
    }

    pub fn take(&mut self, n: usize) -> &'a str {
        let val = &self.value[self.pos..self.pos + n];
        self.pos += n;
        val
    }

    pub fn next(&mut self) -> &'a str {
        let val = &self.value[self.pos..self.pos + 1];
        self.pos += 1;
        val
    }

    pub fn len(&self) -> usize {
        self.value[self.pos..].len()
    }
}

fn chars_to_binary(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

fn parse_literal(bits: &mut Bits) -> usize {
    let mut s = String::new();

    while bits.take(1) == "1" {
        s.push_str(bits.take(4));
    }

    s.push_str(bits.take(4));

    chars_to_binary(&s)
}

fn parse_header(bits: &mut Bits) -> (usize, usize) {
    let version = chars_to_binary(bits.take(3));
    let type_id = chars_to_binary(bits.take(3));

    (version, type_id)
}

fn parse_packet(bits: &mut Bits) -> Option<(usize, usize)> {
    if bits.len() < 11 {
        return None;
    }

    let (version, type_id) = parse_header(bits);

    if type_id == 4 {
        Some((version, parse_literal(bits)))
    } else {
        let (ver, value) = parse_operator(type_id, bits);
        Some((version + ver, value))
    }
}

fn parse_operator(type_id: usize, bits: &mut Bits) -> (usize, usize) {
    let indicator = bits.next();
    let mut version_sum = 0;
    let mut values: Vec<usize> = Vec::new();

    if indicator == "0" {
        let length = chars_to_binary(bits.take(15));
        let sub_packets = bits.take(length);
        let mut sub_packet_bits = Bits::new(sub_packets);
        loop {
            if let Some((version, value)) = parse_packet(&mut sub_packet_bits) {
                version_sum += version;
                values.push(value);
            } else {
                break;
            }
        }
    } else {
        let length = chars_to_binary(bits.take(11));
        for idx in 0..length {
            if let Some((version, value)) = parse_packet(bits) {
                version_sum += version;
                values.push(value);
            } else {
                panic!("Expected {} sub packets but only found {}", length, idx + 1);
            }
        }
    }

    (version_sum, operator_on_value(type_id, &values))
}

fn operator_on_value(type_id: usize, values: &[usize]) -> usize {
    match type_id {
        0 => values.iter().fold(0, |a, c| a + c),
        1 => values.iter().fold(1, |a, c| a * c),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => {
            if values[0] > values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if values[0] < values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if values[0] == values[1] {
                1
            } else {
                0
            }
        }
        _ => panic!("Invalid operator type: {}", type_id),
    }
}

fn part_1(input: &str) -> usize {
    let val = input.chars().flat_map(hex_to_binary).join("");
    let mut bits = Bits::new(&val);

    let (version, _) = parse_packet(&mut bits).unwrap();

    version
}

fn part_2(input: &str) -> usize {
    let val = input.chars().flat_map(hex_to_binary).join("");
    let mut bits = Bits::new(&val);

    let (_, value) = parse_packet(&mut bits).unwrap();

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("8A004A801A8002F478"), 16);
    }

    #[test]
    fn test_part_2() {
        let tests = [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for (source, value) in tests {
            assert_eq!(part_2(source), value);
        }
    }
}
