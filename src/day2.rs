use aoc_runner_derive::{aoc, aoc_generator};
use std::panic;

#[aoc_generator(day2)]
pub fn str_to_input(raw_input: &str) -> Vec<(i32, i32, i32)> {
    raw_input.lines()
    .map(|line| {
        let command: Vec<&str> = line.trim().split(" ").collect();
        let direction: &str = command[0];
        let distance: i32 = command[1].parse().unwrap();
        match direction {
            "forward"   => (distance, 0, 0),
            "up"        => (0, -distance, -distance),
            "down"      => (0, distance, distance),
            _           => panic!("invalid command: {}!", direction)
        } 
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(commands: &[(i32, i32, i32)]) -> i32 {
    let final_loc: (i32, i32) = commands
        .iter()
        .fold((0,0), |(oldx, oldy), &(x, y, _)| {
            (oldx + x, oldy + y)
        });
    return final_loc.0 * final_loc.1;
}

#[aoc(day2, part2)]
pub fn part2(commands: &[(i32, i32, i32)]) -> i32 {
    let final_loc: (i32, i32, i32) = commands
        .iter()
        .fold((0,0,0), |(oldx, oldy, oldaim), &(x, _, aim)| {
            return (oldx + x, oldy + (x * oldaim), oldaim + aim);
        });
    return final_loc.0 * final_loc.1;
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn test_generator() {
        assert_eq!(str_to_input(&example), [
            (5, 0, 0),
            (0, 5, 5),
            (8, 0, 0),
            (0, -3, -3),
            (0, 8, 8),
            (2, 0, 0)
        ]);
    }

    #[test]
    fn test_part1() {
        let example_input: Vec<(i32, i32, i32)> = str_to_input(&example);
        assert_eq!(part1(&example_input), 150);
    }

    #[test]
    fn test_part2() {
        let example_input: Vec<(i32, i32, i32)> = str_to_input(&example);
        assert_eq!(part2(&example_input), 900);
    }
}