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

fn part_1(input: &str) -> usize {
    todo!()
}

fn part_2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "";

        assert_eq!(part_1(&input), todo!());
    }

    #[test]
    fn test_part_2() {
        let input = "";

        assert_eq!(part_2(&input), todo!());
    }
}
