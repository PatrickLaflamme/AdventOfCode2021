use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[aoc_generator(day6)]
pub fn generator(raw_input: &str) -> Vec<usize> {
    raw_input.split(",")
        .map(|s| s.parse().unwrap())
        .fold(vec![0; 9], |mut fish, x: usize| {
            fish[x] += 1;
            fish
        })
}

fn simulate(fish_count: &[usize], days: usize) -> usize {
    let mut fish_queue = fish_count
        .iter()
        .fold(VecDeque::new(), |mut dequeue, &x| {
            dequeue.push_back(x); 
            dequeue
        });

    for _ in 0..days {
        let doubling_fish = fish_queue.pop_front().unwrap();
        fish_queue[6] += doubling_fish;
        fish_queue.push_back(doubling_fish);
    }

    fish_queue.iter()
        .fold(0, |sum, &x| sum + x)
}

#[aoc(day6, part1)]
pub fn solve_part1(fish_count: &[usize]) -> usize {
    simulate(fish_count, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(fish_count: &[usize]) -> usize {
    simulate(fish_count, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_generator() {
        let expected = vec![0, 1, 1, 2, 1, 0, 0, 0, 0];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 5934);
    }

    #[test]
    fn test_solve_part2() {
        let example = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 26984457539);
    }
}