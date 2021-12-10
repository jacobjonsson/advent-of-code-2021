pub fn run() {
    let input = include_str!("./input.txt");

    let part_1 = _run(&input, 80);
    let part_2 = _run(&input, 256);

    println!("[DAY 6] Part 1: {}", part_1);
    println!("[DAY 6] Part 2: {}", part_2);
}

fn _run(input: &str, days: u32) -> u64 {
    let mut fishes: [u64; 9] = [0; 9];

    for s in input.trim().split(',') {
        fishes[s.parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..days {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    fishes.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "3,4,3,1,2";

        assert_eq!(_run(&input, 80), 5934);
    }

    #[test]
    fn test_part_2() {
        let input = "3,4,3,1,2";

        assert_eq!(_run(&input, 256), 26984457539);
    }
}
