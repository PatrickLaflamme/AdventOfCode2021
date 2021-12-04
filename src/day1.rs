use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
  let sum: u32 = 0;
  let prev: u32 = std::u32::MAX;
  for n in input {
    if n > &prev {
      sum += 1
    }
  }
  sum
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {}


#[cfg(test)]
mod tests {
    use super::*;
}