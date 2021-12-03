pub fn run() {
    let input = ints(include_str!("./input.txt"));

    let part_1 = part_1(&input);
    let part_2 = part_2(&input);

    println!("[DAY 1] Part 1: {}", part_1);
    println!("[DAY 1] Part 2: {}", part_2);
}

fn ints(input: &str) -> Vec<u32> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn part_1(lines: &[u32]) -> usize {
    lines.windows(2).filter(|i| i[0] < i[1]).count()
}

#[test]
fn test_part_1() {
    let input = ints(
        "199
200
208
210
200
207
240
269
260
263",
    );

    assert_eq!(part_1(&input), 7);
}

fn part_2(lines: &[u32]) -> usize {
    let depth_windows: Vec<u32> = lines.windows(3).map(|w| w.iter().sum()).collect();
    depth_windows.windows(2).filter(|i| i[0] < i[1]).count()
}

#[test]
fn test_part_2() {
    let input = ints(
        "199
200
208
210
200
207
240
269
260
263",
    );

    assert_eq!(part_2(&input), 5);
}
