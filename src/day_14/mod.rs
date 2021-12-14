use counter::Counter;
use std::collections::HashMap;

pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 14] Part 1: {}", part_1(&input));
    println!("[DAY 14] Part 2: {}", part_2(&input));
}

struct Polymer {
    formula: Counter<(char, char), u64>,
    rules: HashMap<(char, char), char>,
    elements: Counter<char, u64>,
}

impl Polymer {
    fn new(template: Vec<char>, rules: Vec<(char, char, char)>) -> Self {
        Polymer {
            formula: template.windows(2).map(|w| (w[0], w[1])).collect(),
            rules: rules.into_iter().map(|(a, b, c)| ((a, b), c)).collect(),
            elements: template.into_iter().collect(),
        }
    }
}

impl Iterator for Polymer {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut f = self.formula.clone();

        for (&(a, b), &n) in self.formula.iter() {
            if let Some(&i) = self.rules.get(&(a, b)) {
                *f.entry((a, b)).or_default() -= n;
                *f.entry((a, i)).or_default() += n;
                *f.entry((i, b)).or_default() += n;
                *self.elements.entry(i).or_default() += n;
            }
        }

        self.formula = f;
        let min = self.elements.values().min().unwrap();
        let max = self.elements.values().max().unwrap();
        Some(max - min)
    }
}

fn parse_input(input: &str) -> (Vec<char>, Vec<(char, char, char)>) {
    let mut lines = input.lines();

    let template: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let rules: Vec<(char, char, char)> = lines
        .map(|line| {
            let mut parts = line.split(" -> ");
            let mut a = parts.next().unwrap().chars();
            let aa = a.next().unwrap();
            let ab = a.next().unwrap();
            let b = parts.next().unwrap().chars().next().unwrap();
            (aa, ab, b)
        })
        .collect();

    (template, rules)
}

fn part_1(input: &str) -> u64 {
    let (template, rules) = parse_input(input);
    Polymer::new(template, rules).nth(9).unwrap()
}

fn part_2(input: &str) -> u64 {
    let (template, rules) = parse_input(input);
    Polymer::new(template, rules).nth(39).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1588);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 2188189693529);
    }
}
