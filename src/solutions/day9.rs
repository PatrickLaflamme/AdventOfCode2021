use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::vec::IntoIter;

const DIRECTIONS: [(isize, isize); 4] = [
    (1,0),
    (0,1),
    (-1,0),
    (0,-1)
];

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

fn find_lower_points(heights: &[Vec<isize>], x: &isize, point: &isize, y: &isize) ->  bool {
    let width_range = 0..heights[0].len();
    let height_range = 0..heights.len();
    DIRECTIONS.iter()
        .filter(|(dx, dy)| {
            width_range.contains(&((x + dx) as usize)) && height_range.contains(&((y + dy) as usize))
        })
        .all(|(dx, dy)| *point < heights[(y + dy) as usize][(x + dx) as usize])
}

fn map_to_lower_points(heights: &[Vec<isize>], y: isize, row: &[isize]) -> IntoIter<(isize, isize, isize)> {
    row.iter()
        .enumerate()
        .filter(|(x, &point)| find_lower_points(heights, &(*x as isize), &point, &(y as isize)))
        .map(|(x, &point)| (x as isize, y, point))
        .collect::<Vec<_>>()
        .into_iter()
}

fn get_basin_size(heights: &[Vec<isize>], low_point: &(isize, isize, isize), crest_height: &isize) -> HashSet<(usize, usize)> {
    let mut to_visit = VecDeque::from([*low_point]);
    let mut seen = HashSet::<(usize, usize)>::new();
    let width_range = 0..heights[0].len();
    let height_range = 0..heights.len();
    loop {
        if to_visit.len() < 1 {
            break;
        }
        let (x, y, point) = to_visit.pop_front().unwrap();
        if point < *crest_height {
            seen.insert((x as usize, y as usize));
            DIRECTIONS.iter()
                .map(|(dx, dy)| ((x + dx) as usize, (y + dy) as usize))
                .filter(|(neighbor_x, neighbor_y)| {
                    width_range.contains(&neighbor_x) && height_range.contains(&neighbor_y)
                })
                .filter(|neighbor| {
                    !seen.contains(&neighbor)
                })
                .for_each(|(nx, ny)| to_visit.push_back((nx as isize, ny as isize, heights[ny][nx])));
        }
    }
    return seen;
}

#[aoc(day9, part1)]
pub fn solve_part1(heights: &[Vec<isize>]) -> isize {
    heights.iter()
        .enumerate()
        .flat_map(|(y, row)| map_to_lower_points(heights, y.try_into().unwrap(), row))
        .fold(0, |sum, (_x, _y, point)| sum + point + 1)
}

#[aoc(day9, part2)]
pub fn solve_part2(heights: &[Vec<isize>]) -> usize {
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let crest_height = heights.iter()
        .flat_map(|row| row)
        .max()
        .unwrap();
    heights.iter()
        .enumerate()
        .flat_map(|(y, row)| map_to_lower_points(heights, y.try_into().unwrap(), row))
        .map(|low_point| get_basin_size(heights, &low_point, crest_height))
        .fold(Vec::<HashSet<(usize, usize)>>::new(), |mut sets, set| {
            let seen = sets.iter()
                .any(|seen_set| seen_set.intersection(&set).count() > 0);
            if !seen {
                sets.push(set);
            }
            sets
        })
        .iter()
        .map(|set| set.len())
        .for_each(|size| {
            if size >= first {
                third = second;
                second = first;
                first = size;
            } else if size >= second {
                third = second;
                second = size;
            } else if size >= third {
                third = size;
            }
        });
    return first * second * third;
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

    const EXAMPLE2: &str = "
        2199943210
        3987894921
        8856789892
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
    fn test_solve_part2a() {
        let example: Vec<Vec<isize>> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 1134);
    }

    #[test]
    fn test_solve_part2b() {
        let example: Vec<Vec<isize>> = generator(&EXAMPLE2);
        assert_eq!(solve_part2(&example), 1458);
    }
}