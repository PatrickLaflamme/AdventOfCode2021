use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(dayx)]
pub fn generator(raw_input: &str) -> Vec<u32> {
    vec![0; 0]
}

#[aoc(dayx, part1)]
pub fn solve_part1(readings: &[u32]) -> u32 {
    0
}

#[aoc(dayx, part2)]
pub fn solve_part2(readings: &[u32]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_generator() {
        let expected = vec![0; 1];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: u32 = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 0);
    }

    #[test]
    fn test_solve_part2() {
        let example: u32 = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 0);
    }
}