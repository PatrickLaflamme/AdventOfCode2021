use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const END: &str = "end";
const START: &str = "start";

#[aoc_generator(day12)]
pub fn generator(raw_input: &str) -> HashMap<String, HashSet<String>> {
    let mut data = HashMap::<String, HashSet<String>>::new();
    raw_input.lines()
        .filter(|s| !s.trim().is_empty())
        .for_each(|s| {
            let nodes: Vec<&str> = s.trim().split("-").collect();
            let first = nodes[0].to_string();
            let second = nodes[1].to_string();
            if second != END && first != START {
                data.entry(second.to_string()).or_insert(HashSet::new()).insert(first.to_string());
            }
            if second != START && first != END {
                data.entry(first.to_string()).or_insert(HashSet::new()).insert(second.to_string());
            }
        });
    data
}

#[aoc(day12, part1)]
pub fn solve_part1(nodes: &HashMap<String, HashSet<String>>) -> usize {
    let mut paths = 0;
    let mut que: VecDeque<(String, HashSet<String>)> = VecDeque::from([(START.to_string(), HashSet::new())]);
    while que.len() > 0 {
        let (node, mut seen) = que.pop_front().unwrap();
        seen.insert(node.to_string());
        let next_nodes = nodes.get(&node).unwrap();
        if next_nodes.contains(END) {
            paths += 1;
        }
        for next in next_nodes {
            if (!seen.contains(next) || &next.to_uppercase() == next) && next != END {
                que.push_back((next.to_string(), seen.clone()))
            }
        }
    }
    paths
}

#[aoc(day12, part2)]
pub fn solve_part2(nodes: &HashMap<String, HashSet<String>>) -> usize {
    let mut paths = 0;
    let mut que: VecDeque<(String, HashSet<String>, bool)> = VecDeque::from([(START.to_string(), HashSet::new(), false)]);
    while que.len() > 0 {
        let (node, mut seen, doubled) = que.pop_front().unwrap();
        seen.insert(node.to_string());
        let next_nodes = nodes.get(&node).unwrap();
        if next_nodes.contains(END) {
            paths += 1;
        }
        for next in next_nodes {
            if (!seen.contains(next) || &next.to_uppercase() == next) && next != END {
                que.push_back((next.to_string(), seen.clone(), doubled))
            } else if (!doubled || &next.to_uppercase() == next) && next != END {
                que.push_back((next.to_string(), seen.clone(), true))
            }
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ";

    const EXAMPLE2: &str = "
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    ";

    const EXAMPLE3: &str = "
        fs-end
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
        start-RW
    ";

    #[test]
    fn test_generator() {
        let expected = [
            ("start", vec!["A", "b"]),
            ("A", vec!["c", "b", "end"]),
            ("b", vec!["A", "d", "end"]),
            ("c", vec!["A"]),
            ("d", vec!["b"])
        ].iter()
            .map(|(k, v)| (k.to_string(), v.iter().map(|s| s.to_string()).collect::<HashSet<String>>()))
            .collect::<HashMap<String, HashSet<String>>>();
        assert_eq!(generator(&EXAMPLE1), expected);
    }

    #[test]
    fn test_solve_part1_example1() {
        let example: HashMap<String, HashSet<String>> = generator(&EXAMPLE1);
        assert_eq!(solve_part1(&example), 10);
    }

    #[test]
    fn test_solve_part1_example2() {
        let example: HashMap<String, HashSet<String>> = generator(&EXAMPLE2);
        assert_eq!(solve_part1(&example), 19);
    }

    #[test]
    fn test_solve_part1_example3() {
        let example: HashMap<String, HashSet<String>> = generator(&EXAMPLE3);
        assert_eq!(solve_part1(&example), 226);
    }

    #[test]
    fn test_solve_part2() {
        let example: HashMap<String, HashSet<String>> = generator(&EXAMPLE1);
        assert_eq!(solve_part2(&example), 36);
    }
}