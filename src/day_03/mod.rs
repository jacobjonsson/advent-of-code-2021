pub fn run() {
    let input = include_str!("./input.txt");

    let part_1 = part_1(&input);
    let part_2 = part_2(&input);

    println!("[DAY 3] Part 1: {}", part_1);
    println!("[DAY 3] Part 2: {}", part_2);
}

fn lines<'a>(input: &'a str) -> Vec<&'a str> {
    input.lines().collect()
}

fn most_common_at_idx<'a>(lines: &Vec<&'a str>, idx: usize) -> Option<char> {
    let mut counts = [0, 0];

    for line in lines {
        match line.chars().nth(idx) {
            Some('0') => counts[0] += 1,
            Some('1') => counts[1] += 1,
            _ => (),
        }
    }

    if counts[0] > counts[1] {
        Some('0')
    } else if counts[1] > counts[0] {
        Some('1')
    } else {
        None
    }
}

fn part_1(input: &str) -> usize {
    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    let lines = lines(input);

    for idx in 0..lines[0].len() {
        let ch = most_common_at_idx(&lines, idx).unwrap();
        if ch == '0' {
            gamma.push('0');
            epsilon.push('1')
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn part_2(input: &str) -> usize {
    let lines = lines(input);
    let mut oxygen_lines = lines.clone();
    let mut co2_lines = lines.clone();

    // Oxygen
    for idx in 0..lines[0].len() {
        if oxygen_lines.len() == 1 {
            break;
        }

        let ch = most_common_at_idx(&oxygen_lines, idx).unwrap_or('1');

        oxygen_lines = filter_lines(oxygen_lines, idx, ch);
    }

    for idx in 0..lines[0].len() {
        if co2_lines.len() == 1 {
            break;
        }

        let ch = most_common_at_idx(&co2_lines, idx)
            .map(|ch| if ch == '0' { '1' } else { '0' })
            .unwrap_or('0');

        co2_lines = filter_lines(co2_lines, idx, ch);
    }

    assert_eq!(oxygen_lines.len(), 1);
    assert_eq!(co2_lines.len(), 1);

    let oxygen = usize::from_str_radix(&oxygen_lines[0], 2).unwrap();
    let co2 = usize::from_str_radix(&co2_lines[0], 2).unwrap();

    oxygen * co2
}

fn filter_lines<'a>(lines: Vec<&'a str>, idx: usize, ch: char) -> Vec<&'a str> {
    lines
        .iter()
        .filter(move |line| line.chars().nth(idx).unwrap() == ch)
        .map(|line| *line)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(part_1(&input), 198);
    }

    #[test]
    fn test_part_2() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(part_2(&input), 230);
    }
}
