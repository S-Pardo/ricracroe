use colored::*;
use rand::Rng;
use std::io;

struct Game {
    board: [[i8; 3]; 3],
    is_over: bool,
    winner: i8,
}

impl Game {
    fn new() -> Game {
        let board = [[0i8; 3], [0i8; 3], [0i8; 3]];
        Game {
            board: board,
            is_over: false,
            winner: 0,
        }
    }

    fn print_board(&self) {
        for i in 0..3 {
            println!("+---+---+---+");
            for j in 0..3 {
                print!(
                    "| {} ",
                    if self.board[i][j] == 0 {
                        " ".white()
                    } else if self.board[i][j] == 1 {
                        "X".blue()
                    } else {
                        "O".red()
                    }
                )
            }
            print!("|");
            println!("");
        }
        println!("+---+---+---+");
    }

    fn is_ocuppied(&self, row: usize, col: usize) -> Result<bool, String> {
        if self.board.len() < row || self.board[row].len() < col {
            return Err(String::from("The indexes are out of bounds"));
        }
        if self.board[row][col] != 0 {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn play(&mut self, row: usize, col: usize) {
        self.board[row][col] = 1;
        if self.check_winning_board() == 1 {
            self.is_over = true;
            self.winner = 1;
            return;
        }
        loop {
            let [rand_row, rand_col] = [
                rand::thread_rng().gen_range(0, 3),
                rand::thread_rng().gen_range(0, 3),
            ];
            match self.is_ocuppied(rand_row, rand_col) {
                Ok(value) => {
                    if value {
                        continue;
                    }
                }
                Err(_) => continue,
            }
            self.board[rand_row][rand_col] = -1;
            if self.check_winning_board() == -1 {
                self.is_over = true;
                self.winner = 1;
            }
            return;
        }
    }

    fn check_winning_board(&mut self) -> i8 {
        let mut sum_row = 0;
        let mut sum_col = 0;
        let sum_diag = self.board[0][0] + self.board[1][1] + self.board[2][2];
        for i in 0..3 {
            for j in 0..3 {
                if sum_col == 3 || sum_row == 3 || sum_col == -3 || sum_row == -3 {
                    break;
                }
                sum_row += self.board[i][j];
                sum_col += self.board[j][i];
            }
        }
        if sum_row == 3 || sum_col == 3 || sum_diag == 3 {
            1
        } else if sum_row == -3 || sum_col == -3 || sum_diag == -3 {
            -1
        } else {
            0
        }
    }
}

fn main() {
    let mut game = Game::new();
    while !game.is_over {
        game.print_board();
        let [row, col] = get_play(&game);
        game.play(row as usize, col as usize);
    }
    game.print_board();
    println!(
        "the winner is: {}",
        if game.winner == 1 { "X" } else { "O" }
    );
}

fn get_play(game: &Game) -> [u8; 2] {
    loop {
        let [row, col] = get_validated_input();
        match game.is_ocuppied(row as usize, col as usize) {
            Ok(value) => {
                if !value {
                    return [row, col];
                }
            }
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
        println!("the box specified has already been played");
    }
}

fn get_validated_input() -> [u8; 2] {
    loop {
        println!("input your choice");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("error reading input, please try again.");
                continue;
            }
        }
        let mut splitted_input: Vec<&str> = input.trim().split(",").collect();
        if splitted_input.len() != 2 {
            println!("ERROR: enter only 2 numbers");
            continue;
        }
        let mut parsed_input: [u8; 2] = [0, 0];
        let mut is_parse_completed = false;
        // Parses input from &str to uf8
        for i in 0..2 {
            match splitted_input.get_mut(i).unwrap().parse::<u8>() {
                Ok(num) => {
                    is_parse_completed = true;
                    parsed_input[i] = num;
                }
                Err(_) => {
                    println!("error parsing your input");
                    is_parse_completed = false;
                    break;
                }
            }
        }
        if is_parse_completed {
            return parsed_input;
        }
    }
}
