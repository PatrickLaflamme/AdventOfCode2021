use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::max;
use std::collections::HashSet;

#[aoc_generator(day13)]
pub fn generator(raw_input: &str) -> (Vec<(isize, isize)>, HashSet<(isize,isize)>) {
    let mut split = raw_input.splitn(2, "\n\n");
    let (dots_input, folds_input) = (split.next().unwrap(), split.next().unwrap());
    let folds = folds_input.lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().splitn(2, "=").collect::<Vec<_>>())
        .map(|s| match s[0] {
            "fold along x" => (s[1].parse().unwrap(), 0),
            "fold along y" => (0, s[1].parse().unwrap()),
            other          => panic!("unexpected value {}", other)
        })
        .collect::<Vec<(isize, isize)>>();
    let dots = dots_input.lines()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().splitn(2, ",").collect::<Vec<_>>())
        .map(|s| (s[0].parse().unwrap(), s[1].parse().unwrap()))
        .collect::<HashSet<(isize, isize)>>();
    (folds, dots)
}

fn fold_point(fold: &(isize, isize), point: &(isize, isize)) -> (isize, isize) {
    let (mut x, mut y) = point;
    let (fx, fy) = fold;
    if fx > &0 && &x > fx {
        x = 2*fx - x;
    }
    if fy > &0 && &y > fy {
        y = 2*fy - y;
    }
    (x, y)
}

fn fold_paper(fold: &(isize, isize), dots: &HashSet<(isize,isize)>) -> HashSet<(isize,isize)> {
    dots.iter()
        .map(|dot| fold_point(fold, dot))
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(inputs: &(Vec<(isize, isize)>, HashSet<(isize,isize)>)) -> isize {
    let (folds, dots) = inputs;
    fold_paper(&folds[0], &dots).len() as isize
}

#[aoc(day13, part2)]
pub fn solve_part2(inputs: &(Vec<(isize, isize)>, HashSet<(isize,isize)>)) -> String {
    let (folds, dots) = inputs;
    let letters_pic = folds.iter()
        .fold(dots.iter().clone().map(|s| *s).collect::<HashSet<(isize, isize)>>(), |paper, f| fold_paper(f, &paper))
        .iter()
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect::<Vec<(usize, usize)>>();
    let (mx, my) = letters_pic.iter()
        .fold((0,0), |(mx, my), (x,y)| (max(mx, *x + 1), max(my, *y + 1)));
    let mut img = vec![vec![" "; mx]; my];
    letters_pic.iter()
        .for_each(|(x,y)| img[*y][*x] = "#");
    let img_str = img.iter()
        .map(|row| row.join(""))
        .collect::<Vec<String>>()
        .join("\n");
    ["\n".to_string(),img_str].join("").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
        ";
    
    const EXAMPLE2: &str = "
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along x=5
        ";

    #[test]
    fn test_generator() {
        let expected = (
            vec![
                (0,7),
                (5,0)
            ],
            [
                (6,10),
                (0,14),
                (9,10),
                (0,3),
                (10,4),
                (4,11),
                (6,0),
                (6,12),
                (4,1),
                (0,13),
                (10,12),
                (3,4),
                (3,0),
                (8,4),
                (1,10),
                (2,14),
                (8,10),
                (9,0)
            ].iter().map(|&e| e).collect::<HashSet<(isize, isize)>>()
        );
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1_1() {
        let example: (Vec<(isize, isize)>, HashSet<(isize,isize)>) = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 17);
    }

    #[test]
    fn test_solve_part1_2() {
        let example: (Vec<(isize, isize)>, HashSet<(isize,isize)>) = generator(&EXAMPLE2);
        assert_eq!(solve_part1(&example), 17);
    }

    #[test]
    fn test_solve_part2() {
        let example: (Vec<(isize, isize)>, HashSet<(isize,isize)>) = generator(&EXAMPLE);
        let expected = "
        #####
        #   #
        #   #
        #   #
        #####
        ".lines().map(|s| s.trim()).filter(|s| !s.is_empty()).collect::<Vec<&str>>().join("\n");
        assert_eq!(solve_part2(&example), ["".to_string(), expected].join(""));
    }
}