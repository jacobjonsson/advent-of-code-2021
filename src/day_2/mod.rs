use crate::input_loader::read_input;

pub fn run() {
    let input = read_input("src/day_2/input.txt");

    let part_1 = part_1(&input);
    let part_2 = part_2(&input);

    println!("[DAY 2] Part 1: {}", part_1);
    println!("[DAY 2] Part 2: {}", part_2);
}

enum InstructionKind {
    Forward,
    Down,
    Up,
}

struct Instruction {
    kind: InstructionKind,
    value: u8,
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| {
        let items: Vec<_> = line.split_whitespace().collect();

        if items.len() != 2 {
            panic!("Invalid instruction: {}", line);
        }

        let kind = match items[0] {
            "forward" => InstructionKind::Forward,
            "down" => InstructionKind::Down,
            "up" => InstructionKind::Up,
            item => panic!("Unknown instruction kind: {}", item),
        };

        let value = items[1]
            .parse::<u8>()
            .expect(&format!("Failed to parse instruction value: {}", items[1]));

        Instruction { kind, value }
    })
}

fn part_1(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;

    for instruction in parse_instructions(input) {
        match instruction.kind {
            InstructionKind::Forward => horizontal += instruction.value as usize,
            InstructionKind::Down => depth += instruction.value as usize,
            InstructionKind::Up => depth -= instruction.value as usize,
        }
    }

    return horizontal * depth;
}

fn part_2(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in parse_instructions(input) {
        match instruction.kind {
            InstructionKind::Forward => {
                horizontal += instruction.value as usize;
                depth += aim * instruction.value as usize;
            }
            InstructionKind::Down => aim += instruction.value as usize,
            InstructionKind::Up => aim -= instruction.value as usize,
        }
    }

    return horizontal * depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(part_1(&input), 150);
    }

    #[test]
    fn test_part_2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(part_2(&input), 900);
    }
}
