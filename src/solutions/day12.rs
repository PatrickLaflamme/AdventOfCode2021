use aoc_runner_derive::{aoc, aoc_generator};

const END: &str = "end";
const START: &str = "start";

#[aoc_generator(day12)]
pub fn generator(raw_input: &str) -> (u8, u8, Vec<(bool, Vec<u8>)>) {
    let mut ids = raw_input.lines()
        .filter(|s| !s.trim().is_empty())
        .flat_map(|s| s.trim().splitn(2, "-"))
        .collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    let start = ids.binary_search(&START).unwrap() as u8;
    let end = ids.binary_search(&END).unwrap() as u8;
    let mut data = ids.iter()
        .map(|s| (&s.to_uppercase() == s, Vec::with_capacity(ids.len())))
        .collect::<Vec<(bool, Vec<u8>)>>();
    raw_input.lines()
        .filter(|s| !s.trim().is_empty())
        .for_each(|s| {
            let nodes: Vec<&str> = s.trim().split("-").collect();
            let first = ids.binary_search(&nodes[0]).unwrap() as u8;
            let second = ids.binary_search(&nodes[1]).unwrap() as u8;
            if second != end && first != start {
                data[second as usize].1.push(first);
            }
            if second != start && first != end {
                data[first as usize].1.push(second);
            }
        });
    (start, end, data)
}

#[aoc(day12, part1)]
pub fn solve_part1(input_data: &(u8, u8, Vec<(bool, Vec<u8>)>)) -> usize {
    let (start, end, nodes) = input_data;
    let mut paths = 0;
    let mut que: Vec<(u8, Vec<bool>)> = Vec::from([(*start, vec![false; nodes.len()])]);
    que.reserve(1028);
    while que.len() > 0 {
        let (node, mut seen) = que.pop().unwrap();
        let (_, next_nodes) = &nodes[node as usize];
        seen[node as usize] = true;
        if next_nodes.contains(end) {
            paths += 1;
        }
        for next in next_nodes {
            if next == end {
                continue;
            }
            let (is_large_cave, _) = &nodes[*next as usize];
            if !seen[*next as usize] || *is_large_cave {
                que.push((*next, seen.clone()))
            }
        }
    }
    paths
}

#[aoc(day12, part2)]
pub fn solve_part2(input_data: &(u8, u8, Vec<(bool, Vec<u8>)>)) -> usize {
    let (start, end, nodes) = input_data;
    let mut paths = 0;
    let mut que: Vec<(u8, Vec<bool>, bool)> = Vec::from([(*start, vec![false; nodes.len()], false)]);
    que.reserve(32768);
    while que.len() > 0 {
        let (node, mut seen, doubled) = que.pop().unwrap();
        let (_, next_nodes) = &nodes[node as usize];
        seen[node as usize] = true;
        if next_nodes.contains(end) {
            paths += 1;
        }
        for next in next_nodes {
            if next == end {
                continue;
            }
            let (is_large_cave, _) = &nodes[*next as usize];
            if !seen[*next as usize] || *is_large_cave {
                que.push((*next, seen.clone(), doubled))
            } else if !doubled || *is_large_cave {
                que.push((*next, seen.clone(), true))
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
        let expected = vec![
            (true, vec![2, 1, 4]),
            (false, vec![0, 3, 4]),
            (false, vec![0]),
            (false, vec![1]),
            (false, vec![]),
            (false, vec![0,1])
        ];
        assert_eq!(generator(&EXAMPLE1), (5, 4, expected));
    }

    #[test]
    fn test_solve_part1_example1() {
        let example: (u8, u8, Vec<(bool, Vec<u8>)>) = generator(&EXAMPLE1);
        assert_eq!(solve_part1(&example), 10);
    }

    #[test]
    fn test_solve_part1_example2() {
        let example: (u8, u8, Vec<(bool, Vec<u8>)>) = generator(&EXAMPLE2);
        assert_eq!(solve_part1(&example), 19);
    }

    #[test]
    fn test_solve_part1_example3() {
        let example: (u8, u8, Vec<(bool, Vec<u8>)>) = generator(&EXAMPLE3);
        assert_eq!(solve_part1(&example), 226);
    }

    #[test]
    fn test_solve_part2() {
        let example: (u8, u8, Vec<(bool, Vec<u8>)>) = generator(&EXAMPLE1);
        assert_eq!(solve_part2(&example), 36);
    }
}