use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryInto;
use std::vec::IntoIter;

#[aoc_generator(day9)]
pub fn generator(raw_input: &str) -> Vec<Vec<isize>> {
    raw_input.lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            s.trim().split("")
                .filter(|s2| !s2.is_empty())
                .map(|s2| s2.parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_lower_points(heights: &[Vec<isize>], x: &isize, point: &isize, y: &isize, width: usize, height: usize) ->  bool {
    let directions: Vec<(isize, isize)> = vec![
        (1,0),
        (0,1),
        (-1,0),
        (0,-1)
    ];
    directions.iter()
        .filter(|(dx, dy)| {
            (0..width).contains(&((x + dx) as usize)) && (0..height).contains(&((y + dy) as usize))
        })
        .all(|(dx, dy)| *point < heights[(y + dy) as usize][(x + dx) as usize])
}

fn map_to_lower_points(heights: &[Vec<isize>], y: isize, row: &[isize]) -> IntoIter<(isize, isize, isize)> {
    row.iter()
        .enumerate()
        .filter(|(x, &point)| find_lower_points(heights, &(*x as isize), &point, &(y as isize), row.len(), heights.len()))
        .map(|(x, &point)| (x as isize, y, point + &1))
        .collect::<Vec<_>>()
        .into_iter()
}

#[aoc(day9, part1)]
pub fn solve_part1(heights: &[Vec<isize>]) -> isize {
    heights.iter()
        .enumerate()
        .flat_map(|(y, row)| map_to_lower_points(heights, y.try_into().unwrap(), row))
        .fold(0, |sum, (_x, _y, point)| sum + point)
}

#[aoc(day9, part2)]
pub fn solve_part2(readings: &[Vec<isize>]) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    ";

    #[test]
    fn test_generator() {
        let expected = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<Vec<isize>> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 15);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<Vec<isize>> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 0);
    }
}