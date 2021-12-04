use std::env;
use std::fs;

const BOARD_SIZE: usize = 5;

fn parse_input(raw_input: String) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let mut input_lines = raw_input.lines();
    let draws = input_lines.next().unwrap();
    let draws: Vec<i32> = draws
        .split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<i32>>> = Vec::new();
    while let Some(_) = input_lines.next() {
        let mut board: Vec<Vec<i32>> = Vec::new();
        for _ in 0..BOARD_SIZE {
            let row_str = input_lines.next().unwrap().trim().split_ascii_whitespace();
            let row: Vec<i32> = row_str.map(|v| v.parse().unwrap()).collect();
            board.push(row);
        }
        boards.push(board);
    }

    (draws, boards)
}

fn is_winner(board: &Vec<Vec<i32>>) -> bool {
    for row in board.iter() {
        let matches = row.iter().filter(|&&v| v == -1).count();
        if matches == BOARD_SIZE {
            return true;
        }
    }

    for col in 0..BOARD_SIZE {
        let mut matches: usize = 0;
        for row in 0..BOARD_SIZE {
            matches += (board[row][col] == -1) as usize;
        }
        if matches == BOARD_SIZE {
            return true;
        }
    }

    return false;
}

fn sum_board_nonmatches(board: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for r in 0..BOARD_SIZE {
        for c in 0..BOARD_SIZE {
            sum += if board[r][c] != -1 { board[r][c] } else { 0 }
        }
    }

    sum
}

fn calculate_winner_board_score(draws: &[i32], boards: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut boards = boards.clone();
    for draw in draws {
        for board in boards.iter_mut() {
            for r in 0..BOARD_SIZE {
                for c in 0..BOARD_SIZE {
                    if board[r][c] == *draw {
                        board[r][c] = -1;
                    }
                }
            }
        }

        for board in boards.iter() {
            if is_winner(&board) {
                return sum_board_nonmatches(&board) * (*draw);
            }
        }
    }

    return 0;
}

fn calculate_winner_board_score_part2(draws: &[i32], boards: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut boards = boards.clone();
    let mut board_wins: Vec<bool> = vec![false; boards.len()];
    let mut last_score = 0;
    for draw in draws {
        for (idx, board) in boards.iter_mut().enumerate() {
            if board_wins[idx] {
                continue;
            }
            for r in 0..BOARD_SIZE {
                for c in 0..BOARD_SIZE {
                    if board[r][c] == *draw {
                        board[r][c] = -1;
                    }
                }
            }
        }

        for (idx, board) in boards.iter().enumerate() {
            if board_wins[idx] {
                continue;
            }
            if is_winner(&board) {
                board_wins[idx] = true;
                last_score = sum_board_nonmatches(&board) * (*draw);
            }
        }
    }

    return last_score;
}

fn main() {
    let mut file_name: String = "input.txt".to_string();

    let mut args = env::args();
    if args.len() >= 2 {
        file_name = args.nth(1).unwrap();
    }

    let (draws, boards) = parse_input(
        fs::read_to_string(&file_name)
            .expect(format!("Could not read input file {}", &file_name).as_ref()),
    );
    let winner_score = calculate_winner_board_score(&draws, &boards);
    println!("Day 4: Winner score = {}", winner_score);

    let last_winner_score = calculate_winner_board_score_part2(&draws, &boards);
    println!("Day 4: Last Winner score = {}", last_winner_score);
}
