use aoc_runner_derive::{aoc, aoc_generator};

fn parse_line(line: &str) -> Vec<(usize, usize)> {
    let mut depth: usize = 0;
    let mut parsed_line: Vec<(usize, usize)> = Vec::new();
    let mut i: usize = 0;
    while i < line.len() {
        let c: char = line.chars().nth(i).unwrap();
        match c {
            '[' => {
                depth += 1;
            }
            ']' => {
                depth -= 1;
            }
            ',' => {}
            _ => {
                let mut string = c.to_string();
                while i < line.len() - 1 && line.chars().nth(i+1).unwrap().is_numeric() {
                    i += 1;
                    string = string + &line.chars().nth(i).unwrap().to_string();
                }
                let val: usize = string
                    .parse()
                    .expect(&("Expected the value to be an integer! ".to_string() + line + " " + &c.to_string()));
                parsed_line.push((val, depth));
            }
        }
        i += 1;
    }
    parsed_line
}

#[aoc_generator(day18)]
pub fn parse(raw_input: &str) -> Vec<Vec<(usize, usize)>> {
    raw_input.split("\n")
        .map(|line| { line.trim() })
        .filter(|line| !line.is_empty())
        .map(|line| { parse_line(line) })
        .collect()
}

fn explode(reading: &[(usize, usize)]) -> Option<Vec<(usize, usize)>> {
    let mut i: usize = 0;
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let mut change = false;
    while i < reading.len() - 1 {
        let (val, depth) = reading.get(i).unwrap();
        if depth > &4 && &reading.get(i + 1).unwrap().1 == depth {
            change = true;
            if ret.len() > 0 {
                let (prev, prev_depth) = ret.pop().unwrap();
                ret.push((prev + val, prev_depth));
            }
            ret.push((0, depth - 1));
            i += 1;
            if i >= reading.len() {
                break;
            }
            let (next_val, _) = reading.get(i).unwrap();
            i += 1;
            if i >= reading.len() {
                break;
            }
            let (next, next_depth) = reading.get(i).unwrap();
            ret.push((next + next_val, *next_depth));
            i += 1;
            break;
        } else {
            ret.push((*val, *depth));
        }
        i += 1;
    }

    if change {
        if i < reading.len() {
            ret.append(&mut reading[i..].to_vec());
        }
        Some(ret)
    } else {
        None
    }
}

fn split(reading: &[(usize, usize)]) -> Option<Vec<(usize, usize)>> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    let mut change = false;
    while i < reading.len() {
        let (val, depth) = reading.get(i).unwrap();
        if val > &9 {
            change = true;
            ret.push((val / 2, depth + 1));
            ret.push((val / 2 + val % 2, depth + 1));
            i += 1;
            break;
        }
        ret.push((*val, *depth));
        i += 1;
    }

    if change {
        if i < reading.len() {
            ret.append(&mut reading[i..].to_vec());
        }
        Some(ret)
    } else {
        None
    }
}

fn reduce(reading: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut reduction: Vec<(usize, usize)> = reading.to_vec();
    loop {
        let mut result = explode(&reduction);
        if result.is_some() {
            reduction = result.unwrap();
            continue;
        }
        result = split(&reduction);
        if result.is_some() {
            reduction = result.unwrap();
            continue;
        }
        break;
    }
    reduction
}

fn calculate_magnitude(reading: &[(usize, usize)]) -> usize {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push(*reading.get(0).unwrap());
    for (val1, depth1) in reading.get(1..).unwrap() {
        let (_, depth2) = stack.last().unwrap();
        if depth1 == depth2 {
            let (val2, _) = stack.pop().unwrap();
            stack.push((2 * val1 + 3 * val2, depth1 - 1));
            while stack.len() > 1 && stack.last().unwrap().1 == stack.get(stack.len() - 2).unwrap().1 {
                let (val1, depth1) = stack.pop().unwrap();
                let (val2, _) = stack.pop().unwrap();
                stack.push((2*val1 + 3 * val2, depth1 - 1));
            }
        } else {
            stack.push((*val1, *depth1));
        }
    }
    while stack.len() > 1 {
        let (val1, depth1) = stack.pop().unwrap();
        let (val2, depth2) = stack.pop().unwrap();
        if depth1 != depth2 {
            println!("stack = {:?}, val1 = {}, val2 = {}, depth1 = {}, depth2 = {}", stack, val1, val2, depth1, depth2);
            panic!("Invalid depths!");
        }
        stack.push((2*val1 + 3 * val2, depth1 - 1));
    }
    let (ret, _) = stack.first().unwrap();
    *ret as usize
}

fn perform_addition(readings: &[Vec<(usize, usize)>]) -> Vec<(usize, usize)> {
    let mut ans: Vec<(usize, usize)> = readings.get(0)
        .unwrap()
        .into_iter()
        .map(|(val, depth)| (*val, *depth))
        .collect();
    for reading in readings[1..].into_iter() {
        let mut incremented_reading = (*reading).iter().map(|(val, depth)| { (*val, depth + 1) }).collect();
        ans = ans.iter().map(|(val, depth)| { (*val, depth + 1) }).collect();
        ans.append(&mut incremented_reading);
        ans = reduce(&ans);
    }
    ans
}

#[aoc(day18, part1)]
pub fn solve_part1(readings: &[Vec<(usize, usize)>]) -> usize {
    let ans = perform_addition(readings);
    calculate_magnitude(&ans)
}

#[aoc(day18, part2)]
pub fn solve_part2(readings: &[Vec<(usize, usize)>]) -> usize {
    readings.iter()
        .enumerate()
        .map(|(i, left)| {
            readings.iter()
                .enumerate()
                .map(|(j, right)| {
                    if i == j {
                        return 0;
                    }
                    let left_copy = left.iter().map(|(val, depth)| (*val, *depth)).collect();
                    let right_copy = right.iter().map(|(val, depth)| (*val, *depth)).collect();
                    let ans = perform_addition(&[left_copy, right_copy]);
                    calculate_magnitude(&ans)                    
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
    [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    ";

    #[test]
    fn test_magnitude() {
        let mut example = parse_line("[9,1]");
        assert_eq!(calculate_magnitude(&example), 29);
        
        example = parse_line("[1,9]");
        assert_eq!(calculate_magnitude(&example), 21);
        
        example = parse_line("[[9,1],[1,9]]");
        assert_eq!(calculate_magnitude(&example), 129);
        
        example = parse_line("[[1,2],[[3,4],5]]");
        assert_eq!(calculate_magnitude(&example), 143);
        
        example = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(calculate_magnitude(&example), 1384);
        
        example = parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(calculate_magnitude(&example), 445);
        
        example = parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(calculate_magnitude(&example), 791);
        
        example = parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(calculate_magnitude(&example), 1137);
        
        example = parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(calculate_magnitude(&example), 3488);
    }
    
    #[test]
    fn test_explode() {
        let mut example = parse_line("[[[[[9,8],1],2],3],4]");
        let mut expected = parse_line("[[[[0,9],2],3],4]");
        assert_eq!(explode(&example), Some(expected));
        
        example = parse_line("[7,[6,[5,[4,[3,2]]]]]");
        expected = parse_line("[7,[6,[5,[7,0]]]]");
        assert_eq!(explode(&example), Some(expected));
        
        example = parse_line("[[6,[5,[4,[3,2]]]],1]");
        expected = parse_line("[[6,[5,[7,0]]],3]");
        assert_eq!(explode(&example), Some(expected));
        
        example = parse_line("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        expected = parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(explode(&example), Some(expected));
        
        example = parse_line("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        expected = parse_line("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        assert_eq!(explode(&example), Some(expected));
    }
    
    #[test]
    fn test_split() {
        let mut example = parse_line("[[[[0,7],4],[15,[0,13]]],[1,1]]");
        let mut expected = parse_line("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        assert_eq!(split(&example), Some(expected));
        
        example = parse_line("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
        expected = parse_line("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
        assert_eq!(split(&example), Some(expected));
    }
    
    #[test]
    fn test_part1_simple_examples() {
        let mut example = parse("
            [[[[4,3],4],4],[7,[[8,4],9]]]
            [1,1]
        ");
        let mut expected = parse_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(perform_addition(&example), expected);
        
        example = parse("
            [1,1]
            [2,2]
            [3,3]
            [4,4]
        ");
        expected = parse_line("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(perform_addition(&example), expected);
        
        example = parse("
            [1,1]
            [2,2]
            [3,3]
            [4,4]
            [5,5]
        ");
        expected = parse_line("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(perform_addition(&example), expected);
        
        example = parse("
            [1,1]
            [2,2]
            [3,3]
            [4,4]
            [5,5]
            [6,6]
        ");
        expected = parse_line("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(perform_addition(&example), expected);

        example = parse("
            [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        ");
        expected = parse_line("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        assert_eq!(perform_addition(&example), expected);

        example = parse("
            [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        ");
        expected = parse_line("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");
        assert_eq!(perform_addition(&example), expected);
                        
        example = parse("
            [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]
        ");
        expected = parse_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(perform_addition(&example), expected);
        
        example = parse("
            [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        ");
        expected = parse_line("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        assert_eq!(perform_addition(&example), expected);
    }
    
    #[test]
    fn test_solve_part1() {
        let example: Vec<Vec<(usize, usize)>> = parse(&EXAMPLE);
        assert_eq!(solve_part1(&example), 4140);
    }

    #[test]
    fn test_solve_part2() {
        let example: Vec<Vec<(usize, usize)>> = parse(&EXAMPLE);
        assert_eq!(solve_part2(&example), 3993);
    }
}
