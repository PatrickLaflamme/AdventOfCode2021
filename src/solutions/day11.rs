use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::collections::VecDeque;

const DIRECTIONS: [(isize, isize); 8] = [
    (0,1),
    (1,1),
    (1,0),
    (1,-1),
    (0,-1),
    (-1,-1),
    (-1,0),
    (-1,1)
];

#[aoc_generator(day11)]
pub fn generator(raw_input: &str) -> Vec<Vec<usize>> {
    raw_input.lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            s.trim()
                .split("")
                .filter(|s2| !s2.trim().is_empty())
                .map(|s2| s2.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn simulate_step(board: &[Vec<usize>]) -> (Vec<Vec<usize>>, usize) {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut flash: VecDeque<(usize, usize)> = VecDeque::new();
    // step 1
    let mut inc_board: Vec<Vec<usize>> = board.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, oct)| {
                    if oct >= &9 {
                        seen.insert((i,j));
                        flash.push_back((i,j));
                    }
                    oct + 1
                })
                .collect()
        })
        .collect();
    // step 2
    while flash.len() > 0 {
        let oct = flash.pop_front().unwrap();
        DIRECTIONS.iter()
            .map(|(y,x)| (y + oct.0 as isize, x + oct.1 as isize))
            .filter_map(|(y, x)| {
                if y < 0 || x < 0 || y >= board.len() as isize || x >= board[0].len() as isize {
                    return None;
                }
                Some((y as usize, x as usize))
            })
            .for_each(|(y, x)| {
                if inc_board[y][x] >= 9 && !seen.contains(&(y,x)) {
                    seen.insert((y,x));
                    flash.push_back((y,x));
                }
                inc_board[y][x] += 1
            });
    }
    // step 3
    seen.iter()
        .for_each(|(y,x)| inc_board[*y][*x] = 0);
    (inc_board, seen.len())
}

#[aoc(day11, part1)]
pub fn solve_part1(board: &[Vec<usize>]) -> usize {
    let mut octs: Vec<Vec<usize>> = board.iter()
        .map(|s| s.iter().map(|u| *u).collect())
        .collect();
    let mut total_flashes = 0;
    for _ in 0..100 {
        let (new_state, step_flashes) = simulate_step(&octs);
        total_flashes += step_flashes;
        octs = new_state;
    }
    total_flashes
}

#[aoc(day11, part2)]
pub fn solve_part2(board: &[Vec<usize>]) -> usize {
    let mut octs: Vec<Vec<usize>> = board.iter()
        .map(|s| s.iter().map(|u| *u).collect())
        .collect();
    let mut n = 0;
    loop {
        n += 1;
        let (new_state, step_flashes) = simulate_step(&octs);
        octs = new_state;
        if step_flashes == 100 {
            return n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    ";

    #[test]
    fn test_generator() {
        let expected = vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ];
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_simulate_1step() {
        let expected = vec![
            vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
            vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
            vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
            vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
            vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
            vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
            vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
            vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
            vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
            vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
        ];
        let input: Vec<Vec<usize>> = generator(&EXAMPLE);
        let (result, flashes) = simulate_step(&input);
        assert_eq!(0, flashes);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_simulate_2step() {
        let expected = vec![
            vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
            vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
            vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
            vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
            vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
            vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
            vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
            vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
            vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
            vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
        ];
        let input: Vec<Vec<usize>> = generator(&EXAMPLE);
        let (result1, _) = simulate_step(&input);
        let (result2, flashes2) = simulate_step(&result1);
        assert_eq!(35, flashes2);
        assert_eq!(expected, result2);
    }

    #[test]
    fn test_solve_part1() {
        let example: Vec<Vec<usize>> = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 1656);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<Vec<usize>> = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 195);
    }
}