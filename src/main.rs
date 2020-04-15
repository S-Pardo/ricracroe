use std::io;

struct Game {
    board: [[i8; 3]; 3],
    is_over: bool,
}

impl Game {
    fn new() -> Game {
        let board = [[0i8; 3], [0i8; 3], [0i8; 3]];
        Game {
            board: board,
            is_over: false,
        }
    }

    fn print_board(&self) {
        for i in 0..3 {
            println!("+---+---+---+");
            for j in 0..3 {
                print!(
                    "| {} ",
                    if self.board[i][j] == 0 {
                        " "
                    } else if self.board[i][j] == 1 {
                        "X"
                    } else {
                        "O"
                    }
                )
            }
            print!("|");
            println!("");
        }
        println!("+---+---+---+");
    }

    fn is_ocuppied(&self, row: usize, col: usize) -> bool {
        if self.board.len() < row || self.board[row].len() < col {
            return true;
        }
        if self.board[row][col] != 0 {
            true
        } else {
            false
        }
    }

    fn play(&mut self, row: usize, col: usize) {
        self.board[row][col] = 1;
    }
}

fn main() {
    let mut game = Game::new();
    while !game.is_over {
        game.print_board();
        let [row, col] = get_play(&game);
        game.play(row as usize, col as usize);
    }
}

fn get_play(game: &Game) -> [u8; 2] {
    loop {
        let [row, col] = get_validated_input();
        if !game.is_ocuppied(row as usize, col as usize) {
            return [row, col];
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
        // Parses input from &str to u8
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
