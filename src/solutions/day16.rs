use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

lazy_static! {
    static ref HEXMAP: HashMap<char, Vec<u8>> = vec![
        ('0', vec![0, 0, 0, 0]),
        ('1', vec![0, 0, 0, 1]),
        ('2', vec![0, 0, 1, 0]),
        ('3', vec![0, 0, 1, 1]),
        ('4', vec![0, 1, 0, 0]),
        ('5', vec![0, 1, 0, 1]),
        ('6', vec![0, 1, 1, 0]),
        ('7', vec![0, 1, 1, 1]),
        ('8', vec![1, 0, 0, 0]),
        ('9', vec![1, 0, 0, 1]),
        ('A', vec![1, 0, 1, 0]),
        ('B', vec![1, 0, 1, 1]),
        ('C', vec![1, 1, 0, 0]),
        ('D', vec![1, 1, 0, 1]),
        ('E', vec![1, 1, 1, 0]),
        ('F', vec![1, 1, 1, 1]),
    ].into_iter().collect();
}

#[aoc_generator(day16)]
pub fn generator(raw_input: &str) -> Vec<bool> {
    raw_input.chars()
        .flat_map(|b| {
            HEXMAP.get(&b).unwrap()
                .iter()
                .map(|&i| i == 1)
        })
        .collect::<Vec<bool>>()
}

fn parse_value(binary: &[bool]) -> usize {
    binary.iter()
        .rev()
        .map(|b| if *b { 1 } else { 0 })
        .enumerate()
        .fold(0, |val, (i, x)| val + x * usize::pow(2, i as u32))
}

fn parse_prefix(binary: &[bool], pointer: usize, len: usize) -> usize {
    parse_value(&binary[pointer..pointer + len])
}

fn parse_literal(binary: &[bool], mut pointer: usize) -> (usize, usize) {
    let mut value_bin: Vec<bool> = Vec::new();
    loop {
        let prefix = binary[pointer];
        binary[(pointer + 1)..(pointer + 5)].iter().for_each(|b| value_bin.push(*b));
        pointer += 5;
        if !prefix {
            break;
        }
        if pointer >= binary.len() {
            panic!("invalid packet!");
        }
    }
    (pointer, parse_value(&value_bin))
}

fn subpacket_operation(binary: &[bool], mut pointer: usize, operation: fn(Vec<usize>) -> usize) -> (usize, usize, usize) {
    let mut sub_packet_count = usize::MAX;
    let sub_binary: Vec<bool>;
    let subpacket_container_type = binary[pointer];
    pointer += 1;
    if subpacket_container_type {
        sub_packet_count = parse_prefix(binary, pointer, 11);
        pointer += 11;
        sub_binary = binary[pointer..].iter().map(|&b| b).collect();
    } else {
        let sub_packet_len = parse_prefix(binary, pointer, 15);
        pointer += 15;
        sub_binary = binary[pointer..pointer + sub_packet_len].to_vec();
    }
    let (new_pointer, v_sum, values)= parse_packet(&sub_binary, sub_packet_count);
    pointer += new_pointer;
    let value = operation(values);
    (pointer, v_sum, value)
}

fn parse_packet(binary: &[bool], n_packets: usize) -> (usize, usize, Vec<usize>) {
    let mut v_sum: usize = 0;
    let mut values: Vec<usize> = Vec::new();
    let mut pointer = 0;
    let mut seen_packets = 0;
    loop {
        if pointer + 6 >= binary.len() || seen_packets >= n_packets || binary[pointer..].iter().all(|b| !b) {
            break;
        }
        seen_packets += 1;
        let v = parse_prefix(binary, pointer, 3);
        pointer += 3;
        let t = parse_prefix(binary, pointer, 3);
        pointer += 3;
        v_sum += v as usize;
        let (new_pointer, sub_v_sum, value) = match t {
            0 => subpacket_operation(binary, pointer, |v| v.iter().sum()),
            1 => subpacket_operation(binary, pointer, |v| v.iter().product()),
            2 => subpacket_operation(binary, pointer, |v| *v.iter().min().unwrap()),
            3 => subpacket_operation(binary, pointer, |v| *v.iter().max().unwrap()),
            4 => {
                let (new_pointer, value) = parse_literal(binary, pointer);
                (new_pointer, 0, value)
            },
            5 => subpacket_operation(binary, pointer, |v| (v[0] > v[1]) as usize),
            6 => subpacket_operation(binary, pointer, |v| (v[0] < v[1]) as usize),
            7 => subpacket_operation(binary, pointer, |v| (v[0] == v[1]) as usize),
            other => panic!("invalid packet type {}!", other)
        };
        pointer = new_pointer;
        v_sum += sub_v_sum;
        values.push(value);
    }
    (pointer, v_sum.into(), values)
}

#[aoc(day16, part1)]
pub fn solve_part1(binary: &[bool]) -> usize {
    parse_packet(binary, usize::MAX).1
}

#[aoc(day16, part2)]
pub fn solve_part2(binary: &[bool]) -> usize {
    *parse_packet(binary, usize::MAX).2.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE0: &str = "D2FE28";
    const EXAMPLE1: &str = "8A004A801A8002F478";
    const EXAMPLE2: &str = "620080001611562C8802118E34";
    const EXAMPLE3: &str = "C0015000016115A2E0802F182340";
    const EXAMPLE4: &str = "A0016C880162017C3686B18A3D4780";
    const EXAMPLE5: &str = "C200B40A82";
    const EXAMPLE6: &str = "04005AC33890";
    const EXAMPLE7: &str = "880086C3E88112";
    const EXAMPLE8: &str = "CE00C43D881120";
    const EXAMPLE9: &str = "D8005AC2A8F0";
    const EXAMPLE10: &str = "F600BC2D8F";
    const EXAMPLE11: &str = "9C005AC2F8F0";
    const EXAMPLE12: &str = "9C0141080250320F1802104A08";

    #[test]
    fn test_generator() {
        let expected: Vec<bool> = vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0].iter()
            .map(|&b| b == 1)
            .collect();
        assert_eq!(generator(&EXAMPLE0), expected);
    }

    #[test]
    fn test_solve_part1_example0() {
        let example: Vec<bool> = generator(&EXAMPLE0);
        assert_eq!(solve_part1(&example), 6);
    }
    
    #[test]
    fn test_solve_part1_example1() {
        let example: Vec<bool> = generator(&EXAMPLE1);
        assert_eq!(solve_part1(&example), 16);
    }

    #[test]
    fn test_solve_part1_example2() {
        let example: Vec<bool> = generator(&EXAMPLE2);
        assert_eq!(solve_part1(&example), 12);
    }

    #[test]
    fn test_solve_part1_example3() {
        let example: Vec<bool> = generator(&EXAMPLE3);
        assert_eq!(solve_part1(&example), 23);
    }

    #[test]
    fn test_solve_part1_example4() {
        let example: Vec<bool> = generator(&EXAMPLE4);
        assert_eq!(solve_part1(&example), 31);
    }

    #[test]
    fn test_solve_part2_example5() {
        let example: Vec<bool> = generator(&EXAMPLE5);
        assert_eq!(solve_part2(&example), 3);
    }

    #[test]
    fn test_solve_part2_example6() {
        let example: Vec<bool> = generator(&EXAMPLE6);
        assert_eq!(solve_part2(&example), 54);
    }

    #[test]
    fn test_solve_part2_example7() {
        let example: Vec<bool> = generator(&EXAMPLE7);
        assert_eq!(solve_part2(&example), 7);
    }

    #[test]
    fn test_solve_part2_example8() {
        let example: Vec<bool> = generator(&EXAMPLE8);
        assert_eq!(solve_part2(&example), 9);
    }

    #[test]
    fn test_solve_part2_example9() {
        let example: Vec<bool> = generator(&EXAMPLE9);
          assert_eq!(solve_part2(&example), 1);
    }

    #[test]
    fn test_solve_part2_example10() {
        let example: Vec<bool> = generator(&EXAMPLE10);
        assert_eq!(solve_part2(&example), 0);
    }

    #[test]
    fn test_solve_part2_example11() {
        let example: Vec<bool> = generator(&EXAMPLE11);
        assert_eq!(solve_part2(&example), 0);
    }

    #[test]
    fn test_solve_part2_example12() {
        let example: Vec<bool> = generator(&EXAMPLE12);
        assert_eq!(solve_part2(&example), 1);
    }
}