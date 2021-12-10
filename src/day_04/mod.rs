use anyhow::Result;
use std::{collections::HashMap, str::FromStr};

pub fn run() {
    let input = include_str!("./input.txt");

    let part_1 = part_1(&input).unwrap();
    let part_2 = part_2(&input).unwrap();

    println!("[DAY 4] Part 1: {}", part_1);
    println!("[DAY 4] Part 2: {}", part_2);
}

#[derive(Debug, Clone)]
struct Board {
    numbers: HashMap<(usize, usize), u64>,
    marks: HashMap<(usize, usize), bool>,
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = HashMap::new();
        let mut marks = HashMap::new();

        for (y, line) in s.trim().lines().enumerate() {
            for (x, num) in line.split_whitespace().enumerate() {
                numbers.insert((x, y), num.parse()?);
                marks.insert((x, y), false);
            }
        }

        Ok(Self { numbers, marks })
    }
}

impl Board {
    pub fn mark(&mut self, number: u64) {
        for (k, v) in &self.numbers {
            if v == &number {
                *self.marks.get_mut(k).unwrap() = true;
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        for i in 0..5 {
            let winner = self
                .marks
                .iter()
                .filter(|((x, _), _)| x == &i)
                .all(|(_, m)| *m);
            if winner {
                return true;
            }
        }

        for j in 0..5 {
            let winner = self
                .marks
                .iter()
                .filter(|((_, y), _)| y == &j)
                .all(|(_, m)| *m);
            if winner {
                return true;
            }
        }

        false
    }

    pub fn sum_unmarked(&self) -> u64 {
        self.marks
            .iter()
            .filter_map(|((x, y), m)| match m {
                true => None,
                false => self.numbers.get(&(*x, *y)),
            })
            .sum()
    }
}

fn parse_input(input: &str) -> Result<(Vec<u64>, Vec<Board>)> {
    let mut lines = input.lines();

    let numbers: Vec<u64> = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut boards = Vec::new();
    let mut board_iter = lines.peekable();

    while board_iter.peek().is_some() {
        let board: Board = board_iter
            .by_ref()
            .skip(1)
            .take(5)
            .map(|l| l.to_owned() + "\n")
            .collect::<String>()
            .parse()?;

        boards.push(board);
    }

    Ok((numbers, boards))
}

fn part_1(input: &str) -> Result<u64> {
    let (numbers, boards) = parse_input(input)?;
    let mut boards = boards.clone();

    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.is_winner() {
                return Ok(board.sum_unmarked() * number);
            }
        }
    }

    Ok(0)
}

fn part_2(input: &str) -> Result<u64> {
    let (numbers, boards) = parse_input(input)?;

    let mut boards = boards;
    let mut boards_remaining = boards.len();

    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);

            if board.is_winner() && boards_remaining == 1 {
                return Ok(board.sum_unmarked() * number);
            }
        }

        boards = boards
            .iter()
            .filter_map(|board| match board.is_winner() {
                true => None,
                false => Some(board.clone()),
            })
            .collect();

        boards_remaining = boards.len();
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(part_1(&input).unwrap(), 4512);
    }

    #[test]
    fn test_part_2() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(part_2(&input).unwrap(), 1924);
    }
}
