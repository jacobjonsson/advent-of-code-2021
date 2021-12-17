pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 17] Part 1: {}", part_1(&input));
    println!("[DAY 17] Part 2: {}", part_2(&input));
}

type Range = (isize, isize);
type Target = (Range, Range);

fn in_target(x: isize, y: isize, target: Target) -> bool {
    let (x_range, y_range) = target;
    let (x0, x1) = x_range;
    let (y0, y1) = y_range;

    x0 <= x && x <= x1 && y0 <= y && y <= y1
}

fn parse_range(input: &str) -> (isize, isize) {
    let mut parts = input.split("=");
    parts.next();
    let mut range_parts = parts.next().unwrap().split("..");
    let start = range_parts.next().unwrap().parse::<isize>().unwrap();
    let end = range_parts.next().unwrap().parse::<isize>().unwrap();
    (start, end)
}

fn part_1(input: &str) -> isize {
    let mut parts = (&input[13..].trim()).split(", ");

    let _ = parse_range(parts.next().unwrap());
    let y_target = parse_range(parts.next().unwrap());

    let y0 = y_target.0;

    y0 * (y0 - isize::signum(y0)) / 2
}

fn part_2(input: &str) -> usize {
    let mut parts = (&input[13..].trim()).split(", ");

    let x_target = parse_range(parts.next().unwrap());
    let (_, x1) = x_target;

    let y_target = parse_range(parts.next().unwrap());
    let (y0, _) = y_target;

    let mut cnt = 0;

    for x_vel in 1..=x1 {
        for y_vel in y0..(1 - y0) {
            let mut x_velocity: isize = x_vel;
            let mut y_velocity = y_vel;
            let mut x = 0;
            let mut y = 0;

            loop {
                x += x_velocity;
                y += y_velocity;

                x_velocity -= x_velocity.signum();
                y_velocity -= 1;

                if in_target(x, y, (x_target, y_target)) {
                    cnt += 1;
                    break;
                }

                if y >= y_target.0 && x < x_target.1 {
                    continue;
                }

                break;
            }
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 45);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 112);
    }
}
