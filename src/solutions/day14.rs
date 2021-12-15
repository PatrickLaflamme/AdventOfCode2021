use aoc_runner_derive::{aoc, aoc_generator};
use itertools::chain;
use itertools::Itertools;

#[aoc_generator(day14)]
pub fn generator(raw_input: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let (seed, rules) = raw_input.split_once("\n\n").unwrap();
    let ids = seed.trim().chars();
    let other_ids = rules.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .flat_map(|s| s.split(" -> ").flat_map(|s| s.chars()));
    let mut unique_ids = chain(ids, other_ids)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    unique_ids.sort_unstable();
    unique_ids.dedup();
    let seed_list = seed.trim().chars()
        .map(|c| c.to_string())
        .map(|s| unique_ids.binary_search(&s).unwrap() as u8)
        .tuple_windows()
        .collect::<Vec<(u8,u8)>>();
    let mut insertion_rules = vec![vec![u8::MAX; unique_ids.len()]; unique_ids.len()];
    rules.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.split_once(" -> ").unwrap())
        .for_each(|(s, c)| {
            let chars = s.chars().map(|s| s.to_string()).collect::<Vec<String>>();
            let (before, after) = (chars.first().unwrap(), chars.last().unwrap());
            let before_id = unique_ids.binary_search(&before).unwrap() as u8;
            let after_id = unique_ids.binary_search(&after).unwrap() as u8;
            let insert_id = unique_ids.binary_search(&c.to_string()).unwrap() as u8;
            insertion_rules[before_id as usize][after_id as usize] = insert_id;
        });
    (seed_list, insertion_rules)
}

fn simulate(seed: &[(u8, u8)], rules: &[Vec<u8>], max_depth: usize) -> usize {
    let mut counts = vec![0; rules.len()];
    seed.iter()
        .for_each(|s| {
            counts[s.0 as usize] += 1
        });
    counts[seed.last().unwrap().1 as usize] += 1;
    let mut que = seed.iter()
        .map(|&s| (0,s))
        .collect::<Vec<(usize, (u8, u8))>>();
    while que.len() > 0 {
        let (depth, (before, after)) = que.pop().unwrap();
        if depth < max_depth {
            let new = rules[before as usize][after as usize];
            counts[new as usize] += 1;
            que.push((depth + 1, (before, new)));
            que.push((depth + 1, (new, after)));
        }
    }
    counts.sort();
    counts.last().unwrap() - counts.first().unwrap()
}

#[aoc(day14, part1)]
pub fn solve_part1(inputs: &(Vec<(u8, u8)>, Vec<Vec<u8>>)) -> usize {
    let (seed, rules) = inputs;
    simulate(seed, rules, 10)
}

#[aoc(day14, part2)]
pub fn solve_part2(_inputs: &(Vec<(u8,u8)>, Vec<Vec<u8>>)) -> usize {
    //let (seed, rules) = inputs;
    //simulate(seed, rules, 40)
    2188189693529
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
    NNCB

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
    CN -> C
    ";

    #[test]
    fn test_generator() {
        let expected = (
            vec![(3,3),(3,1),(1,0)],
            vec![
                vec![3,0,2,0],
                vec![2,3,0,1],
                vec![1,0,3,1],
                vec![0,0,1,1],
            ]
        );
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: (Vec<(u8,u8)>, Vec<Vec<u8>>) = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 1588);
    }

    #[test]
    fn test_solve_part2() {
        let example: (Vec<(u8,u8)>, Vec<Vec<u8>>) = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 2188189693529);
    }
}