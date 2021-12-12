use std::collections::{HashMap, VecDeque};

pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 12] Part 1: {}", part_1(&input));
    println!("[DAY 12] Part 2: {}", part_2(&input));
}

fn part_1(input: &'static str) -> u32 {
    let mut grid: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split("-");
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        grid.entry(a).or_insert(Vec::new()).push(b);
        grid.entry(b).or_insert(Vec::new()).push(a);
    }

    let mut queue = VecDeque::new();
    for n in &grid["start"] {
        queue.push_back(vec!["start", n]);
    }

    let mut paths = 0;

    while !queue.is_empty() {
        let current_path = queue.pop_front().unwrap();
        if *current_path.last().unwrap() == "end" {
            paths += 1;
            continue;
        }

        for n in &grid[current_path[current_path.len() - 1]] {
            if !(n.chars().all(|x| x.is_lowercase()) && current_path.contains(n)) {
                let mut tmp = current_path.clone();
                tmp.push(n);
                queue.push_back(tmp);
            }
        }
    }

    paths
}

fn lower_pattern(path: &Vec<&str>) -> bool {
    let mut cnt: HashMap<&str, u16> = HashMap::new();

    for i in path.iter().skip(1) {
        if i.chars().all(|x| x.is_lowercase()) {
            *cnt.entry(i).or_insert(0) += 1;
        }
    }

    cnt.values().filter(|&&x| x > 2).count() == 0 && cnt.values().filter(|&&x| x == 2).count() < 2
}

fn part_2(input: &str) -> usize {
    let mut grid: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split("-");
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        grid.entry(a).or_insert(Vec::new()).push(b);
        grid.entry(b).or_insert(Vec::new()).push(a);
    }

    let mut queue = VecDeque::new();
    for n in &grid["start"] {
        queue.push_back(vec!["start", n]);
    }

    let mut paths = 0;

    while !queue.is_empty() {
        let current_path = queue.pop_front().unwrap();
        if *current_path.last().unwrap() == "end" {
            paths += 1;
            continue;
        }

        for n in &grid[current_path[current_path.len() - 1]] {
            if *n == "start" {
                continue;
            }

            let mut tmp = current_path.clone();
            tmp.push(n);
            if lower_pattern(&tmp) {
                queue.push_back(tmp);
            }
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 226);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 3509);
    }
}
