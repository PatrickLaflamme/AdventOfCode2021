use aoc_runner_derive::{aoc, aoc_generator};
use std::println;

#[aoc_generator(day1)]
pub fn str_to_u32(raw_input: &str) -> Vec<u32> {
  raw_input.lines()
    .map(|string_int|{
      string_int.parse().unwrap()
    })
    .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(readings: &[u32]) -> u32 {
  let mut sum: u32 = 0;
  let mut prev = std::u32::MAX;
  for n in readings {
    if n > &prev {
      sum += 1
    }
    prev = *n
  }
  return sum;
}

//#[aoc(day1, part2)]
pub fn solve_part2(readings: &str) -> u32 {
  0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
      let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
      let gen = str_to_u32(input);
      let ans = solve_part1(&gen);
      assert_eq!(ans, 7)
    }

    #[test]
    fn part2_example() {
      let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
      let gen = str_to_u32(input);
      let ans = solve_part2(&gen);
      assert_eq!(ans, 5)
    }
}