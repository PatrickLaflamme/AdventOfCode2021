use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Line {
    x1: usize, 
    y1: usize, 
    x2: usize, 
    y2: usize
}

#[aoc_generator(day5)]
pub fn to_lines(raw_input: &str) -> (usize, usize, Vec<Line>) {
    let lines: Vec<Line> = raw_input.lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let points: Vec<Vec<usize>> = s.trim().split(" -> ")
                .map(|s| {
                    s.split(",")
                        .map(|s2| s2.trim().parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            Line {
                x1: points[0][0],
                y1: points[0][1],
                x2: points[1][0],
                y2: points[1][1]
            }
        })
        .collect();
    let x_size = lines.iter()
        .map(|l| max(l.x1, l.x2))
        .fold(0, |max_val, x| { max(max_val, x) });
    let y_size = lines.iter()
        .map(|l| max(l.y1, l.y2))
        .fold(0, |max_val, x| { max(max_val, x) });
    (x_size, y_size, lines)
}

fn simulate(x_size: usize, y_size: usize, lines: Vec<Line>) -> usize {
    let mut space: Vec<Vec<usize>> = vec![vec![0; x_size + 1]; y_size + 1];
    for line in lines {
        let mut x1: usize = line.x1;
        let mut y1: usize = line.y1;
        space[y1][x1] += 1;
        loop {
            if x1 > line.x2 {
                x1 -= 1
            } else if x1 < line.x2 {
                x1 += 1
            }
            if y1 > line.y2 {
                y1 -= 1
            } else if y1 < line.y2 {
                y1 += 1
            }
            space[y1][x1] += 1;
            if x1 == line.x2 && y1 == line.y2 {
                break;
            }
        }
    }
    space
        .iter()
        .flat_map(|x| x)
        .filter(|x| *x > &1)
        .collect::<Vec<&usize>>()
        .len()
}


#[aoc(day5, part1)]
pub fn part1(input_data: &(usize, usize, Vec<Line>)) -> usize {
    let (x_size, y_size, lines) = input_data.clone();
    let filtered_lines = lines.into_iter()
        .filter(|line| {
            // filter to only horizontal/vertical lines
            line.x1 == line.x2 || line.y1 == line.y2
        })
        .clone()
        .collect();
    simulate(x_size, y_size, filtered_lines)
}

#[aoc(day5, part2)]
pub fn part2(input_data: &(usize, usize, Vec<Line>)) -> usize {
    let (x_size, y_size, lines) = input_data.clone();
    simulate(x_size, y_size, lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";

    #[test]
    fn test_to_lines() {
        let expected = vec![
            Line{x1: 0, y1: 9, x2: 5, y2: 9},
            Line{x1: 8, y1: 0, x2: 0, y2: 8},
            Line{x1: 9, y1: 4, x2: 3, y2: 4},
            Line{x1: 2, y1: 2, x2: 2, y2: 1},
            Line{x1: 7, y1: 0, x2: 7, y2: 4},
            Line{x1: 6, y1: 4, x2: 2, y2: 0},
            Line{x1: 0, y1: 9, x2: 2, y2: 9},
            Line{x1: 3, y1: 4, x2: 1, y2: 4},
            Line{x1: 0, y1: 0, x2: 8, y2: 8},
            Line{x1: 5, y1: 5, x2: 8, y2: 2}
        ];
        assert_eq!(to_lines(&EXAMPLE), (9, 9, expected));
    }

    #[test]
    fn test_part1() {
        let example = to_lines(&EXAMPLE);
        assert_eq!(part1(&example), 5);
    }

    #[test]
    fn test_part2() {
        let example = to_lines(&EXAMPLE);
        assert_eq!(part2(&example), 12);
    }
}