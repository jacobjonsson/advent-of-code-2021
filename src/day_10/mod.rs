pub fn run() {
    let input = include_str!("./input.txt");

    println!("[DAY 10] Part 1: {}", part_1(&input));
    println!("[DAY 10] Part 2: {}", part_2(&input));
}

fn is_closing(ch: char) -> bool {
    match ch {
        ')' | ']' | '}' | '>' => true,
        _ => false,
    }
}

fn open_to_closing(ch: char) -> char {
    match ch {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn char_score(ch: char) -> usize {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn check_line(input: &str) -> Result<Vec<char>, char> {
    let mut chars = input.chars();

    let mut stack = vec![chars.next().unwrap()];

    for char in chars {
        if is_closing(char) {
            let expected = match stack.last().unwrap() {
                '(' => ')',
                '{' => '}',
                '[' => ']',
                '<' => '>',
                _ => unreachable!(),
            };

            if char == expected {
                stack.pop();
            } else {
                return Err(char);
            }
        } else {
            stack.push(char);
        }
    }

    Ok(stack)
}

fn part_1(input: &str) -> usize {
    let mut scores = 0;

    for line in input.lines() {
        if let Err(ch) = check_line(line) {
            scores += char_score(ch);
        }
    }

    scores
}

fn part_2(input: &str) -> usize {
    let mut scores: Vec<usize> = Vec::new();

    for line in input.lines() {
        if let Ok(stack) = check_line(line) {
            scores.push(
                stack
                    .into_iter()
                    .rev()
                    .map(open_to_closing)
                    .map(|ch| match ch {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    })
                    .fold(0, |acc, x| acc * 5 + x),
            );
        }
    }

    assert_ne!(scores.len() % 2, 0);
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 26397);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 288957);
    }
}
