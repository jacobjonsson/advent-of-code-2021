pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY _] Part 1: {}", part_1(&input));
    println!("[DAY _] Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "";

        assert_eq!(part_1(input), 1);
    }

    #[test]
    fn test_part_2() {
        let input = "";

        assert_eq!(part_2(input), 1);
    }
}
