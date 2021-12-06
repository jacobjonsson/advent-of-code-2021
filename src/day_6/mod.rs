use std::collections::HashMap;

pub fn run() {
    let input = include_str!("./input.txt");

    let part_1 = part_1(&input, 80);
    let part_2 = part_2(&input, 256);

    println!("[DAY 6] Part 1: {}", part_1);
    println!("[DAY 6] Part 2: {}", part_2);
}

#[derive(Debug)]
struct LanternFishPool {
    fishes: Vec<(u8, u64)>,
}

impl LanternFishPool {
    fn new(input: &str) -> Self {
        let mut fish_map: HashMap<u8, u64> = HashMap::new();

        for state in input.trim().split(",").filter_map(|x| x.parse::<u8>().ok()) {
            match fish_map.get_mut(&state) {
                Some(count) => (*count += 1),
                None => {
                    fish_map.insert(state, 1);
                }
            };
        }

        Self {
            fishes: fish_map.iter().map(|(k, v)| (*k, *v)).collect(),
        }
    }

    fn increment(&mut self) {
        let new_fishes = self
            .fishes
            .iter()
            .filter(|(s, _)| *s == 0)
            .map(|(_, v)| v)
            .sum();

        let mut new_state: Vec<_> = self
            .fishes
            .iter()
            .map(|(s, c)| if *s == 0 { (6, *c) } else { (s - 1, *c) })
            .collect();

        new_state.push((8, new_fishes));

        // Balance the fish array to prevent it from growing
        let mut fish_map: HashMap<u8, u64> = HashMap::new();
        for (state, count) in new_state {
            match fish_map.get_mut(&state) {
                Some(c) => (*c += count),
                None => {
                    fish_map.insert(state, count);
                }
            };
        }

        self.fishes = fish_map.iter().map(|(k, v)| (*k, *v)).collect();
    }

    fn count(&self) -> u64 {
        self.fishes.iter().map(|(_, c)| *c).sum()
    }
}

fn part_1(input: &str, days: u8) -> u64 {
    let mut fish_pool = LanternFishPool::new(input);

    for _ in 0..days {
        fish_pool.increment();
    }

    fish_pool.count()
}

fn part_2(input: &str, days: u16) -> u64 {
    let mut fish_pool = LanternFishPool::new(input);

    for _ in 0..days {
        fish_pool.increment();
    }

    fish_pool.count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "3,4,3,1,2";

        assert_eq!(part_1(&input, 80), 5934);
    }

    #[test]
    fn test_part_2() {
        let input = "3,4,3,1,2";

        assert_eq!(part_2(&input, 256), 26984457539);
    }
}
