use itertools::{EitherOrBoth::*, Itertools};
use std::collections::HashMap;

pub fn run() {
    let input = include_str!("./input.txt");

    let part_1 = part_1(&input);
    let part_2 = part_2(&input);

    println!("[DAY 5] Part 1: {}", part_1);
    println!("[DAY 5] Part 2: {}", part_2);
}

fn part_1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|s| {
            s.split(" -> ")
                .map(|x| {
                    x.split(",")
                        .filter_map(|s| s.parse().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();

    let mut grid_insert = |(x, y): (usize, usize), val: usize| {
        if grid.contains_key(&(x, y)) {
            *grid.get_mut(&(x, y)).unwrap() += val;
        } else {
            grid.insert((x, y), val);
        }
    };

    for line in lines {
        let (start, end) = (&line[0], &line[1]);
        let (mut start_x, mut start_y) = (start[0], start[1]);
        let (mut end_x, mut end_y) = (end[0], end[1]);

        if start_x != end_x && start_y != end_y {
            continue;
        }

        if start_x > end_x {
            std::mem::swap(&mut start_x, &mut end_x);
        } else if start_y > end_y {
            std::mem::swap(&mut start_y, &mut end_y);
        }

        if start_x == end_x {
            for y in start_y..end_y + 1 {
                grid_insert((start_x, y), 1);
            }
        }

        if start_y == end_y {
            for x in start_x..end_x + 1 {
                grid_insert((x, start_y), 1);
            }
        }
    }

    grid.iter().filter(|(_, v)| **v >= 2).count()
}

fn part_2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|s| {
            s.split(" -> ")
                .map(|x| {
                    x.split(",")
                        .filter_map(|s| s.parse().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();

    let mut grid_insert = |x: usize, y: usize| {
        if grid.contains_key(&(x, y)) {
            *grid.get_mut(&(x, y)).unwrap() += 1;
        } else {
            grid.insert((x, y), 1);
        }
    };

    for line in lines {
        let (start, end) = (&line[0], &line[1]);
        let (start_x, start_y) = (start[0], start[1]);
        let (end_x, end_y) = (end[0], end[1]);

        let (xs, x_fallback) = if start_x > end_x {
            ((end_x..start_x + 1).rev().collect::<Vec<_>>(), start_x)
        } else {
            ((start_x..end_x + 1).collect::<Vec<_>>(), end_x)
        };

        let (ys, y_fallback) = if start_y > end_y {
            ((end_y..start_y + 1).rev().collect::<Vec<_>>(), start_y)
        } else {
            ((start_y..end_y + 1).collect::<Vec<_>>(), end_y)
        };

        xs.into_iter().zip_longest(ys).for_each(|pair| {
            let (x, y) = match pair {
                Both(x, y) => (x, y),
                Left(x) => (x, y_fallback),
                Right(y) => (x_fallback, y),
            };

            grid_insert(x, y);
        });
    }

    grid.iter().filter(|(_, v)| **v >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(part_1(&input), 5);
    }

    #[test]
    fn test_part_2() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(part_2(&input), 12);
    }
}
