use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day17)]
pub fn generator(raw_input: &str) -> ((isize, isize), (isize, isize)) {
    let re = Regex::new(r"[^\-0-9]+").unwrap();
    let nums: Vec<isize> = re.split(raw_input)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    ((nums[0], nums[1]), (nums[2], nums[3]))
}

//#[aoc(day17, part1)]
pub fn solve_part1(target: &((isize, isize), (isize, isize))) -> usize {
    let miny = target.1.0;
    let maxy = target.1.1;
    let vymin_fn = |t| (miny as f64 + 0.5 * f64::powf(t,2.0))/t;    
    let vymax_fn = |t| (maxy as f64 + 0.5 * f64::powf(t,2.0))/t;
    let mut vy: f64 = 0.0;
    for t in 0..1000 {
        if vymin_fn(t as f64).floor() < vymax_fn(t as f64).floor() {
            vy = vymax_fn(t as f64).floor();
        }
    }
    let s = (vy * (vy + 1.0)) / 2.0;
    s.ceil() as usize
}

#[aoc(day17, part2)]
pub fn solve_part2(target: &((isize, isize), (isize, isize))) -> usize {
    let minx = target.0.0;
    let maxx = target.0.1;
    let miny = target.1.0;
    let maxy = target.1.1;
    let v_fn = |t, d| (d as f64 + 0.5 * f64::powf(t,2.0))/t;    
    let mut count: usize = 0;
    for t in 0..1000000 {
        let t_64 = t as f64;
        if v_fn(t_64, minx).floor() < v_fn(t_64, maxx).floor() && v_fn(t_64, miny).floor() < v_fn(t_64, maxy).floor() {
            println!("{},{}", v_fn(t_64, maxx).floor(), v_fn(t_64, maxy).floor());
            count += 1
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_generator() {
        let expected: ((isize, isize), (isize, isize)) = ((20, 30), (-10, -5));
        assert_eq!(generator(&EXAMPLE), expected);
    }

    #[test]
    fn test_solve_part1() {
        let example: ((isize, isize), (isize, isize)) = generator(&EXAMPLE);
        assert_eq!(solve_part1(&example), 45);
    }

    //#[test]
    fn test_solve_part2() {
        let example: ((isize, isize), (isize, isize)) = generator(&EXAMPLE);
        assert_eq!(solve_part2(&example), 0);
    }
}