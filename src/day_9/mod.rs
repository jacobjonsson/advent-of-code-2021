use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 9] Part 1: {}", part_1(&input));
    println!("[DAY 9] Part 2: {}", part_2(&input));
}

type PointKey = (i32, i32);
type PointValue = u8;

fn parse_input(input: &str) -> HashMap<PointKey, PointValue> {
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), c.to_digit(10).unwrap() as u8);
        }
    }

    grid
}

fn find_low_points(grid: &HashMap<PointKey, PointValue>) -> Vec<(PointKey, PointValue)> {
    let mut low_points: Vec<(PointKey, PointValue)> = vec![];

    for ((x, y), v) in grid.iter() {
        let top = (*x, y - 1);
        let bottom = (*x, y + 1);
        let left = (x - 1, *y);
        let right = (x + 1, *y);

        let adjacent_locations = vec![
            grid.get(&top),
            grid.get(&bottom),
            grid.get(&left),
            grid.get(&right),
        ];

        let low_point = adjacent_locations.iter().all(|i| match i {
            Some(i) => *i > v,
            None => true,
        });

        if low_point {
            low_points.push(((*x, *y), *v));
        }
    }

    low_points
}

fn part_1(input: &str) -> u32 {
    let grid = parse_input(input);

    find_low_points(&grid)
        .iter()
        .map(|(_, v)| (v + 1) as u32)
        .sum()
}

fn part_2(input: &str) -> usize {
    let grid = parse_input(input);
    let low_points = find_low_points(&grid);
    let mut basins = Vec::new();

    for ((x, y), v) in low_points {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        let mut basin = vec![v];

        queue.push_back((x, y - 1));
        queue.push_back((x, y + 1));
        queue.push_back((x - 1, y));
        queue.push_back((x + 1, y));

        let mut visited: HashSet<(u8, u8)> = HashSet::new();
        visited.insert((x as u8, y as u8));

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            let value = grid.get(&(x, y));

            if visited.contains(&(x as u8, y as u8)) {
                continue;
            }

            visited.insert((x as u8, y as u8));

            if value.is_none() || *value.unwrap() == 9 {
                continue;
            }

            basin.push(*value.unwrap());
            queue.push_back((x, y - 1));
            queue.push_back((x, y + 1));
            queue.push_back((x - 1, y));
            queue.push_back((x + 1, y));
        }

        basins.push(basin.len());
    }

    basins.sort_by(|a, b| b.cmp(a));
    basins.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(part_1(input), 15);
    }

    #[test]
    fn test_part_2() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

        assert_eq!(part_2(input), 1134);
    }
}
