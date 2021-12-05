use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub rows_seen: Vec<usize>,
    pub cols_seen: Vec<usize>,
    pub entries: HashMap<usize, (usize, usize)>
}

fn parse_board(board_input: &str) -> Board {
    let row_str: Vec<&str> = board_input.split("\n").collect();
    let mut entries: HashMap<usize, (usize, usize)> = HashMap::new();
    let row_len: usize = row_str.len();
    let mut col_len: usize = 0;
    for (i, row) in row_str.iter().enumerate() {
        let cells: Vec<usize> = row.trim()
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.trim().parse().unwrap()
            })
            .collect();
        col_len = cells.len();
        for (j, cell) in cells.iter().enumerate() {
            entries.insert(*cell, (i,j));
        }
    }
    Board {
        rows: row_len,
        cols: col_len,
        rows_seen: vec![0; row_len],
        cols_seen: vec![0; col_len],
        entries: entries
    }
}

#[aoc_generator(day4)]
pub fn parse_input(raw_input: &str) -> (Vec<usize>, Vec<Board>) {
    let components: Vec<&str> = raw_input.split("\n\n").collect();
    let draws: Vec<usize> = components[0].split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let boards = components[1..]
        .iter()
        .map(|s| parse_board(s))
        .collect();
    (draws, boards)
}

fn update_board(board: &mut Board, row: usize, col: usize) -> bool {
    board.rows_seen[row] += 1;
    board.cols_seen[col] += 1;
    if board.rows_seen[row] >= board.rows || board.cols_seen[col] >= board.cols {
        return true;
    }
    return false;
}

fn calculate_answer(board: &Board, draw: &usize) -> usize {
    let sum = board.entries
        .keys()
        .fold(0, |sum, &x| sum + x);
    sum * draw
}

fn play_bingo(draws: &[usize], boards: &mut [Board], play_until: usize) -> usize {
    let mut complete_boards: Vec<usize> = Vec::new();
    for draw in draws.iter() {
        for (i, mut board) in boards.iter_mut().enumerate() {
            if complete_boards.contains(&i) {
                continue
            }
            match board.entries.remove(draw) {
                Some((row, col)) => {
                    if update_board(&mut board, row, col) {
                        complete_boards.push(i);
                    }
                },
                None => continue
            }
        }
        if complete_boards.len() >= play_until {
            return calculate_answer(&boards[*complete_boards.last().unwrap()], draw);
        }
    }
    panic!("we should never get here!");
}


 
#[aoc(day4, part1)]
pub fn bingo_winner(input_data: &(Vec<usize>, Vec<Board>)) -> usize {
    let (draws, mut boards) = input_data.clone();
    return play_bingo(&draws, &mut boards, 1);
}

#[aoc(day4, part2)]
pub fn bingo_looser(input_data: &(Vec<usize>, Vec<Board>)) -> usize {
    let (draws, mut boards) = input_data.clone();
    return play_bingo(&draws, &mut boards, input_data.1.len());
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

    #[test]
    fn test_parse_input() {
        let actual = parse_input(&EXAMPLE);

        // draws parsed correctly
        assert_eq!(
            vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
            actual.0
        );

        //boards parsed correctly
        assert_eq!(3, actual.1.len());
        assert_eq!(5, actual.1[0].rows);
        assert_eq!(5, actual.1[0].cols);
        assert_eq!(vec![0; 5], actual.1[0].rows_seen);
        assert_eq!(vec![0; 5], actual.1[0].cols_seen);
        assert_eq!(&(2,1), actual.1[0].entries.get(&9).unwrap());
        assert_eq!(5, actual.1[1].rows);
        assert_eq!(5, actual.1[1].cols);
        assert_eq!(vec![0; 5], actual.1[1].rows_seen);
        assert_eq!(vec![0; 5], actual.1[1].cols_seen);
        assert_eq!(&(3,3), actual.1[1].entries.get(&24).unwrap());
        assert_eq!(5, actual.1[2].rows);
        assert_eq!(5, actual.1[2].cols);
        assert_eq!(vec![0; 5], actual.1[2].rows_seen);
        assert_eq!(vec![0; 5], actual.1[2].cols_seen);
        assert_eq!(&(2,2), actual.1[2].entries.get(&23).unwrap());
    }

    #[test]
    fn test_bingo_winner() {
        let input = parse_input(&EXAMPLE);
        assert_eq!(bingo_winner(&input), 4512);
    }

    #[test]
    fn test_bingo_looser() {
        let input = parse_input(&EXAMPLE);
        assert_eq!(bingo_looser(&input), 1924);
    }
}