use aoc_runner_derive::{aoc, aoc_generator};
use itertools::sorted;
use num::signum;
use std::cmp::min;

#[aoc_generator(day7)]
pub fn generator(raw_input: &str) -> Vec<isize> {
    raw_input.split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(positions: &[isize]) -> isize {
    let n = positions.len();
    let sorted_positions = sorted(positions);
    let med = (*sorted_positions[(n as f64 / 2.0).ceil() as usize] as f64 + *sorted_positions[(n as f64 / 2.0).floor() as usize] as f64) / 2.0;
    positions.iter()
        .map(|&x| x as f64)
        .fold(0.0, |dist, x| dist + (med - x).abs()) as isize
}
 
#[aoc(day7, part2)]
pub fn solve_part2(positions: &[isize]) -> isize {
    let mut min_cost = isize::MAX;
    let sorted_positions = sorted(positions);
    let mut i = positions.iter().sum::<isize>() / positions.len() as isize;
    let direction = signum((sorted_positions[sorted_positions.len() - 1]- i) - (i - sorted_positions[0]));
    loop {
        let cost = positions.iter()
            .fold(0, |cost, x| {
                let abs_diff = (i - x).abs();
                cost + (abs_diff * (abs_diff + 1)) / 2
            }) as isize;
        if min_cost != isize::MAX && min_cost < cost {
            break;
        } 
        i = i + direction;
        min_cost = min(cost, min_cost);
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
        let example: Vec<isize> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 37);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<isize> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 168);
    }
}