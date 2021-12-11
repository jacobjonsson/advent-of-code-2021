use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 11] Part 1: {}", part_1(&input));
    println!("[DAY 11] Part 2: {}", part_2(&input));
}

type OctopusMap = HashMap<(usize, usize), u8>;

fn parse_input(input: &str) -> OctopusMap {
    let mut map = OctopusMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x, y), c.to_digit(10).unwrap() as u8);
        }
    }

    map
}

fn increment_map(map: &mut OctopusMap) {
    for (_, octopus) in map.iter_mut() {
        *octopus += 1;
    }
}

fn find_flashed(map: &OctopusMap) -> Vec<(usize, usize)> {
    map.iter()
        .filter(|(_, o)| **o > 9)
        .map(|(x, _)| *x)
        .collect()
}

fn get_adjacent(x: i32, y: i32, map: &OctopusMap) -> Vec<(usize, usize)> {
    [
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
    ]
    .into_iter()
    .filter(|(x, y)| *x >= 0 && *y >= 0)
    .map(|(x, y)| (x as usize, y as usize))
    .filter(|(x, y)| map.contains_key(&(*x, *y)))
    .map(|(x, y)| (x, y))
    .collect()
}

fn simulate(map: &mut OctopusMap) -> HashSet<(usize, usize)> {
    increment_map(map);

    let mut flash_stack = find_flashed(&map);
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    for flash in flash_stack.iter() {
        flashed.insert(*flash);
    }

    while !flash_stack.is_empty() {
        let item = flash_stack.pop().unwrap();

        let adjacent = get_adjacent(item.0 as i32, item.1 as i32, &map);

        for adj in adjacent {
            let item = map.get_mut(&adj).unwrap();
            *item += 1;
            if *item > 9 && !flashed.contains(&adj) {
                flash_stack.push(adj);
                flashed.insert(adj);
            }
        }
    }

    map.iter_mut()
        .filter(|(_, o)| **o > 9)
        .for_each(|(_, o)| *o = 0);

    flashed
}

fn part_1(input: &str) -> usize {
    let mut map = parse_input(input);
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += simulate(&mut map).len();
    }

    flashes
}

fn part_2(input: &str) -> usize {
    let mut map = parse_input(input);
    let grid_width = map.keys().filter(|(x, _)| *x == 0).count();

    for step in 0..9999 {
        if grid_width * grid_width == simulate(&mut map).len() {
            return step + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1656);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 195);
    }
}
