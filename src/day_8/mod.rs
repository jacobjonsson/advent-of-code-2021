#![allow(dead_code)]
use std::collections::HashSet;

pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 7] Part 1: {}", part_1(&input));
    println!("[DAY 7] Part 2: {}", part_2(&input));
}

fn is_easy_digit(p: &str) -> Option<u8> {
    match p.len() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => return None,
    }
}

type Entry = (Vec<HashSet<char>>, Vec<HashSet<char>>);

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split('|');
            (
                iter.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.chars().collect())
                    .collect::<Vec<_>>(),
                iter.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.chars().collect())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|(_, b)| b)
        .flatten()
        .filter(|&digit| {
            digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7
        })
        .count()
}

fn part_2(input: &str) -> u32 {
    let entries = parse_input(input);

    let mut result = 0;

    for entry in entries {
        let mut signal_patterns = entry.0.to_owned();
        let outputs = entry.1.to_owned();

        let mut patterns = vec![None; 10];

        let idx = signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 2)
            .unwrap()
            .0;

        patterns[1] = Some(signal_patterns.swap_remove(idx));

        let idx = signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 4)
            .unwrap()
            .0;

        patterns[4] = Some(signal_patterns.swap_remove(idx));

        let idx = signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 3)
            .unwrap()
            .0;

        patterns[7] = Some(signal_patterns.swap_remove(idx));

        let idx = signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 7)
            .unwrap()
            .0;

        patterns[8] = Some(signal_patterns.swap_remove(idx));

        let idx = signal_patterns
            .iter()
            .enumerate()
            .filter(|(_, pattern)| pattern.len() == 6)
            .find(|(_, pattern)| {
                patterns[4]
                    .as_ref()
                    .unwrap()
                    .difference(pattern)
                    .next()
                    .is_none()
            })
            .unwrap()
            .0;

        patterns[9] = Some(signal_patterns.swap_remove(idx));

        let idx = signal_patterns
            .iter()
            .enumerate()
            .filter(|(_, pattern)| pattern.len() == 6)
            .find(|(_, pattern)| {
                patterns[7]
                    .as_ref()
                    .unwrap()
                    .difference(pattern)
                    .next()
                    .is_none()
            })
            .unwrap()
            .0;

        patterns[0] = Some(signal_patterns.swap_remove(idx));

        let idx = signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 6)
            .unwrap()
            .0;

        patterns[6] = Some(signal_patterns.swap_remove(idx));

        let idx = signal_patterns
            .iter()
            .enumerate()
            .filter(|(_, pattern)| pattern.len() == 5)
            .find(|(_, pattern)| {
                pattern
                    .difference(patterns[6].as_ref().unwrap())
                    .next()
                    .is_none()
            })
            .unwrap()
            .0;

        patterns[5] = Some(signal_patterns.swap_remove(idx));

        let idx = signal_patterns
            .iter()
            .enumerate()
            .filter(|(_, pattern)| pattern.len() == 5)
            .find(|(_, pattern)| {
                pattern
                    .difference(patterns[9].as_ref().unwrap())
                    .next()
                    .is_none()
            })
            .unwrap()
            .0;

        patterns[3] = Some(signal_patterns.swap_remove(idx));

        patterns[2] = signal_patterns.into_iter().next();

        result += outputs.iter().fold(0, |acc, digit| {
            acc * 10
                + patterns
                    .iter()
                    .enumerate()
                    .find(|(_, pattern)| pattern.as_ref().unwrap() == digit)
                    .unwrap()
                    .0 as u32
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(part_1(input), 26);
    }

    #[test]
    fn test_part_2() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
        assert_eq!(part_2(input), 61229);
    }
}
