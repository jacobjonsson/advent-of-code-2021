use std::collections::HashMap;

pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 13] Part 1: {}", part_1(&input));
    println!("[DAY 13] Part 2:\n{}", part_2(&input));
}

type Grid = HashMap<(usize, usize), bool>;

fn create_grid(max_x: usize, max_y: usize) -> Grid {
    let mut grid = HashMap::new();

    for x in 0..=max_x {
        for y in 0..=max_y {
            grid.insert((x, y), false);
        }
    }

    grid
}

fn fill_grid(points: Vec<(usize, usize)>, grid: &mut Grid) {
    for point in points {
        grid.insert(point, true);
    }
}

#[derive(Debug, Clone, Copy)]
enum FoldInstruction {
    X(usize),
    Y(usize),
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<FoldInstruction>) {
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<FoldInstruction> = Vec::new();

    for line in input.lines() {
        if line == "" {
            continue;
        }

        if line.contains("fold along y") {
            folds.push(FoldInstruction::Y(
                line.split("=").nth(1).unwrap().parse().unwrap(),
            ));

            continue;
        }

        if line.contains("fold along x") {
            folds.push(FoldInstruction::X(
                line.split("=").nth(1).unwrap().parse().unwrap(),
            ));

            continue;
        }

        let mut parts = line.split(",");
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();

        points.push((x, y));
    }

    (points, folds)
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) -> String {
    let max_x = grid.iter().map(|((x, _), _)| *x).max().unwrap();
    let max_y = grid.iter().map(|((_, y), _)| *y).max().unwrap();

    let mut output = String::new();

    for y in 0..=max_y {
        if y != 0 {
            output.push_str("\n");
        }
        for x in 0..=max_x {
            if *grid.get(&(x, y)).unwrap() {
                output.push_str("#");
            } else {
                output.push_str(".");
            }
        }
    }

    output
}

fn fold_grid(grid: &Grid, fold_instruction: FoldInstruction) -> Grid {
    let max_x = grid.iter().map(|((x, _), _)| *x).max().unwrap();
    let max_y = grid.iter().map(|((_, y), _)| *y).max().unwrap();

    let mut new_grid = match fold_instruction {
        FoldInstruction::X(x) => create_grid(x - 1, max_y),
        FoldInstruction::Y(y) => create_grid(max_x, y - 1),
    };

    for ((x, y), filled) in grid.iter().filter(|(_, filled)| **filled) {
        let mut new_x = *x;
        let mut new_y = *y;

        if let FoldInstruction::X(fold_x) = fold_instruction {
            if *x > fold_x {
                new_x = max_x - *x;
            }
        }

        if let FoldInstruction::Y(fold_y) = fold_instruction {
            if *y > fold_y {
                new_y = max_y - *y;
            }
        }

        match new_grid.get_mut(&(new_x, new_y)) {
            Some(value) => *value = *filled,
            None => println!("Failed to insert {:?}", (new_x, new_y)),
        }
    }

    new_grid
}

fn part_1(input: &str) -> usize {
    let (points, fold_instructions) = parse_input(input);

    let max_x = points.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap();

    let mut grid = create_grid(max_x, max_y);
    fill_grid(points, &mut grid);

    grid = fold_grid(&grid, fold_instructions[0]);

    // print_grid(&grid);

    grid.iter().filter(|(_, filled)| **filled).count()
}

fn part_2(input: &str) -> String {
    let (points, fold_instructions) = parse_input(input);

    let max_x = points.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap();

    let mut grid = create_grid(max_x, max_y);
    fill_grid(points, &mut grid);

    for fold_instruction in fold_instructions {
        grid = fold_grid(&grid, fold_instruction);
    }

    print_grid(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 17);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(INPUT),
            "#####
#...#
#...#
#...#
#####
.....
....."
        );
    }
}
