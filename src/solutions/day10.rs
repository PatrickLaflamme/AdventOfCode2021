use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

lazy_static! {
    static ref CHARMAP: HashMap<&'static str, &'static str> = vec![
        ("{", "}"),
        ("[", "]"),
        ("(", ")"),
        ("<", ">")
    ].into_iter().collect();

    static ref CORRUPT_POINTS: HashMap<&'static str, usize> = vec![
        (")", 3),
        ("]", 57),
        ("}", 1197),
        (">", 25137)
    ].into_iter().collect();

    static ref COMPLETE_POINTS: HashMap<&'static str, usize> = vec![
        (")", 1),
        ("]", 2),
        ("}", 3),
        (">", 4)
    ].into_iter().collect();
}

#[aoc_generator(day10)]
pub fn generator(raw_input: &str) -> Vec<Vec<String>> {
    raw_input.lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            s.trim()
                .split("")
                .filter(|s2| !s2.is_empty())
                .map(|s2| s2.to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}

fn analyze_line(line: &[String]) -> (usize, Option<Vec<String>>) {
    let mut stack = Vec::<String>::new();
    for c in line {
        let mut next_close: &str = "";
        if stack.len() > 0 { 
            next_close = stack.last().unwrap();
        }
        if next_close == c {
            stack.pop();
        } else if CHARMAP.contains_key(&c as &str) {
            stack.push(CHARMAP.get(&c as &str).unwrap().to_string());
        } else {
            let corrupt_score = CORRUPT_POINTS.get(&c as &str).unwrap();
            return (*corrupt_score, None);
        }
    }
    return (0, Some(stack));
}

fn score_incomplete_lines(line: &[String]) -> usize {
    let (_, incomplete_vec) = analyze_line(line);
    match incomplete_vec {
        Some(chars) => chars.iter()
            .rev()
            .map(|s| COMPLETE_POINTS.get(&s as &str).unwrap())
            .fold(0, |score, x| {
                (score * 5) + x
            }),
        None => 0
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(lines: &[Vec<String>]) -> usize {
    lines.iter()
        .map(|line| analyze_line(line).0)
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(lines: &[Vec<String>]) -> usize {
    let mut scores: Vec<usize> = lines.iter()
        .map(|line| score_incomplete_lines(line))
        .filter(|score| score > &0)
        .collect::<Vec<usize>>();
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    ";

    #[test]
    fn test_generator() {
        let expected = vec![
            vec!["[", "(", "{", "(", "<", "(", "(", ")", ")", "[", "]", ">", "[", "[", "{", "[", "]", "{", "<", "(", ")", "<", ">", ">"],
            vec!["[", "(", "(", ")", "[", "<", ">", "]", ")", "]", "(", "{", "[", "<", "{", "<", "<", "[", "]", ">", ">", "("],
            vec!["{", "(", "[", "(", "<", "{", "}", "[", "<", ">", "[", "]", "}", ">", "{", "[", "]", "{", "[", "(", "<", "(", ")", ">"],
            vec!["(", "(", "(", "(", "{", "<", ">", "}", "<", "{", "<", "{", "<", ">", "}", "{", "[", "]", "{", "[", "]", "{", "}"],
            vec!["[", "[", "<", "[", "(", "[", "]", ")", ")", "<", "(", "[", "[", "{", "}", "[", "[", "(", ")", "]", "]", "]"],
            vec!["[", "{", "[", "{", "(", "{", "}", "]", "{", "}", "}", "(", "[", "{", "[", "{", "{", "{", "}", "}", "(", "[", "]"],
            vec!["{", "<", "[", "[", "]", "]", ">", "}", "<", "{", "[", "{", "[", "{", "[", "]", "{", "(", ")", "[", "[", "[", "]"],
            vec!["[", "<", "(", "<", "(", "<", "(", "<", "{", "}", ")", ")", ">", "<", "(", "[", "]", "(", "[", "]", "(", ")"],
            vec!["<", "{", "(", "[", "(", "[", "[", "(", "<", ">", "(", ")", ")", "{", "}", "]", ">", "(", "<", "<", "{", "{"],
            vec!["<", "{", "(", "[", "{", "{", "}", "}", "[", "<", "[", "[", "[", "<", ">", "{", "}", "]", "]", "]", ">", "[", "]", "]"]
        ];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<Vec<String>> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 26397);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<Vec<String>> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 288957);
    }
}