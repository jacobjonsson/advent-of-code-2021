use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 15] Part 1: {}", part_1(&input));
    println!("[DAY 15] Part 2: {}", part_2(&input));
}

type Coord = (i32, i32);
type Grid = HashMap<Coord, u32>;

fn parse_input(input: &str) -> Grid {
    let mut map = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }

    map
}

fn find_neighbors(coord: Coord, grid: &Grid) -> Vec<(Coord, u32)> {
    let mut neighbors = Vec::new();

    for (x, y) in [
        (coord.0 - 1, coord.1),
        (coord.0 + 1, coord.1),
        (coord.0, coord.1 - 1),
        (coord.0, coord.1 + 1),
    ] {
        match grid.get(&(x, y)) {
            Some(c) => neighbors.push(((x, y), *c)),
            None => continue,
        }
    }

    neighbors
}

#[derive(Debug, Eq)]
struct Visit {
    coord: Coord,
    risk: u32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Visit {
    fn eq(&self, other: &Self) -> bool {
        self.risk.eq(&other.risk)
    }
}

fn dijkstra<'a>(start: Coord, grid: &'a Grid) -> HashMap<Coord, u32> {
    let mut risks = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();

    risks.insert(start, 0);
    to_visit.push(Visit {
        coord: start,
        risk: 0,
    });

    while let Some(Visit { coord, risk }) = to_visit.pop() {
        if !visited.insert(coord) {
            continue;
        }

        for (neighbor, cost) in find_neighbors(coord, &grid) {
            let new_risk = risk + cost;
            let is_shorter = risks
                .get(&neighbor)
                .map_or(true, |&current| new_risk < current);
            if is_shorter {
                risks.insert(neighbor, new_risk);
                to_visit.push(Visit {
                    coord: neighbor,
                    risk: new_risk,
                });
            }
        }
    }

    risks
}

fn part_1(input: &str) -> u32 {
    let grid = parse_input(input);

    let start_coord = (0, 0);
    let end_coord = (
        grid.keys().map(|&(x, _)| x).max().unwrap(),
        grid.keys().map(|&(_, y)| y).max().unwrap(),
    );

    let risks = dijkstra(start_coord, &grid);

    *risks.get(&end_coord).unwrap()
}

fn increment_tile(
    start_coord: Coord,
    tile: &Grid,
    tile_width: i32,
    tile_height: i32,
    top_row: bool,
    check_ver: bool,
) -> Grid {
    let mut new_tile: Grid = HashMap::new();

    for x in 0..tile_width {
        for y in 0..tile_height {
            let new_coord = (start_coord.0 + x, start_coord.1 + y);

            let prev_coord = if top_row {
                (new_coord.0 - tile_width, new_coord.1)
            } else if check_ver {
                (new_coord.0, new_coord.1 - tile_height)
            } else {
                (new_coord.0 - tile_width, new_coord.1)
            };

            let prev_risk = match tile.get(&prev_coord) {
                Some(r) => *r,
                None => panic!("Failed to get {:?}", prev_coord),
            };

            let new_risk = match prev_risk {
                9 => 1,
                _ => prev_risk + 1,
            };
            new_tile.insert(new_coord, new_risk);
        }
    }

    new_tile
}

fn part_2(input: &str) -> u32 {
    let mut grid = parse_input(input);

    let tile_width = grid.keys().map(|&(x, _)| x).max().unwrap() + 1;
    let tile_height = grid.keys().map(|&(_, y)| y).max().unwrap() + 1;

    // Do the top row first
    for x in 1..=4 {
        let new_tile = increment_tile(
            (tile_width * x, 0),
            &grid,
            tile_width,
            tile_height,
            true,
            false,
        );
        grid.extend(new_tile);
    }

    for x in 0..=4 {
        for y in 1..=4 {
            let new_tile = increment_tile(
                (tile_width * x, tile_height * y),
                &grid,
                tile_width,
                tile_height,
                false,
                x == 0,
            );
            grid.extend(new_tile);
        }
    }

    let max_x = grid.keys().map(|&(x, _)| x).max().unwrap();
    let max_y = grid.keys().map(|&(_, y)| y).max().unwrap();

    let end_coord = (max_x, max_y);

    let risks = dijkstra((0, 0), &grid);

    *risks.get(&end_coord).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 315);
    }
}
