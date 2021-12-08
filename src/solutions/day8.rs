use aoc_runner_derive::{aoc, aoc_generator};
use itertools::sorted;
use std::collections::HashMap;

#[aoc_generator(day8)]
pub fn generator(raw_input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    raw_input.lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let mut it = s.trim().split(" | ")
                .map(|s| {
                    s.trim().split(" ")
                        .map(|s| s.to_string())
                        .collect()
                });
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(readings: &[(Vec<String>, Vec<String>)]) -> usize {
    let unique_seg_len = vec![2,3,4,7];
    readings.iter()
        .map(|i| &i.1)
        .map(|display| {
            display.iter()
                .filter(|d| unique_seg_len.contains(&d.len()))
                .collect::<Vec<_>>()
                .len()
        })
        .sum()

}

fn solve_single(digits: &[String], display: &[String]) -> usize {
    let mut work_map: HashMap<String, Vec<usize>> = HashMap::new();
    let mut ans_map: HashMap<usize, String> = HashMap::new();
    digits.iter()
        .map(|d| sorted(d.chars()).iter().collect::<String>())
        .for_each(|d| {
            let digits = match d.len() {
                2 => vec![1],
                3 => vec![7],
                4 => vec![4],
                5 => vec![2,3,5],
                6 => vec![0,6,9],
                7 => vec![8],
                _ => panic!("something went very wrong!")
            };
            if digits.len() == 1 {
                ans_map.insert(digits[0], d.to_string());
            } else {
                work_map.insert(d.to_string(), digits);
            }
        });
    work_map.iter()
        .for_each(|(d, possibles)| {
            let filtered_possibles: Vec<usize> = possibles.iter()
                .filter(|d| !ans_map.contains_key(d))
                .map(|&d| d)
                .collect();
            if filtered_possibles.len() == 1 {
                ans_map.insert(filtered_possibles[0], d.to_string());
                return;
            }
            let n_shared_1_chars = d.chars().filter(|c| ans_map[&1].contains(&c.to_string())).collect::<Vec<_>>().len();
            let n_shared_4_chars = d.chars().filter(|c| ans_map[&4].contains(&c.to_string())).collect::<Vec<_>>().len();
            if n_shared_1_chars == 1 && filtered_possibles.contains(&6) {
                ans_map.insert(6,  d.to_string());
            } else if n_shared_1_chars == 2 && filtered_possibles.contains(&3) {
                ans_map.insert(3,  d.to_string());
            } else if n_shared_4_chars == 4 && filtered_possibles.contains(&9) {
                ans_map.insert(9,  d.to_string());
            } else if n_shared_4_chars == 3 && filtered_possibles.contains(&5) {
                ans_map.insert(5,  d.to_string());
            } else if n_shared_1_chars == 2 && filtered_possibles.contains(&0) {
                ans_map.insert(0,  d.to_string());
            } else if n_shared_4_chars == 2 && filtered_possibles.contains(&2) {
                ans_map.insert(2,  d.to_string());
            } else {
                println!("{}, {:?}, 4: {}, 1: {}", d, filtered_possibles, n_shared_4_chars, n_shared_1_chars);
                panic!("shoudn't get here");
            }
        });
    let flipped_ans_map: HashMap<String, usize> = ans_map.iter()
        .map(|(&k, v)| (v.to_string(), k))
        .collect();
    display.iter()
        .map(|d| sorted(d.chars()).iter().collect::<String>())
        .map(|d| {
            flipped_ans_map.get(&d).unwrap().to_string()
        })
        .fold(String::with_capacity(4), |mut s, d| {s.push_str(&d); s})
        .parse()
        .unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(readings: &[(Vec<String>, Vec<String>)]) -> usize {
    readings.iter()
        .map(|(digits, display)| solve_single(digits, display))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
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

    #[test]
    fn test_generator() {
        let expected: Vec<(Vec<String>, Vec<String>)> = vec![
            (vec!["be".to_string(), "cfbegad".to_string(), "cbdgef".to_string(), "fgaecd".to_string(), "cgeb".to_string(), "fdcge".to_string(), "agebfd".to_string(), "fecdb".to_string(), "fabcd".to_string(), "edb".to_string()], vec!["fdgacbe".to_string(), "cefdb".to_string(), "cefbgd".to_string(), "gcbe".to_string()]),
            (vec!["edbfga".to_string(), "begcd".to_string(), "cbg".to_string(), "gc".to_string(), "gcadebf".to_string(), "fbgde".to_string(), "acbgfd".to_string(), "abcde".to_string(), "gfcbed".to_string(), "gfec".to_string()], vec!["fcgedb".to_string(), "cgb".to_string(), "dgebacf".to_string(), "gc".to_string()]),
            (vec!["fgaebd".to_string(), "cg".to_string(), "bdaec".to_string(), "gdafb".to_string(), "agbcfd".to_string(), "gdcbef".to_string(), "bgcad".to_string(), "gfac".to_string(), "gcb".to_string(), "cdgabef".to_string()], vec!["cg".to_string(), "cg".to_string(), "fdcagb".to_string(), "cbg".to_string()]),
            (vec!["fbegcd".to_string(), "cbd".to_string(), "adcefb".to_string(), "dageb".to_string(), "afcb".to_string(), "bc".to_string(), "aefdc".to_string(), "ecdab".to_string(), "fgdeca".to_string(), "fcdbega".to_string()], vec!["efabcd".to_string(), "cedba".to_string(), "gadfec".to_string(), "cb".to_string()]),
            (vec!["aecbfdg".to_string(), "fbg".to_string(), "gf".to_string(), "bafeg".to_string(), "dbefa".to_string(), "fcge".to_string(), "gcbea".to_string(), "fcaegb".to_string(), "dgceab".to_string(), "fcbdga".to_string()], vec!["gecf".to_string(), "egdcabf".to_string(), "bgf".to_string(), "bfgea".to_string()]),
            (vec!["fgeab".to_string(), "ca".to_string(), "afcebg".to_string(), "bdacfeg".to_string(), "cfaedg".to_string(), "gcfdb".to_string(), "baec".to_string(), "bfadeg".to_string(), "bafgc".to_string(), "acf".to_string()], vec!["gebdcfa".to_string(), "ecba".to_string(), "ca".to_string(), "fadegcb".to_string()]),
            (vec!["dbcfg".to_string(), "fgd".to_string(), "bdegcaf".to_string(), "fgec".to_string(), "aegbdf".to_string(), "ecdfab".to_string(), "fbedc".to_string(), "dacgb".to_string(), "gdcebf".to_string(), "gf".to_string()], vec!["cefg".to_string(), "dcbef".to_string(), "fcge".to_string(), "gbcadfe".to_string()]),
            (vec!["bdfegc".to_string(), "cbegaf".to_string(), "gecbf".to_string(), "dfcage".to_string(), "bdacg".to_string(), "ed".to_string(), "bedf".to_string(), "ced".to_string(), "adcbefg".to_string(), "gebcd".to_string()], vec!["ed".to_string(), "bcgafe".to_string(), "cdgba".to_string(), "cbgef".to_string()]),
            (vec!["egadfb".to_string(), "cdbfeg".to_string(), "cegd".to_string(), "fecab".to_string(), "cgb".to_string(), "gbdefca".to_string(), "cg".to_string(), "fgcdab".to_string(), "egfdb".to_string(), "bfceg".to_string()], vec!["gbdfcae".to_string(), "bgc".to_string(), "cg".to_string(), "cgb".to_string()]),
            (vec!["gcafb".to_string(), "gcf".to_string(), "dcaebfg".to_string(), "ecagb".to_string(), "gf".to_string(), "abcdeg".to_string(), "gaef".to_string(), "cafbge".to_string(), "fdbac".to_string(), "fegbdc".to_string()], vec!["fgae".to_string(), "cfgab".to_string(), "fg".to_string(), "bagce".to_string()])
        ];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<(Vec<String>, Vec<String>)> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 26);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<(Vec<String>, Vec<String>)> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 61229);
    }
}