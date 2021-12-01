use crate::input_loader::{read_input, split_by_line};

pub fn run() {
    let input = read_input("src/day_1/input.txt");
    let lines = split_by_line(&input);

    let part_1 = part_1(&lines);
    let part_2 = part_2(&lines);

    println!("[DAY 1] Part 1: {}", part_1);
    println!("[DAY 1] Part 2: {}", part_2);
}

fn part_1(lines: &[&str]) -> i32 {
    let mut count = 0;
    let mut prev = lines.get(0).unwrap().parse::<i32>().unwrap();

    for line in lines[1..].iter() {
        let num = line.parse::<i32>().unwrap();
        if prev < num {
            count += 1;
        }
        prev = num;
    }

    count
}

fn part_2(lines: &[&str]) -> i32 {
    let mut count = 0;
    let mut prev = get_rolling_window(lines, 0);

    for (idx, _) in lines[1..lines.len()].iter().enumerate() {
        if idx + 2 > lines.len() - 1 {
            break;
        }

        let num = get_rolling_window(lines, idx);
        if prev < num {
            count += 1;
        }
        prev = num;
    }

    count
}

fn get_rolling_window(lines: &[&str], start: usize) -> i32 {
    lines[start..start + 3]
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .sum()
}

#[test]
fn test_part_2() {
    let lines = split_by_line(
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

    assert_eq!(part_2(&lines), 5);
}
