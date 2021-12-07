use aoc_runner_derive::{aoc, aoc_generator};
use itertools::sorted;
use std::cmp::min;

#[aoc_generator(day7)]
pub fn generator(raw_input: &str) -> Vec<usize> {
    raw_input.split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(positions: &[usize]) -> usize {
    let n = positions.len();
    let sorted_positions = sorted(positions);
    let med = (*sorted_positions[(n as f64 / 2.0).ceil() as usize] as f64 + *sorted_positions[(n as f64 / 2.0).floor() as usize] as f64) / 2.0;
    positions.iter()
        .map(|&x| x as f64)
        .fold(0.0, |dist, x| dist + (med - x).abs()) as usize
}
 
#[aoc(day7, part2)]
pub fn solve_part2(positions: &[usize]) -> usize {
    let mut min_cost = usize::MAX;
    for i in 0..*positions.iter().max().unwrap() {
        let cost = positions.iter()
            .map(|&x| x as isize)
            .fold(0, |cost, x| {
                let abs_diff = (i as isize - x).abs();
                cost + (abs_diff * (abs_diff + 1)) / 2
            }) as usize;
        min_cost = min(cost, min_cost)
    }
    return min_cost;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_generator() {
        let expected = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<usize> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 37);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<usize> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 168);
    }
}