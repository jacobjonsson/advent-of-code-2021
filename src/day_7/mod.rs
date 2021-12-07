pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 7] Part 1: {}", part_1(&input));
    println!("[DAY 7] Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    let mut positions: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    positions.sort();

    let position = positions[positions.len() / 2];
    positions.iter().map(|pos| (pos - position).abs()).sum()
}

fn part_2(input: &str) -> i32 {
    let positions: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut cost = i32::MAX;

    for pos in min..max as i32 {
        let new_cost = positions
            .iter()
            .map(|x| (x - pos).abs())
            .map(|n| n * (n + 1) / 2)
            .sum();

        if new_cost > cost {
            break;
        }

        cost = new_cost;
    }

    cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(part_1(input), 37);
    }

    #[test]
    fn test_part_2() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(part_2(input), 168);
    }
}
