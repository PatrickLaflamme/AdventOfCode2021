use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::convert::TryInto;

const ONE: char = '1';
const ZERO: char = '0';

pub fn split(raw_input: &str) -> Vec<String> {
    raw_input
        .lines()
        .map(|split| {
            split.trim().to_string()
        })
        .filter(|line| {
            !line.is_empty()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(raw_input: &str) -> u32 {
    let split_input = split(raw_input);
    let len: usize = split_input.len();
    let bit_size: usize = split_input.first().unwrap().len();

    let empty_str: String = String::with_capacity(bit_size);
    let gamma_str = split_input
        .iter()
        .fold(vec![0; bit_size], |mut counts, bin| {
            let mut i = 0;
            while i < bit_size {
                if bin.chars().nth(i).unwrap() == ONE {
                    counts[i] += 1
                }
                i += 1
            }
            counts
        })
        .iter()
        .fold(empty_str, |bit_str, count| {
            let mut owned_bit_str = bit_str.to_owned();
            if count > &(len / 2) {
                owned_bit_str.push_str(&ONE.to_string())
            } else {
                owned_bit_str.push_str(&ZERO.to_string())
            }
            owned_bit_str
        });
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let magic_int = u32::pow(2, bit_size .try_into().unwrap()) - 1;
    let epsilon = !gamma & magic_int;
    return gamma * epsilon;
}

fn choose_bit<F>(vals: &[String], index: usize, handler: F) -> Vec<String> where F: for <'a> Fn(&'a [String], &'a [String]) -> &'a [String] {
    let grouped: HashMap<char, Vec<String>> = vals.iter()
        .fold(HashMap::new(), |mut map, elem| {
            let key = elem.chars().nth(index).unwrap();
            map.entry(key).or_insert(Vec::new()).push(elem.to_string());
            map
        });
    return handler(grouped.get(&ONE).unwrap(), grouped.get(&ZERO).unwrap()).to_vec();
}

fn choose_elem<F>(mut vals: Vec<String>, handler: F) -> String where F: for <'a> Fn(&'a [String], &'a [String]) -> &'a [String] + Copy {
    let mut index = 0;
    while vals.len() > 1 {
        vals = choose_bit(&vals, index, handler);
        index += 1;
    }
    return vals.first().expect("Something went wrong!").to_string();
}

#[aoc(day3, part2)]
pub fn part2(raw_input: &str) -> u32 {
    let split_input = split(raw_input);
    let o2_str = choose_elem(split_input.iter().cloned().collect(), |ones, zeros| {
        if ones.len() >= zeros.len() {
            return ones;
        } else {
            return zeros;
        }
    });
    let co2_str = choose_elem(split_input, |ones, zeros| {
        if ones.len() < zeros.len() {
            return ones;
        } else {
            return zeros;
        }
    });
    let o2 = u32::from_str_radix(&o2_str, 2).unwrap();
    let co2 = u32::from_str_radix(&co2_str, 2).unwrap();
    return o2 * co2;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    ";

    #[test]
    fn test_split() {
        let expected = [
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ];
        assert_eq!(split(&EXAMPLE), expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE), 198);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE), 230);
    }
}